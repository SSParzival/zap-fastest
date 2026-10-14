# Call quality follow-up, 14 October 2026

The attended repeat on the diagnostic development build received both audio and
video, but failed quality acceptance. The user reported crackling before enabling
either camera and laggy video after both cameras were enabled. This does not
establish the cause of the earlier call that received no media.

## Attended evidence

Installed source: `e66761f5a9bc9d8edb1414a8451e1531d2b58df6`.
Executable SHA-256:
`792555874f001c2112026b38793b6f77ccc791660bd025a7a027dec81fe0db1c`.
The numeric-only export covers 18:56:41 through 18:57:20 UTC, call generation 1.

- Native relay: 599 packets received and handed onward, no reported transport
  drops or send failures. This does not prove absence of loss before this boundary.
- Audio: 278 authenticated packets delivered to Opus, all 278 decoded and queued
  for playback, zero codec rejections or receive-pipe drops. Ten audio SRTP
  authentication failures are separate from the successful deliveries.
- Speaker: ten opens, nine stalls and nine restarts. Four restarts preceded
  local camera-on at 18:57:05 UTC. A stream restart interrupts queued audio.
- Video: 165 access units reached the decoder, 93 produced pictures and 72 failed
  decoding. All 93 published pictures were taken by the interface. Decoder time
  totalled 23,322 microseconds, which does not suggest a slow decoder backlog.
  This timing excludes picture conversion and painting.

## Playback correction

The speaker watchdog compared `rodio::Player::get_pos()` against its previous
high-water mark. Rodio tracks position independently for each appended source.
Every received PCM block is a separate source, so normal playback resets that
position. Once the watchdog observed a high position, later healthy blocks could
fail to exceed it for the 500 ms stall window and trigger a stream restart.

The correction counts progress while rodio actually consumes samples, accumulating
each source's position deltas into one clock for that opened stream. Queuing
audio does not advance this clock. Device changes create a new clock; the
existing startup and genuine-stall recovery rules remain in place.

The regression drives the actual rodio player and mixer offline across variable
packet sizes. A synthetic speaker that reports an ever-increasing position would
hide this integration bug, so it is not used for this check.

## Video correction

The call decoder used OpenH264's default flush-after-decode behavior, whereas
the existing file player disables that mode because it breaks streams with
reordered frames. The call path now also retains decoder state between access
units. A regression sends the existing synthetic MP4 fixture's parameter sets
and access units through the real call receive thread. Its B-frames exercise
the behavior omitted by the previous single-picture encoding test.

The old attended counters do not identify the peer's H.264 frame types or the
exact decoder error. This correction addresses a reproducible receive defect;
an attended repeat must determine how much of the reported video failure it
resolves. No peer payload or media was captured to obtain this evidence.

## Verification and delivery

The companion verification record separates baseline reproductions, corrected
checks, installation and attended acceptance. Preserve the original non-delivery
incident as unresolved until there is evidence of its cause. Do not publish a
release or describe acceptable call quality based on synthetic tests or CI.

The focused corrected call suite passed 69 tests, with one hardware test ignored.
This includes both quality regressions, actual speaker-stall recovery, the
library-to-Opus receive boundary, mute, transition lifetime, account routing and
the synthetic call interface checks. The first video baseline produced 34 decode
failures on the existing B-frame fixture; the corrected receive thread produced
none. The speaker harness drains each source through rodio's keep-alive silence
before sampling, and confirms that the original per-source clock resets while
the corrected stream clock continues advancing.

The frozen batch contains 328 inputs, manifest SHA-256
`98f18e7119852f13f360152ef1ea45dd6fb94f2bb7fc50367552e9237f33fa4a`.
See [the verification record](verification/development-call-quality-20261014.json)
for exact gate commands and outcomes. Runtime delivery receipts must be checked
against the installed executable's hash separately from these source checks.

All six required Windows gates passed for that frozen source: formatting,
strict default/all-feature Clippy, both all-target test suites and strict
all-feature rustdoc. Each library suite passed 1,859 tests with 13 hardware tests
ignored. The executable suites passed 10 and 12 tests respectively, and each
update-helper suite passed two. The three privacy-exporter tests also passed.
