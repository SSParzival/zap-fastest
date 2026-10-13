# Call correction, 13 October 2026

The attended failure was an outgoing voice call. Audio had already failed before
the camera was enabled. Earlier incoming voice success and outgoing zero-reception
reports remain separate evidence. Cold-start sticker batching and account-owned
preparation cancellation are retained.

## Camera

The installed executable matched SHA-256
`A078DFF3A7A77F493D5E886815717A73C19215347B8369643DF44A7D8B905A38`.
A private-pipe helper probe enumerated one camera and returned stream-ready followed
by failure, with no encoded frame. No images were saved, displayed or logged.
The app requests MJPEG, but Nokhwa's optional MJPEG decoding feature is disabled.
Its `RgbFormat` MJPEG conversion consequently returns an error. This is a separate
confirmed decoder defect, not proof of the observed camera's first failure.

A bounded native probe then reproduced frame-read failure across MJPEG, YUYV and
NV12 with HRESULT `0xC00D3704`. The installed camera helper has the same frame-read
boundary. Windows identifies this as `MF_E_HW_MFT_FAILED_START_STREAMING`, a
hardware stream that cannot start, for example because another application uses
the camera. It is not an access-denied HRESULT. Which application or driver state
holds those resources is not established. The user was asked whether another app
used the camera. No settings were changed and no other applications were closed.
See [Microsoft's media-source documentation](https://learn.microsoft.com/en-us/windows/win32/api/mfidl/nf-mfidl-imfmediasource-start).

MJPEG now uses the existing image crate's JPEG decoder, with 4096-pixel dimension
limits. Uncompressed formats retain Nokhwa's native conversions. Readiness waits
for an encoded first frame. Fixed helper codes distinguish camera opening, frame
capture, decoding, encoding, dimensions and confirmed permission refusal. Native
error text and device identities never enter the diagnostic log.

## Audio and devices

Allowlisted counters from the latest local runtime log show zero authenticated
incoming RTP, decoding and delivery throughout the sampled calls. Each opened one
microphone and one speaker stream without stalls or restarts. Microphone cleanup
capture counters progressed, and cleanup did not report failure. This establishes
local capture activity and absent authenticated incoming media, not peer reception
or the precise relay cause. No personal content or raw log lines were read.

The library owns offer negotiation, relay transport, decryption and RTP. The app
uses its ordinary call builder with stable microphone/output channels. The pinned
library does not expose outbound packet counters. Two additional fixed numeric
diagnostics count microphone frames handed to that channel and rejected forwarding
envelopes before RTP classification. The outgoing audio defect remains unresolved;
an attended repeat is required before attributing its exact cause upstream.
The current upstream commits after the pinned revision contain no VoIP correction.
No upstream crate was patched or vendored, and no call or message was placed.

Audio enumeration failures now propagate through the worker's account-local device
event and into the selectors. A panicked discovery task no longer becomes an
unexplained empty list. Opening the device section refreshes microphones, speakers
and cameras; enumeration does not open camera capture. Fixed numeric counts expose
published list sizes and failures without device names. The original reason all
three selectors appeared empty is not yet proven in the installed client's UI.

## Call presentation

The movable call panel defaults to 360 points at the upper right. It keeps labeled
inline microphone, camera and end-call controls visible, uses two rows for incoming
video answer choices, omits the large portrait and bounds the scrollable body.
An empty video waiting area takes 56 points and grows for real frames. Devices
remain collapsed until requested. Existing theme, keyboard, screen privacy and
account ownership are retained. A new window identity avoids restoring the old
large window geometry.

## Verification and delivery

All six required Windows gates passed against 324 frozen inputs (manifest SHA-256
`4fa0df019b76f1b5d6737dde5461afd611e11d698d6286348ae876c6a2d45c27`).
Default and all-feature library suites each passed 1,835 tests, with 13 existing
hardware tests ignored. Binary tests passed 10/12 respectively and update-helper
tests passed two in each configuration. Formatting, strict default/all-feature
Clippy and strict all-feature rustdoc passed. Existing OpenSSL debug-symbol linker
warnings remain separate from these passing source gates.

Both bounded synthetic rounds captured and inspected 12 views without taking
foreground focus. Controls retain at least 44-point targets. The existing layout
regression covers five window sizes, both themes and 100%/150% text, requiring all
three complete selector rectangles to become visible by scrolling. The mechanical
detector reported no findings. The initial corrected helper probe returned the specific
resource-failure code before declaring video ready. Actual camera streaming is
not claimed successful by that probe. The later successful retry is recorded below.

An explicitly selected hardware enumeration test read two microphones and two
speakers without enumeration errors. Camera enumeration found one camera. These
counts prove local enumeration, not acceptance of native selector clicks or peer
audio. Worker and app regressions independently prove that lists and discovery
errors reach account state and clear after a successful refresh.

The corrective batch was committed and pushed directly to `main` as
`592fe1fe31c492f75bed9cddff786beaedf83f75`. The default Windows release build passed
in 561.93 seconds. Installed readback matches SHA-256
`6229FEB968C429F431B9E8723643D15BB848686CF3A94ACB4301F14C4E9506B3`
(64,798,208 bytes). Version remains 0.30.0; no release was tagged or published.
Fresh backup `C:/Users/Victor/AppData/Local/ZapFasterRollback/20261013-193420`
contains 5,907 verified files (5,922,183,285 bytes). Executable rollback was
rehearsed before replacement. The linked session, archive and keyring were retained.
The client was not automatically reopened. Existing dirty checkpoint files and
`packaging/__pycache__/` are preserved.

The user reported no other known camera consumer. Windows reported one healthy
present webcam and no active-use marker in its recent camera-use metadata.
A standalone Windows Media Foundation probe then obtained a sample under all
four combinations of STA/MTA and enabled/disabled converters. Subsequent Nokhwa
frame reads and the corrected app helper succeeded too. The release executable
produced readiness followed by five nonempty H.264 packets; the harness intentionally
stopped it afterward. No images were saved, displayed or inspected. This proves
local capture, JPEG decoding and H.264 encoding on this webcam. It does not prove
peer video delivery or establish what cleared the earlier Windows resource failure.
The earlier failed probes remain recorded separately from this successful retry.

[Matching CI](https://github.com/eternalwaitt/zap-faster/actions/runs/38477502877)
is still running. Windows ARM64 compilation and dependency advisories have passed;
the other jobs require their own completion receipts. The user was asked to reopen
the usual shortcut and report outgoing voice before camera-on, camera-on and actual
device choices separately. The attended transition below supersedes the pending
audio/video acceptance; device selector choices remain unconfirmed.

## Attended camera transition, still unresolved

The user confirmed that both sides could hear before camera-on. After enabling
video, audio stopped and the peer reported laggy video. Camera-off recovery was
not tested. The supplied screenshot shows both video pictures and the incoming
audio warning. No personal screenshot content is retained here.

At 22:43:36 UTC, successful audio decoding reached 279 frames. It remained at
279 through 22:43:56, while authenticated RTP reception continued and MLOW
out-of-operating-point rejections rose from 63 to 340. Authentication failures,
unexpected payload types, sink drops and forwarding-envelope rejections stayed
zero. The microphone continued handing frames to the library, and microphone
and speaker opens remained one each, with no stalls, restarts or cleanup failure.
Channel handoff does not prove outbound wire delivery or what the peer heard.

This establishes receive-side codec rejection, separate from the earlier outgoing
call with zero authenticated incoming RTP. The decoder rejects standard-Opus
frames, non-16-kHz internal rates, low-rate geometry or unsupported durations.
Its aggregate counter does not identify which one the peer sent or prove why
camera-on triggered it. The integration uses public PCM audio and video-upgrade
APIs without rebinding audio devices. The default video-source RTP cadence is
15 fps and matches capture; no cadence mismatch was found.

The codec boundary is whatsapp-rust/wacore at pinned revision
`94c53e833805c58c9f5966b08d08c990650a651e`, in
`wacore/src/voip/mlow/decoder.rs:279` and `wacore/src/voip/engine.rs:3082`.
Upstream decoder history checked during this investigation has no newer decoder
fix than `2b9a8d799ec2da008cc47fbd1c61d8995d652238`, already in pinned history.
A dependency upgrade is therefore not an established fix. The repository
boundary forbids patching or vendoring the calling library here. The prerequisite
is an upstream negotiation/decoder correction followed by an attended transition
that keeps bidirectional audio working. No upstream message or issue was sent.

Video lag remains independently unresolved. Zero video sink drops do not measure
capture rate, outbound delivery, network delay or peer rendering. No lag fix is
claimed. The installed client was not replaced again or interrupted during the call.
