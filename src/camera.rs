//! User-initiated native capture and local H.264 in an isolated, private camera helper.
//! The helper is this executable, with no archive access. Its process can be killed even when
//! a native camera driver blocks, so locking or ending a call releases capture immediately.

use anyhow::{Result, anyhow};
use nokhwa::utils::{
    ApiBackend, CameraFormat, CameraIndex, CameraInfo, FrameFormat, RequestedFormat,
    RequestedFormatType, Resolution,
};
use nokhwa::{Camera, pixel_format::RgbFormat};
use openh264::encoder::{BitRate, Encoder, EncoderConfig, FrameRate, Profile};
use openh264::formats::{RgbSliceU8, YUVBuffer};
use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use whatsapp_rust::voip::{CallHandle, KeyframeUrgency, VideoFrame};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const FPS: u32 = 15;
const MAX_AU: usize = 2 * 1024 * 1024;
const MAX_DEVICE_LIST: usize = 64 * 1024;
const MAX_SELECTION: usize = 4096;

/// The platform identity stays local to the account's settings and the helper pipe.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CameraDevice {
    pub id: String,
    pub label: String,
}

fn camera_id(info: &CameraInfo) -> String {
    // Windows symbolic links and macOS uniqueIDs survive enumeration order changes.
    let native_id = info.misc();
    if !native_id.is_empty() {
        format!("native:{native_id}")
    } else {
        // V4L exposes a node index rather than a stable hardware ID. Include its name so a
        // differently named replacement at the same node is never selected silently.
        serde_json::to_string(&(
            info.index().as_string(),
            info.human_name(),
            info.description(),
        ))
        .expect("camera identity consists only of strings")
    }
}

fn camera_devices(infos: &[CameraInfo]) -> Vec<CameraDevice> {
    infos
        .iter()
        .map(|info| CameraDevice {
            id: camera_id(info),
            label: info.human_name(),
        })
        .collect()
}

fn selected_index(infos: &[CameraInfo], selected: Option<&str>) -> Result<CameraIndex> {
    match selected {
        None => Ok(CameraIndex::Index(0)),
        Some(id) => infos
            .iter()
            .find(|info| camera_id(info) == id)
            .map(|info| {
                let native_id = info.misc();
                if native_id.is_empty() {
                    info.index().clone()
                } else {
                    CameraIndex::String(native_id)
                }
            })
            .ok_or_else(|| anyhow!("The selected camera is unavailable. Choose another camera.")),
    }
}

fn helper_command(argument: &str) -> Result<Command> {
    let mut command = Command::new(std::env::current_exe()?);
    command
        .arg(argument)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000);
    }
    Ok(command)
}

struct HelperChild(Option<Child>);
impl HelperChild {
    fn process(&mut self) -> &mut Child {
        self.0
            .as_mut()
            .expect("helper child exists until handed to session")
    }
    fn into_process(mut self) -> Child {
        self.0
            .take()
            .expect("helper child exists until handed to session")
    }
}
impl Drop for HelperChild {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn write_bounded(output: &mut impl Write, bytes: &[u8], limit: usize) -> std::io::Result<()> {
    if bytes.len() > limit {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "camera metadata too large",
        ));
    }
    output.write_all(&(bytes.len() as u32).to_le_bytes())?;
    output.write_all(bytes)?;
    output.flush()
}

fn read_bounded(input: &mut impl Read, limit: usize) -> std::io::Result<Vec<u8>> {
    let mut size = [0; 4];
    input.read_exact(&mut size)?;
    let size = u32::from_le_bytes(size) as usize;
    if size > limit {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "camera metadata too large",
        ));
    }
    let mut bytes = vec![0; size];
    input.read_exact(&mut bytes)?;
    Ok(bytes)
}

/// Enumeration never opens a camera stream. A stuck native query is killed after ten seconds;
/// cancellation also kills the child instead of leaving a blocking driver on the worker.
pub async fn devices() -> Result<Vec<CameraDevice>> {
    let mut child = HelperChild(Some(helper_command("--camera-list-helper")?.spawn()?));
    let mut output = child
        .process()
        .stdout
        .take()
        .ok_or_else(|| anyhow!("camera list pipe unavailable"))?;
    let (sender, result) = async_channel::bounded(1);
    std::thread::Builder::new()
        .name("camera-list".into())
        .spawn(move || {
            let listed = read_bounded(&mut output, MAX_DEVICE_LIST)
                .map_err(anyhow::Error::from)
                .and_then(|bytes| Ok(serde_json::from_slice::<Vec<CameraDevice>>(&bytes)?));
            let _ = sender.send_blocking(listed);
        })?;
    tokio::time::timeout(Duration::from_secs(10), result.recv())
        .await
        .map_err(|_| anyhow!("Camera discovery timed out. Try refreshing devices."))?
        .map_err(|_| anyhow!("Camera discovery failed. Try refreshing devices."))?
}

/// Intercept before regular application startup. No settings or account storage is opened.
pub fn run_list_helper() {
    if let Ok(infos) = nokhwa::query(ApiBackend::Auto)
        && let Ok(bytes) = serde_json::to_vec(&camera_devices(&infos))
    {
        let _ = write_bounded(&mut std::io::stdout().lock(), &bytes, MAX_DEVICE_LIST);
    }
}

#[derive(Default)]
struct Frames {
    peer_keyframe_needed: AtomicBool,
    local: Mutex<Option<egui::ColorImage>>,
    peer: Mutex<Option<egui::ColorImage>>,
    error: Mutex<Option<&'static str>>,
    context: Mutex<Option<egui::Context>>,
}

/// Shared latest pictures only. A view never contains a protocol identity or camera handle.
#[derive(Clone, Default)]
pub struct VideoView(Arc<Frames>);
impl std::fmt::Debug for VideoView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VideoView")
    }
}
impl PartialEq for VideoView {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl VideoView {
    #[cfg(any(test, feature = "demo"))]
    pub fn synthetic(local: egui::ColorImage, peer: egui::ColorImage) -> Self {
        let view = Self::default();
        view.publish(true, local);
        view.publish(false, peer);
        view
    }
    pub fn attach(&self, ctx: &egui::Context) {
        *self.0.context.lock().unwrap_or_else(|e| e.into_inner()) = Some(ctx.clone());
    }
    pub fn take_local(&self) -> Option<egui::ColorImage> {
        self.0
            .local
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take()
    }
    pub fn take_peer(&self) -> Option<egui::ColorImage> {
        self.0.peer.lock().unwrap_or_else(|e| e.into_inner()).take()
    }
    pub fn error(&self) -> Option<&'static str> {
        *self.0.error.lock().unwrap_or_else(|e| e.into_inner())
    }
    fn publish(&self, local: bool, image: egui::ColorImage) {
        let slot = if local { &self.0.local } else { &self.0.peer };
        *slot.lock().unwrap_or_else(|e| e.into_inner()) = Some(image);
        self.wake();
    }
    fn fail(&self, message: &'static str) {
        *self.0.error.lock().unwrap_or_else(|e| e.into_inner()) = Some(message);
        self.wake();
    }
    fn wake(&self) {
        if let Some(ctx) = self
            .0
            .context
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
        {
            ctx.request_repaint();
        }
    }
}

/// Owns both camera and decoder lifetimes. Dropping closes every outgoing media queue immediately.
pub struct VideoSession {
    pub handle: Option<Arc<CallHandle>>,
    stop: Arc<AtomicBool>,
    child: Arc<Mutex<Option<Child>>>,
    control: async_channel::Sender<()>,
    source: async_channel::Receiver<Vec<u8>>,
    sink: async_channel::Sender<VideoFrame>,
    pub view: VideoView,
}
impl VideoSession {
    /// Called only after an explicit video action. Device/permission errors are returned without
    /// upgrading the call. Slow native camera operations run outside the WhatsApp worker.
    pub async fn open(selected: Option<String>) -> Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let view = VideoView::default();
        let (encoded, source) = async_channel::bounded(2);
        let (sink, peer) = async_channel::bounded::<VideoFrame>(2);
        let (ready, opened) = async_channel::bounded(1);
        let mut command = helper_command("--camera-helper")?;
        let mut process = HelperChild(Some(command.spawn()?));
        let mut output = process
            .process()
            .stdout
            .take()
            .ok_or_else(|| anyhow!("camera helper has no output pipe"))?;
        let mut input = process
            .process()
            .stdin
            .take()
            .ok_or_else(|| anyhow!("camera helper has no control pipe"))?;
        let selection = serde_json::to_vec(&selected)?;
        if selection.len() > MAX_SELECTION {
            return Err(anyhow!(
                "The saved camera selection is invalid. Choose another camera."
            ));
        }
        let (control, requests) = async_channel::bounded::<()>(1);
        let child = Arc::new(Mutex::new(Some(process.into_process())));
        let session = Self {
            handle: None,
            stop: stop.clone(),
            child: child.clone(),
            control: control.clone(),
            source,
            sink,
            view: view.clone(),
        };
        std::thread::Builder::new()
            .name("camera-keyframe-control".into())
            .spawn(move || {
                // Pipe writes can block before a newly spawned helper starts reading. Keep this
                // off the runtime, with the session already owning the kill-on-cancel guard.
                if write_bounded(&mut input, &selection, MAX_SELECTION).is_err() {
                    return;
                }
                while requests.recv_blocking().is_ok() {
                    if input.write_all(&[1]).is_err() {
                        break;
                    }
                }
            })?;
        std::thread::Builder::new()
            .name("call-camera".into())
            .spawn(move || {
                let Ok(mut decoder) = openh264::decoder::Decoder::new() else {
                    return;
                };
                while !stop.load(Ordering::Acquire) {
                    let Ok((kind, payload)) = read_packet(&mut output) else {
                        break;
                    };
                    if stop.load(Ordering::Acquire) {
                        break;
                    }
                    match kind {
                        0 => {
                            let _ = ready.try_send(Ok(()));
                        }
                        1 => {
                            if let Ok(Some(frame)) = decoder.decode(&payload)
                                && let Some(image) = crate::video::picture(&frame, 0, 640)
                            {
                                view.publish(true, image);
                            }
                            if encoded.try_send(payload).is_err() {
                                request_keyframe(&control);
                            }
                        }
                        3 => {
                            let _ = ready.try_send(Err(
                                "The selected camera is unavailable. Choose another camera.",
                            ));
                            break;
                        }
                        _ => break,
                    }
                }
                if !stop.load(Ordering::Acquire) {
                    let _ = ready.try_send(Err(
                        "Camera access failed. Check permission and device availability.",
                    ));
                    view.fail("The camera stopped. Check its connection and permissions.");
                }
                if let Some(mut process) = child.lock().unwrap_or_else(|e| e.into_inner()).take() {
                    let _ = process.kill();
                    let _ = process.wait();
                }
            })?;
        let decode_stop = session.stop.clone();
        let decode_view = session.view.clone();
        std::thread::Builder::new()
            .name("call-video-decoder".into())
            .spawn(move || {
                let Ok(mut decoder) = openh264::decoder::Decoder::new() else {
                    decode_view.fail("Video decoding is unavailable.");
                    return;
                };
                while let Ok(frame) = peer.recv_blocking() {
                    if decode_stop.load(Ordering::Acquire) {
                        return;
                    }
                    if frame.data.len() > MAX_AU {
                        decode_view
                            .0
                            .peer_keyframe_needed
                            .store(true, Ordering::Release);
                        continue;
                    }
                    match decoder.decode(&frame.data) {
                        Ok(Some(yuv)) => {
                            if let Some(image) = crate::video::picture(&yuv, frame.orientation, 720)
                                && !decode_stop.load(Ordering::Acquire)
                            {
                                decode_view.publish(false, image);
                            }
                        }
                        Ok(None) => {}
                        Err(_) => {
                            decode_view
                                .0
                                .peer_keyframe_needed
                                .store(true, Ordering::Release);
                            decode_view.fail("Some incoming video frames could not be decoded.");
                        }
                    }
                }
            })?;
        match tokio::time::timeout(Duration::from_secs(20), opened.recv()).await {
            Ok(Ok(Ok(()))) => Ok(session),
            Ok(Ok(Err(message))) => Err(anyhow!(message)),
            _ => Err(anyhow!(
                "The camera did not open in time. Check its permissions and availability."
            )),
        }
    }
    pub fn source(&self) -> async_channel::Receiver<Vec<u8>> {
        self.source.clone()
    }
    pub fn sink(&self) -> async_channel::Sender<VideoFrame> {
        self.sink.clone()
    }
    pub fn request_keyframe(&self) {
        request_keyframe(&self.control);
    }
    pub fn recover_peer(&self, handle: &CallHandle) {
        if self
            .view
            .0
            .peer_keyframe_needed
            .swap(false, Ordering::AcqRel)
        {
            handle.request_peer_keyframe(KeyframeUrgency::Immediate);
        }
    }
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Release);
        self.source.close();
        self.sink.close();
        if let Some(mut process) = self.child.lock().unwrap_or_else(|e| e.into_inner()).take() {
            let _ = process.kill();
            let _ = process.wait();
        }
        self.control.close();
        self.view
            .0
            .local
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take();
        self.view
            .0
            .peer
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take();
    }
}
impl std::fmt::Debug for VideoSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VideoSession")
    }
}
impl Drop for VideoSession {
    fn drop(&mut self) {
        self.stop();
    }
}

fn request_keyframe(control: &async_channel::Sender<()>) {
    let _ = control.try_send(());
}

/// Internal entry point, intercepted before settings, logging, sessions, or the archive open.
/// The helper communicates only through inherited private pipes and never logs media.
pub fn run_helper() {
    let mut input = std::io::stdin();
    let selected = read_bounded(&mut input, MAX_SELECTION)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Option<String>>(&bytes).ok());
    let Some(selected) = selected else {
        return;
    };
    let stop = Arc::new(AtomicBool::new(false));
    let keyframe = Arc::new(AtomicBool::new(true));
    let command_stop = stop.clone();
    let command_keyframe = keyframe.clone();
    let _ = std::thread::Builder::new()
        .name("camera-control".into())
        .spawn(move || {
            let mut input = std::io::stdin().lock();
            let mut command = [0_u8; 1];
            while input.read_exact(&mut command).is_ok() {
                if command[0] == 1 {
                    command_keyframe.store(true, Ordering::Release);
                }
            }
            command_stop.store(true, Ordering::Release);
        });
    let mut output = std::io::stdout().lock();
    if capture(selected.as_deref(), &stop, &keyframe, &mut output).is_err() {
        let _ = write_packet(&mut output, 2, &[]);
    }
}

fn write_packet(output: &mut impl Write, kind: u8, payload: &[u8]) -> std::io::Result<()> {
    if payload.len() > MAX_AU {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "camera frame too large",
        ));
    }
    output.write_all(&[kind])?;
    output.write_all(&(payload.len() as u32).to_le_bytes())?;
    output.write_all(payload)?;
    output.flush()
}

fn read_packet(input: &mut impl Read) -> std::io::Result<(u8, Vec<u8>)> {
    let mut header = [0_u8; 5];
    input.read_exact(&mut header)?;
    let size = u32::from_le_bytes(header[1..].try_into().expect("fixed header")) as usize;
    if size > MAX_AU || header[0] > 3 || (header[0] != 1 && size != 0) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "invalid camera helper frame",
        ));
    }
    let mut payload = vec![0; size];
    input.read_exact(&mut payload)?;
    Ok((header[0], payload))
}

fn capture(
    selected: Option<&str>,
    stop: &AtomicBool,
    keyframe: &AtomicBool,
    output: &mut impl Write,
) -> Result<()> {
    if stop.load(Ordering::Acquire) {
        return Ok(());
    }
    #[cfg(target_os = "macos")]
    {
        let (permission, permitted) = std::sync::mpsc::channel();
        nokhwa::nokhwa_initialize(move |allowed| {
            let _ = permission.send(allowed);
        });
        if !permitted
            .recv_timeout(Duration::from_secs(20))
            .unwrap_or(false)
        {
            return Err(anyhow!("camera permission not granted"));
        }
    }
    if stop.load(Ordering::Acquire) {
        return Ok(());
    }
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Closest(
        CameraFormat::new(Resolution::new(WIDTH, HEIGHT), FrameFormat::MJPEG, FPS),
    ));
    let index = if selected.is_some() {
        match selected_index(&nokhwa::query(ApiBackend::Auto)?, selected) {
            Ok(index) => index,
            Err(error) => {
                write_packet(output, 3, &[])?;
                return Err(error);
            }
        }
    } else {
        CameraIndex::Index(0)
    };
    let mut camera = Camera::new(index, requested)?;
    if stop.load(Ordering::Acquire) {
        return Ok(());
    }
    camera.open_stream()?;
    if stop.load(Ordering::Acquire) {
        let _ = camera.stop_stream();
        return Ok(());
    }
    let mut encoder = encoder()?;
    write_packet(output, 0, &[])?;
    let mut next = Instant::now();
    let mut number = 0_u64;
    while !stop.load(Ordering::Acquire) {
        let frame = camera.frame()?;
        if stop.load(Ordering::Acquire) {
            break;
        }
        let resolution = frame.resolution();
        if resolution.width() == 0
            || resolution.height() == 0
            || resolution.width() > 4096
            || resolution.height() > 4096
        {
            return Err(anyhow!("unsupported camera dimensions"));
        }
        let rgb = frame.decode_image::<RgbFormat>()?;
        if rgb.width() == 0 || rgb.height() == 0 || rgb.width() > 4096 || rgb.height() > 4096 {
            return Err(anyhow!("unsupported camera dimensions"));
        }
        let (width, height) = encoding_dimensions(rgb.width(), rgb.height())?;
        let rgb =
            image::imageops::resize(&rgb, width, height, image::imageops::FilterType::Triangle);
        let yuv = YUVBuffer::from_rgb8_source(RgbSliceU8::new(
            rgb.as_raw(),
            (width as usize, height as usize),
        ));
        if keyframe.swap(false, Ordering::AcqRel) || number.is_multiple_of(u64::from(FPS) * 2) {
            encoder.force_intra_frame();
        }
        let access = encoder.encode(&yuv)?.to_vec();
        // Queues are bounded. A dropped delta frame means the next frame must be an IDR.
        if !stop.load(Ordering::Acquire) {
            write_packet(output, 1, &access)?;
        }
        number += 1;
        next += Duration::from_secs_f64(1.0 / f64::from(FPS));
        if let Some(delay) = next.checked_duration_since(Instant::now()) {
            std::thread::sleep(delay);
        } else {
            next = Instant::now();
        }
    }
    let _ = camera.stop_stream();
    Ok(())
}

// H.264 needs even dimensions. Bound the longest side and retain portrait/landscape shape,
// allowing only the subpixel rounding needed by the chroma plane, never a fixed 4:3 stretch.
fn encoding_dimensions(width: u32, height: u32) -> Result<(u32, u32)> {
    if width < 2 || height < 2 || width > 4096 || height > 4096 {
        return Err(anyhow!("unsupported camera dimensions"));
    }
    let scale = (640.0 / f64::from(width.max(height))).min(1.0);
    let even = |value: u32| ((f64::from(value) * scale).round() as u32 / 2 * 2).max(2);
    Ok((even(width), even(height)))
}

fn encoder() -> Result<Encoder> {
    let config = EncoderConfig::new()
        .bitrate(BitRate::from_bps(600_000))
        .max_frame_rate(FrameRate::from_hz(FPS as f32))
        .profile(Profile::Baseline);
    Ok(Encoder::with_api_config(
        openh264::OpenH264API::from_source(),
        config,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn camera_selection_survives_reordering_and_refuses_a_missing_device() {
        let a = CameraInfo::new("Front", "", "device-a", CameraIndex::Index(0));
        let b = CameraInfo::new("Rear", "", "device-b", CameraIndex::Index(1));
        let chosen = camera_id(&b);
        let reordered = CameraInfo::new("Rear", "", "device-b", CameraIndex::Index(0));
        assert_eq!(
            selected_index(&[reordered, a.clone()], Some(&chosen)).unwrap(),
            CameraIndex::String("device-b".into())
        );
        assert!(selected_index(&[a], Some(&chosen)).is_err());
        assert_eq!(selected_index(&[], None).unwrap(), CameraIndex::Index(0));
    }

    #[test]
    fn replacement_at_the_same_v4l_node_does_not_match_the_saved_camera() {
        let previous = CameraInfo::new("Front", "/dev/video0", "", CameraIndex::Index(0));
        let replacement = CameraInfo::new("Other", "/dev/video0", "", CameraIndex::Index(0));
        assert!(selected_index(&[replacement], Some(&camera_id(&previous))).is_err());
    }

    #[test]
    fn metadata_pipe_round_trips_and_rejects_oversized_or_truncated_input() {
        let device = CameraDevice {
            id: "invented-device".into(),
            label: "Invented camera".into(),
        };
        let mut bytes = Vec::new();
        write_bounded(
            &mut bytes,
            &serde_json::to_vec(&vec![device.clone()]).unwrap(),
            MAX_DEVICE_LIST,
        )
        .unwrap();
        let payload = read_bounded(&mut bytes.as_slice(), MAX_DEVICE_LIST).unwrap();
        assert_eq!(
            serde_json::from_slice::<Vec<CameraDevice>>(&payload).unwrap(),
            vec![device]
        );
        bytes.pop();
        assert!(read_bounded(&mut bytes.as_slice(), MAX_DEVICE_LIST).is_err());
        assert!(
            read_bounded(
                &mut ((MAX_DEVICE_LIST + 1) as u32).to_le_bytes().as_slice(),
                MAX_DEVICE_LIST
            )
            .is_err()
        );
        assert!(write_bounded(&mut Vec::new(), &[0; 5], 4).is_err());
    }

    #[test]
    fn camera_encoding_keeps_portrait_and_landscape_shape() {
        assert_eq!(encoding_dimensions(1920, 1080).unwrap(), (640, 360));
        assert_eq!(encoding_dimensions(1080, 1920).unwrap(), (360, 640));
        assert_eq!(encoding_dimensions(640, 480).unwrap(), (640, 480));
        assert_eq!(encoding_dimensions(320, 240).unwrap(), (320, 240));
        assert!(encoding_dimensions(0, 480).is_err());
        assert!(encoding_dimensions(5000, 480).is_err());
    }

    #[test]
    fn synthetic_camera_picture_round_trips_through_h264_without_a_device() {
        use openh264::formats::YUVSource;
        for (width, height) in [(WIDTH as usize, HEIGHT as usize), (360, 640)] {
            let rgb = vec![100_u8; width * height * 3];
            let yuv = YUVBuffer::from_rgb8_source(RgbSliceU8::new(&rgb, (width, height)));
            let access = encoder().unwrap().encode(&yuv).unwrap().to_vec();
            assert!(access.len() < MAX_AU);
            let mut decoder = openh264::decoder::Decoder::new().unwrap();
            let decoded = decoder.decode(&access).unwrap().unwrap();
            assert_eq!(decoded.dimensions(), (width, height));
        }
    }
    #[test]
    fn preview_retains_only_the_latest_picture() {
        let view = VideoView::default();
        for value in [10, 20, 30] {
            view.publish(true, egui::ColorImage::from_rgb([2, 2], &[value; 12]));
        }
        assert_eq!(
            view.take_local().unwrap().pixels[0],
            egui::Color32::from_rgb(30, 30, 30)
        );
        assert!(view.take_local().is_none());
        assert!(view.take_peer().is_none());
    }

    #[test]
    fn helper_pipe_is_framed_and_rejects_oversized_or_truncated_data() {
        let mut bytes = Vec::new();
        write_packet(&mut bytes, 1, &[1, 2, 3]).unwrap();
        assert_eq!(
            read_packet(&mut bytes.as_slice()).unwrap(),
            (1, vec![1, 2, 3])
        );
        bytes.pop();
        assert!(read_packet(&mut bytes.as_slice()).is_err());
        let mut oversized = vec![1];
        oversized.extend_from_slice(&((MAX_AU + 1) as u32).to_le_bytes());
        assert!(read_packet(&mut oversized.as_slice()).is_err());
    }
}
