# Voice-to-video correction, 13 October 2026

Starting point: `792881fc` on main. Existing dirty development-ux evidence and
packaging/__pycache__ are preserved. The prior matching CI run
[38477502877](https://github.com/eternalwaitt/zap-faster/actions/runs/38477502877)
finished cancelled, not successful.

## Audio integration

The attended call had working bidirectional voice before camera-on. Decoding
stopped at 279 frames while authenticated reception continued and MLOW rejection
increased from 63 to 340. Those counters establish receive decoder rejection;
they do not identify the exact rejected setting. No new live call was placed.

The pinned library's PCM convenience facade configures MLOW_16KHZ_60MS. Its
MLOW decoder accepts one operating range, refusing low-rate geometry, fullband
internal rates, unsupported durations and standard-Opus escapes that reach it.
Turning on video does not rebind this app's microphone or speaker channels.
The library's public encoded_audio API instead supports OPUS_16KHZ_60MS and
signals standard Opus with the MLOW capability disabled. The app now uses that
supported route for offers and answers. WaOpusEncoder and WaOpusDecoder from
the same library do the codec work; no protocol parsing or crate patch was added.

The endpoint pair lives with its call and survives video upgrade/downgrade.
Its queues are bounded and close at teardown. Native decoder/encoder counters
are distinct from engine delivery counters. Local decode errors and a refused
fixed-codec source reach the existing audio warning; they are not credited as
successful decoding or silently hidden by the engine's encoded handoff count.

Regression coverage includes native Opus bandwidth and 20/60/120 ms duration
changes, bounded endpoint round trips, teardown, and codec refusal visibility.
These are synthetic codec/integration checks, not proof that a particular phone
honors negotiation or that both sides hear after camera-on. A successful attended
transition remains required. The exact prior MLOW rejection subtype is unresolved.

## Video investigation and correction

Capture and RTP defaults both remain 15 fps. No cadence mismatch is assumed.
The closest native camera mode can produce more samples. Sleeping in the capture
loop can leave driver samples queued; it now reads every sample and encodes at
an anchored 15 fps schedule with a small scheduling tolerance. A synthetic timing
fixture prevents tiny native jitter from skipping every second nominal frame.

A private helper envelope carries the monotonic capture clock and conversion/
encoding cost. A timestamped VideoSource passes capture time to the library's
supported timed-frame path. Dropped access units therefore retain gaps in the
media clock. Queue loss requests an IDR and suppresses dependent delta frames
until a clean IDR can be enqueued. An IDR dropped under continuing pressure
requests another, rather than waiting for the periodic keyframe.

Numeric counters distinguish captured/encoded frames and bytes, queue losses,
conversion/encoding cost, peer decoder deliveries/cost and images taken by the
UI. They contain no media, identity, device name, packet dump or call access code.
Capture timestamps measure sample retrieval, not the sensor's hardware clock.
UI image consumption is not a GPU-presentation or peer-rendering measurement.
No outbound wire timing, network delivery or peer rendering was measured in this
continuation. These local corrections do not establish the sole cause of the
reported lag or claim smooth peer video without attended evidence.

## Native controls

Call-control rows use one height based on their longest wrapped label, with
consistent eight-point gaps. Minimize call collapses the panel to a movable
240-point overlay. Video keeps peer and local pictures; voice uses a compact
name/control bar. Mute, camera, end-call and Restore call remain accessible.
Incoming-audio warnings and camera errors survive minimization. The compact body scrolls independently of
the controls. The state is keyed by account and call generation and changes no
application/media state. Screen privacy and locked-chat restrictions are retained.
Minimize/restore copy is translated in all ten existing locales.

Source tests operate actual minimize/restore click targets, require equal footer
heights/spacing and preserve media generation. Existing short-window, enlarged
text, keyboard, privacy and account tests remain intact. An early test caught
three-point overflow at 400x300 with 150% text after the added header action;
Minimize now shares the scrolling header row with the call title/name rather than weakening the assertion. The first full suite also caught the standalone action pushing Your camera below the visible body; the shared header row fixes both regressions.

See [upstream integration review](upstream-integration-20261013.md) for the
separate 111-commit review and the retained protocol pin. Final gate, visual,
commit/CI and installation receipts will be recorded after completion.


The first bounded visual matrix exposed a cropped video preview at 150% text in
400x440. Video now fits the remaining scrolling-body budget, reserving room for
the local-camera caption. The minimized overlay uses the same calculation. A
native enlarged-text regression requires the complete preview rectangle to stay
inside its viewport. The task-owned release build was stopped before replacement
when that visual defect was found; no unfinished candidate was installed.


The encoded API explicitly leaves microphone muting to its caller. The app now
zeros captured PCM under its local mute gate before Opus encoding, purges queued
speech on mute, and keeps silence flowing without closing the microphone or
transport. Enqueue and purge share a short mutex; full outgoing queues drop a
frame instead of blocking that gate. Unmute waits for the library's signaling
result, retaining mute on failure. The interface reflects the actual local gate.
A regression queues speech, mutes, checks sustained decoded silence, and unmutes
on the same endpoint pair. No physical microphone or peer is used by that test.


## Frozen verification snapshot

All six required Windows gates pass on pinned Rust 1.98.0. Default and all-feature
library suites each pass 1,854 tests, with 13 existing hardware tests ignored.
Binary and update-helper tests also pass. Strict default/all-feature Clippy,
formatting and strict all-feature rustdoc pass. Existing OpenSSL debug-symbol
linker warnings remain unchanged. The final 325-input manifest is
`a18b8d99c591e59bc4b5213443c87fc580209b655e484b6f9f5c238cb84c1891`.

Both bounded rounds captured 12 synthetic views without foreground focus.
The confirmation shows complete enlarged-text previews, aligned controls,
compact voice/video overlays and retained privacy masking. No further polishing
round is planned. The structured source/visual snapshot is
[development-call-upgrade-20261013.json](verification/development-call-upgrade-20261013.json).

This snapshot precedes the optimized-build installation and matching remote CI.
Their final delivery receipts are task-owned artifacts under
`H:/Scratch/Codex/zap-call-followup-20261013`; inspect the pushed code's matching
GitHub Actions run for current remote results. Source/visual checks do not replace
an attended camera transition with working duplex audio and smooth peer video.
Version stays 0.30.0 and no release is requested or published.

## Installation and CI follow-up

After the user confirmed that no call or recording was active, the client quit
cleanly. A fresh backup at `H:/Backups/ZapFasterRollback/20261013-220104`
verified 5,918 files (5,933,489,790 bytes) by SHA-256. The executable rollback
script was rehearsed successfully before replacement. Session, archive and
the existing Windows keyring were preserved. The installed executable reports
0.30.0 and matches the optimized candidate's SHA-256:
`c3f3a0609ad631ad149ded2c9738bde4b01897226facc6cc37759c5750f17a09`.

[The initial matching CI run](https://github.com/eternalwaitt/zap-faster/actions/runs/38490733094)
exposed a race in the pre-existing synthetic microphone test on Ubuntu with all
features. Receiving a frame can precede the pump's subsequent diagnostic-counter
increment. The test now waits for that accounting within its existing five-second
deadline, retaining the counter, frame-size and non-silence assertions. This
follow-up changes only test code; the installed runtime remains the verified
`b366bcfe` build. The focused microphone test and all six required Windows
gates pass again after this correction. Default and all-feature library suites
each pass 1,854 tests, with the same 13 hardware tests ignored.

[The matching Flatpak build](https://github.com/eternalwaitt/zap-faster/actions/runs/38490733247)
passed. Final CI status must be checked against the pushed follow-up commit.
Audio continuity after camera-on and smooth peer video remain attended-test
requirements, rather than claims established by installation or synthetic tests.
