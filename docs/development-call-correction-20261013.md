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
detector reported no findings. The final helper probe returned the specific
resource-failure code before declaring video ready. Actual camera streaming is
not claimed successful.

An explicitly selected hardware enumeration test read two microphones and two
speakers without enumeration errors. Camera enumeration found one camera. These
counts prove local enumeration, not acceptance of native selector clicks or peer
audio. Worker and app regressions independently prove that lists and discovery
errors reach account state and clear after a successful refresh.

Commit, matching CI, release build and installation remain pending. The previous
installed build is separate from this source correction. Existing dirty checkpoint
files and `packaging/__pycache__/` are preserved.
