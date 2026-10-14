# Inbound media investigation, 14 October 2026

The attended outgoing call reported today failed before camera-on: the peer heard
the caller, but the caller heard no peer audio. After camera-on the peer could see
the caller; the caller received neither peer audio nor peer video. The peer's
camera transmission is not independently established. This failure supersedes
any earlier suggestion that calling was ready to release.

## Fresh evidence

The user confirmed the 17:28-17:30 UTC interval belongs to this call. A strict
counter-only export, without raw lines, identities or media, contains 57 records
from 17:28:25 through 17:29:40 UTC. Throughout that interval:

- Authenticated audio RTP, unexpected audio payloads, SRTP failures, SFrame
  failures, unclassified relay packets, forwarding rejections and sink drops
  stayed at zero.
- The Opus bridge decoded and rejected zero packets; it encoded 67 through
  1,315 local microphone frames. Encoding and channel submission alone do not
  prove outbound wire delivery; peer reception is the user's separate evidence.
- Video capture progressed from 17 to 617 frames after camera-on. The peer video
  sink received and decoded zero access units. Its zero sink-drop count does not
  prove that encrypted video packets never arrived at the engine: the existing
  engine counters do not report every video receive/authentication stage.

This establishes an absent audio input to the application codec, rather than a
repetition of the previous MLOW rejection. It does not identify the failing
transport layer or establish that the peer transmitted video. There are no new
attended results for the diagnostic build yet.

The checkout started at `f26265d4` on `main`, matching public `fork/main`. The
installed executable was freshly verified as version 0.30.0, SHA-256
`C3F3A0609AD631AD149DED2C9738BDE4B01897226FACC6CC37759C5750F17A09`.
Unrelated UX evidence and `packaging/__pycache__/` were preserved.

## Source boundary and diagnostic change

The application advertises the pinned library's `OPUS_16KHZ_60MS` format using
`encoded_audio` for both offers and answers. Its signaling uses the standard-Opus
capability with MLOW disabled. The encoded receive API delivers codec payloads
with format metadata, not PCM. The application bridge decodes them and queues
PCM for the stable speaker endpoint. No authenticated packet from this call
reached that bridge, so the peer's actual inbound payload mapping is unobserved.

The application keeps audio endpoints with the call while video endpoints are
attached or removed. Worker events remain account-owned; camera preparation is
guarded by the call generation. The reviewed source does not establish a receive
endpoint lifetime defect explaining this attended failure.

The new observer uses `Client::set_relay_transport_provider` and delegates to the
same `RelayMediaChannelFactory` as the library's native default. It counts packets
received from that factory, packets handed toward the engine, the library's own
RTP/RTCP/STUN-like classification, native drop reports, outbound transport
submissions and failures, and endpoint closure. Counters are scoped to the call
generation and each transport connection, including reconnects. It forwards
packets and control events unchanged through one bounded slot and retires on
receiver closure or transport teardown. STUN-like classification is not proof of
a valid STUN message; outbound submission is not a wire-send counter.

The Opus bridge separately reports received codec frames, decoded frames, PCM
submissions and channel closure. Video reports decode errors and frames waiting
for decoder output separately from decoded pictures. Existing audio/video path
records now carry the call generation. No packet bytes, keys, addresses, device
names, identities or message content are added to diagnostics.

Snapshots run on activation, mute, immediately before and after camera changes,
and before resource teardown. Peer-keyframe recovery keeps its existing heartbeat
cadence; reading a transition snapshot does not request network activity.

`packaging/export-call-diagnostics.py` exports only recognized event categories,
field names, numeric/boolean values and fixed transition markers. Unknown fields
reject the entire record. Use an explicit UTC `--since` and `--output` path.

The dependency stays at `94c53e833805c58c9f5966b08d08c990650a651e`.
The clean inspection checkout at `H:\Github\whatsapp-rust` was at `fb970318`;
the relevant engine, driver, transport and endpoint code matches the pinned
revision. Its newer facade changes concern generated protobuf construction.
Neither the inspection checkout nor Cargo's cached source was modified.
An additional fetch refreshed upstream `origin/main` to `1f25f7e1`. No changes
after the inspection checkout's HEAD touched the calling facade, transport,
engine or neutral calling contract. An upstream update is not an established
correction for this failure.

## Verification and next attended step

The subsequent attended repeat received audio before camera-on and video from
both cameras, but the user reported crackling audio and laggy video. It failed
quality acceptance. See the [quality follow-up](development-call-quality-20261014.md)
for its separate counters, playback defect and decoder investigation. The
original no-delivery incident's cause remains unproven.

Synthetic verification exercises real library SRTP authentication and encoded
audio delivery into the application's Opus bridge and speaker queue. It checks
outbound recovery through the peer pipeline as video is enabled, disabled and
enabled again. Synthetic H.264 is packetized by the library, received through its
engine and decoded by the same application receive thread used in real calls.
Transport tests cover unchanged packet/control forwarding, backpressure teardown
and separate call counters. These are diagnostic regressions, not a reproduction
or proof of correction of the live failure.

Windows gates, source manifest, CI, installation/backup receipts and subsequent
attended results are tracked in the companion verification record. A build or CI
pass does not resolve this incident.

Before the separate TLS patch below, all six Windows gates passed for 328 frozen inputs (manifest SHA-256
`63f0a24a0bb25a460763df8aea8d3de9690d253766209fa4564437a85c48fa6b`).
Default and all-feature library suites each passed 1,857 tests with 13 hardware
tests ignored. Binary suites passed 10/12 tests respectively; update-helper
suites passed two each. All three exporter privacy tests passed. Existing
OpenSSL missing-PDB linker warnings remain separate from these successful gates.

## CI security follow-up

The first CI run for `0834937c` failed its advisory gate on the existing Rustls
0.23.45 dependency. RustSec published four advisories dated 14 October 2026:
[0346](https://github.com/RustSec/advisory-db/blob/main/crates/rustls/RUSTSEC-2026-0346.md),
[0347](https://github.com/RustSec/advisory-db/blob/main/crates/rustls/RUSTSEC-2026-0347.md),
[0348](https://github.com/RustSec/advisory-db/blob/main/crates/rustls/RUSTSEC-2026-0348.md)
and [0349](https://github.com/RustSec/advisory-db/blob/main/crates/rustls/RUSTSEC-2026-0349.md).
Their official database entries all identify 0.23.46 as patched. The lockfile
follow-up changes only that package's version and checksum. This resolves a
separate dependency finding; it does not establish the inbound-media cause.

The follow-up's 328 frozen inputs have manifest SHA-256
`21c3b51b9741cca69c9c4a61b779d3917102eadd85e8ef7042ff82d01bc41d6c`.
Local Windows formatting, strict default/all-feature Clippy and strict rustdoc
cover the patched dependency. The matching Windows CI job supplies both full
all-target test suites, as permitted by the verification guide. The companion
record points to the exact CI and installed-build receipts. Those receipts must
be checked independently of the successful pre-patch test checkpoint above.

The next attended call must first test bidirectional voice without cameras. If
either direction fails, record that before camera-on. Then test cameras with
both people explicitly confirming transmission, camera-off, minimize/restore,
mute and end-call. The observer determines whether media reaches the public
native relay boundary; further transport-level diagnostics or a dependency
correction must follow the resulting evidence. Do not release this batch as an
inbound-media fix before those results exist.
