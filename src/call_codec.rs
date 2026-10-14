//! Standard Opus endpoints using the calling library's codec and negotiation APIs.
//! Keep the device channels alive across video upgrades; never advertise MLOW's
//! broader operating range when the built-in decoder implements only one point.

use anyhow::Result;
use async_channel::{Receiver, Sender};
use bytes::Bytes;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU32, Ordering},
};
use whatsapp_rust::voip::audio::{WaOpusDecoder, WaOpusEncoder};
use whatsapp_rust::voip::{AudioFormat, EncodedAudioFrame};

pub(crate) const FORMAT: AudioFormat = AudioFormat::OPUS_16KHZ_60MS;

#[derive(Default)]
struct Counters {
    received: AtomicU32,
    encoded: AtomicU32,
    decoded: AtomicU32,
    submitted: AtomicU32,
    rejected: AtomicU32,
    dropped: AtomicU32,
    failed: AtomicBool,
    source_failed: AtomicBool,
}

pub(crate) struct Audio {
    muted: Arc<Mutex<bool>>,
    counters: Arc<Counters>,
    pub source: Receiver<Bytes>,
    pub sink: Sender<EncodedAudioFrame>,
}

impl Audio {
    pub fn open(mic: Receiver<Vec<i16>>, speaker: Sender<Vec<i16>>) -> Result<Self> {
        let mut encoder = WaOpusEncoder::new()?;
        let mut decoder = WaOpusDecoder::new()?;
        let (encoded, source) = async_channel::bounded(2);
        let (sink, incoming) = async_channel::bounded::<EncodedAudioFrame>(4);
        let counters = Arc::new(Counters::default());
        let muted = Arc::new(Mutex::new(false));
        let mute_gate = muted.clone();
        let endpoints = Self {
            source,
            sink,
            counters: counters.clone(),
            muted,
        };
        let capture_counts = counters.clone();
        std::thread::Builder::new()
            .name("call-opus-encode".into())
            .spawn(move || {
                while let Ok(mut pcm) = mic.recv_blocking() {
                    // Encoded endpoints do not pass through the library's PCM mute feed.
                    // Serialize encoding/enqueue with mute's queue purge, and never block
                    // on a full network queue while holding this short-lived gate.
                    let muted = mute_gate.lock().unwrap_or_else(|e| e.into_inner());
                    if *muted {
                        pcm.fill(0);
                    }
                    match encoder.encode(&pcm) {
                        Ok(packet) => {
                            capture_counts.encoded.fetch_add(1, Ordering::Relaxed);
                            match encoded.try_send(Bytes::from(packet)) {
                                Ok(()) => {}
                                Err(async_channel::TrySendError::Full(_)) => {
                                    capture_counts.dropped.fetch_add(1, Ordering::Relaxed);
                                }
                                Err(async_channel::TrySendError::Closed(_)) => break,
                            }
                        }
                        Err(_) => {
                            capture_counts.source_failed.store(true, Ordering::Relaxed);
                            log::warn!("call Opus encode failed");
                            break;
                        }
                    }
                }
            })?;
        std::thread::Builder::new()
            .name("call-opus-decode".into())
            .spawn(move || {
                while let Ok(frame) = incoming.recv_blocking() {
                    counters.received.fetch_add(1, Ordering::Relaxed);
                    // The library identifies each frame's codec and RTP container.
                    // Native Opus negotiation must never feed an MLOW payload to libopus.
                    if frame.format.rtp_profile != FORMAT.rtp_profile || frame.codec != FORMAT.codec
                    {
                        counters.failed.store(true, Ordering::Relaxed);
                        counters.rejected.fetch_add(1, Ordering::Relaxed);
                        log::warn!("call unexpected negotiated audio format");
                        break;
                    }
                    match decoder.decode(&frame.data) {
                        Ok(pcm) => {
                            counters.decoded.fetch_add(1, Ordering::Relaxed);
                            counters.failed.store(false, Ordering::Relaxed);
                            if speaker.send_blocking(pcm.to_vec()).is_err() {
                                break;
                            }
                            counters.submitted.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(_) => {
                            counters.rejected.fetch_add(1, Ordering::Relaxed);
                            counters.failed.store(true, Ordering::Relaxed);
                            // Conceal a lost packet using libopus, preserving its prediction state.
                            if let Ok(pcm) = decoder.decode(&[])
                                && speaker.send_blocking(pcm.to_vec()).is_err()
                            {
                                break;
                            }
                        }
                    }
                }
            })?;
        Ok(endpoints)
    }
}

impl Audio {
    pub fn set_muted(&self, muted: bool) {
        let mut gate = self.muted.lock().unwrap_or_else(|e| e.into_inner());
        *gate = muted;
        if muted {
            while self.source.try_recv().is_ok() {}
        }
    }
    pub fn is_muted(&self) -> bool {
        *self.muted.lock().unwrap_or_else(|e| e.into_inner())
    }
    pub fn refuse_source(&self) {
        self.counters.source_failed.store(true, Ordering::Relaxed);
    }
    pub fn decoded(&self) -> u32 {
        self.counters.decoded.load(Ordering::Relaxed)
    }
    pub fn failed(&self) -> bool {
        self.counters.failed.load(Ordering::Relaxed)
            || self.counters.source_failed.load(Ordering::Relaxed)
    }
    pub fn log_stats(&self, generation: u64) {
        log::info!(
            "[CALL] Opus generation={} received={} encoded={} decoded={} submitted={} rejected={} dropped={} failed={} source_closed={} sink_closed={}",
            generation,
            self.counters.received.load(Ordering::Relaxed),
            self.counters.encoded.load(Ordering::Relaxed),
            self.decoded(),
            self.counters.submitted.load(Ordering::Relaxed),
            self.counters.rejected.load(Ordering::Relaxed),
            self.counters.dropped.load(Ordering::Relaxed),
            self.failed(),
            self.source.is_closed(),
            self.sink.is_closed(),
        );
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        self.source.close();
        self.sink.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn authenticated_rtp_reaches_the_real_opus_sink_before_and_after_camera_changes() {
        use whatsapp_rust::wacore::voip::{
            AudioConfig,
            engine::{CallConfig, CallEngine, Input, Output, SequentialTxIds},
            session::{
                CallDirection, MediaPipeline, MediaPipelineParams, VideoPipeline,
                VideoPipelineParams,
            },
        };
        let config = CallConfig {
            call_id: "synthetic-receive-boundary".into(),
            direction: CallDirection::Incoming,
            self_lid: "15550000001:0@lid".into(),
            peer_lid: "15550000002:0@lid".into(),
            call_key: vec![42; 32],
            ssrc: 0x5741_0001,
            audio: AudioConfig::encoded(FORMAT),
            relay_token: vec![1; 16],
            auth_token: vec![2; 8],
            relay_ip: "203.0.113.7".into(),
            relay_port: 3478,
            integrity_key: b"synthetic-relay-key".to_vec(),
            warp_mi_tag_len: 4,
            enable_media: true,
            enable_video: false,
            enable_sframe: false,
        };
        let mut engine = CallEngine::new(config.clone(), Box::new(SequentialTxIds::new())).unwrap();
        let mut peer = MediaPipeline::new(&MediaPipelineParams {
            call_key: &config.call_key,
            self_lid: &config.peer_lid,
            peer_lid: &config.self_lid,
            ssrc: config.ssrc,
            samples_per_packet: FORMAT.rtp_timestamp_step,
            warp_mi_tag_len: config.warp_mi_tag_len,
        })
        .unwrap();
        assert!(peer.set_audio_payload_type(FORMAT.rtp_payload_type));
        let mut peer_video = VideoPipeline::new(&VideoPipelineParams {
            call_key: &config.call_key,
            self_lid: &config.peer_lid,
            peer_lid: &config.self_lid,
            ssrc: 0x5741_0002,
            ts_stride: 6000,
            warp_mi_tag_len: config.warp_mi_tag_len,
        })
        .unwrap();
        let (mic, input) = async_channel::bounded(1);
        let (output, speaker) = async_channel::bounded(1);
        let audio = Audio::open(input, output).unwrap();
        let mut peer_encoder = WaOpusEncoder::new().unwrap();
        let voice: Vec<_> = (0..960)
            .map(|i| ((i as f32 * 0.17).sin() * 5000.0) as i16)
            .collect();

        for step in 0..4 {
            match step {
                1 | 3 => assert!(engine.enable_video()),
                2 => engine.disable_video(),
                _ => {}
            }
            // Encrypt a peer packet with the dependency's actual SRTP pipeline. The engine must
            // authenticate it, preserve the negotiated format and deliver it to our actual sink.
            let packet = peer.protect_audio(&peer_encoder.encode(&voice).unwrap());
            engine.handle_input(step, Input::RelayPacket(&packet));
            let mut delivered = false;
            loop {
                match engine.poll_output() {
                    Output::EncodedAudio(frame) => {
                        assert_eq!(frame.format, FORMAT);
                        audio.sink.send(frame).await.unwrap();
                        delivered = true;
                    }
                    Output::Timeout(_) => break,
                    _ => {}
                }
            }
            assert!(
                delivered,
                "inbound audio must survive each camera transition"
            );
            let pcm = tokio::time::timeout(std::time::Duration::from_secs(2), speaker.recv())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(pcm.len(), 960);
            assert!(pcm.iter().any(|sample| sample.abs() > 100));

            // The same live source must still reach the peer on the unchanged outbound profile.
            mic.send(voice.clone()).await.unwrap();
            let encoded =
                tokio::time::timeout(std::time::Duration::from_secs(2), audio.source.recv())
                    .await
                    .unwrap()
                    .unwrap();
            engine.handle_input(step, Input::EncodedAudio(&encoded));
            let mut sent = false;
            loop {
                match engine.poll_output() {
                    Output::Transmit(packet) => {
                        if let Some((_, recovered)) = peer.unprotect_audio(&packet) {
                            assert_eq!(recovered.as_slice(), encoded.as_ref());
                            sent = true;
                        }
                    }
                    Output::Timeout(_) => break,
                    _ => {}
                }
            }
            assert!(sent, "outbound audio must survive each camera transition");
            if step == 1 || step == 3 {
                // Exercise the app's real receive decoder on each new video endpoint. No
                // camera, socket, account data or platform speaker is opened by this test.
                use openh264::formats::{RgbSliceU8, YUVBuffer};
                let rgb = vec![100; 64 * 48 * 3];
                let yuv = YUVBuffer::from_rgb8_source(RgbSliceU8::new(&rgb, (64, 48)));
                let access = openh264::encoder::Encoder::new()
                    .unwrap()
                    .encode(&yuv)
                    .unwrap()
                    .to_vec();
                let (sink, view) = crate::camera::test_peer_sink();
                for packet in peer_video.protect_video(&access) {
                    engine.handle_input(step, Input::RelayPacket(&packet));
                    loop {
                        match engine.poll_output() {
                            Output::VideoPlayout(frame) => sink.send(frame).await.unwrap(),
                            Output::Timeout(_) => break,
                            _ => {}
                        }
                    }
                }
                let image = tokio::time::timeout(std::time::Duration::from_secs(2), async {
                    loop {
                        if let Some(image) = view.take_peer() {
                            break image;
                        }
                        tokio::task::yield_now().await;
                    }
                })
                .await
                .unwrap();
                assert_eq!(image.size, [64, 48]);
                drop(sink);
            }
        }
        assert_eq!(engine.media_stats().rtp_received, 4);
        assert_eq!(audio.decoded(), 4);
        assert!(!audio.failed());
        drop(audio);
        drop(mic);
    }

    #[test]
    fn native_opus_decodes_voice_and_video_bandwidth_changes() {
        use opus::{Application, Bandwidth, Channels, Encoder};
        let mut encoder = Encoder::new(16_000, Channels::Mono, Application::Voip).unwrap();
        encoder.set_dtx(false).unwrap();
        let mut decoder = WaOpusDecoder::new().unwrap();
        for bandwidth in [
            Bandwidth::Wideband,
            Bandwidth::Narrowband,
            Bandwidth::Wideband,
        ] {
            encoder.set_bandwidth(bandwidth).unwrap();
            for length in [320, 960, 1920] {
                let pcm: Vec<_> = (0..length)
                    .map(|i| ((i as f32 * 0.17).sin() * 5000.0) as i16)
                    .collect();
                let packet = encoder.encode_vec(&pcm, 1275).unwrap();
                let decoded = decoder.decode(&packet).unwrap();
                assert_eq!(decoded.len(), length);
                assert!(decoded.iter().any(|sample| *sample != 0));
            }
        }
        assert_eq!(
            FORMAT.rtp_profile,
            whatsapp_rust::voip::AudioRtpProfile::StandardOpus
        );
    }

    #[tokio::test]
    async fn codec_endpoints_keep_decoding_after_a_video_transition() {
        use std::time::Duration;
        let (mic, input) = async_channel::bounded(1);
        let (output, speaker) = async_channel::bounded(1);
        let audio = Audio::open(input, output).unwrap();
        for sequence in 0..4u16 {
            // Video attachment never replaces this source or sink.
            let source = audio.source.clone();
            let sink = audio.sink.clone();
            mic.send(vec![1000; 960]).await.unwrap();
            let data = tokio::time::timeout(Duration::from_secs(2), source.recv())
                .await
                .unwrap()
                .unwrap();
            let frame = EncodedAudioFrame::builder()
                .format(FORMAT)
                .codec(FORMAT.codec)
                .data(data)
                .payload_type(FORMAT.rtp_payload_type)
                .sequence_number(sequence)
                .timestamp(u32::from(sequence) * 960)
                .marker(false)
                .build();
            sink.send(frame).await.unwrap();
            let decoded = tokio::time::timeout(Duration::from_secs(2), speaker.recv())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(decoded.len(), 960);
        }
        assert_eq!(audio.decoded(), 4);
        assert!(!audio.failed());
        audio.refuse_source();
        assert!(
            audio.failed(),
            "a codec negotiation refusal must remain visible"
        );
    }

    #[tokio::test]
    async fn mute_flushes_queued_speech_and_silences_the_encoder_until_unmuted() {
        use std::time::Duration;
        let (mic, input) = async_channel::bounded(1);
        let (output, _speaker) = async_channel::bounded(1);
        let audio = Audio::open(input, output).unwrap();
        let loud: Vec<_> = (0..960)
            .map(|i| ((i as f32 * 0.17).sin() * 16000.0) as i16)
            .collect();
        mic.send(loud.clone()).await.unwrap();
        mic.send(loud.clone()).await.unwrap();
        tokio::time::timeout(Duration::from_secs(2), async {
            while audio.source.len() < 2 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        audio.set_muted(true);
        assert!(audio.is_muted());
        assert!(
            audio.source.is_empty(),
            "old encoded speech must be discarded on mute"
        );
        let mut decoder = WaOpusDecoder::new().unwrap();
        for frame in 0..4 {
            mic.send(loud.clone()).await.unwrap();
            let packet = tokio::time::timeout(Duration::from_secs(2), audio.source.recv())
                .await
                .unwrap()
                .unwrap();
            let pcm = decoder.decode(&packet).unwrap();
            assert!(!pcm.is_empty(), "muting keeps the media stream fed");
            if frame >= 2 {
                assert!(
                    pcm.iter().all(|sample| sample.abs() < 100),
                    "no live microphone signal while muted"
                );
            }
        }
        audio.set_muted(false);
        assert!(!audio.is_muted());
        let mut audible = false;
        for _ in 0..4 {
            mic.send(loud.clone()).await.unwrap();
            let packet = tokio::time::timeout(Duration::from_secs(2), audio.source.recv())
                .await
                .unwrap()
                .unwrap();
            audible |= decoder
                .decode(&packet)
                .unwrap()
                .iter()
                .any(|sample| sample.abs() > 1000);
        }
        assert!(audible, "unmuting restores this same source");
    }

    #[test]
    fn closing_endpoints_wakes_waiting_codec_workers() {
        let (mic, input) = async_channel::bounded(1);
        let (output, speaker) = async_channel::bounded(1);
        let audio = Audio::open(input, output).unwrap();
        let source = audio.source.clone();
        let sink = audio.sink.clone();
        drop(audio);
        assert!(source.is_closed());
        assert!(sink.is_closed());
        drop(mic);
        drop(speaker);
    }
}
