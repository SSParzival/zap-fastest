# Native UI and UX improvement batch, 2026-10-13

Task contract: improve all existing surfaces using Impeccable Operate guidance,
preserving the native Rust/egui client, theme identity, account isolation and
protocol boundaries. No release tag or publication. Only synthetic offline
content may appear in visual evidence. Personal installation and attended
testing are authorized, after verified cold backup and executable rollback.

## Recovered inputs

Starting main: `2a41af748ec173ee45fc76ae99be83f3cf4f848f`.
Flatpak `38338474925` passed; source CI `38338443966` subsequently passed
all nine jobs. Earlier CI `38323347352` passed all nine jobs.
These receipts describe the starting source, not this batch.
Existing dirty checkpoint/evidence files and `packaging/__pycache__/` are
preserved. Previous personal-account startup and synthetic fixture delivery
were accepted; connected call timing and several native flows were pending.

## Surface inventory and prioritized worklist

Status vocabulary: **fix** means implemented and covered by the successful local gates;
**existing** means source/tests provide the behavior, native acceptance is
separate; **pending** names the prerequisite and does not claim completion.
The visual round must cover the relevant variants together after implementation.

| Surface and flow | Finding or reviewed behavior | Disposition and evidence |
| --- | --- | --- |
| Shell, wide/narrow navigation | Existing structural list/detail switching, back navigation and history controls | Existing: `ui/mod.rs`, narrow navigation regressions |
| Chat list and filters | Horizontal chip scrolling, counts, pinned ordering and local labels | Existing: `ui/chats.rs`, `ui/labels.rs`, demo focus/filter tests |
| Global chat search | Arrow/Enter navigation and clear returns focus | Existing: `ui/chats.rs`, `ui/widgets.rs` |
| Message search and date filter | Pending empty results showed only a spinner | P2 fix: readable search status, `ui/pane.rs` |
| Account switcher | Fixed row height and unbounded list can clip large text/many accounts | P1 fix: measured row height, bounded scroll and viewport width, `ui/accounts.rs` |
| Account identity and removal | Local identity, offline archive status, confirmation and backend stop | Existing; removal is not exercised on personal accounts |
| Conversation hierarchy and grouping | Sender, quote, forwarded/edit state, footer, reactions and system entries | Existing: `ui/conversation.rs`; review synthetic variants |
| Composer, drafts and send admission | Draft ownership, reply/edit cancellation, connection/permission gating | Existing: `app.rs`, composer regressions; no autonomous contact sends |
| Attachment staging and editing | Tile staging, caption, crop/rotate, cancel and batch partial outcomes | Existing; crop keyboard and short-window review required |
| Replies, selections and menus | Stable target identities, text copy annotation, batch operations | Existing; shared menus lose long action labels and enlarged text |
| Context menus and submenus | Fixed 28-point rows and one-line ellipsis conceal actions | P1 fix: text-measured wrapped action rows, `ui/widgets.rs` |
| Polls and interactive messages | Supported replies only, validation, incomplete-history disclosure | Existing: `ui/polls.rs`, `ui/dialogs.rs`; protocol unchanged |
| Contacts, groups, locations and labels | Forms, rights gating, permissions, local label semantics | Existing; shared bounded dialog treatment applies |
| Statuses | Read-only delivered statuses, expiry and no read receipt | Existing: `ui/statuses.rs`; no promise of older recovery |
| Media gallery and documents/links | Header/tab overflow, loading status, starting-download cancellation | P1/P2 fix: wrapped navigation, status, cancel and discoverable full filenames |
| Image/sticker previews and motion photos | Missing/download/progress states, navigation, save and clip playback | Existing: `ui/image_preview.rs`; live availability is account/device dependent |
| Inline and expanded video | Expanded picture/close targets lack accessible labels/focus reveal | P1 fix: label and focus for preview controls, `ui/video_preview.rs` |
| Voice playback and recording | Seek/speed, played receipts, next voice, explicit recording cancel/send | Existing: `audio.rs`, `ui/conversation.rs`; native microphone acceptance pending |
| Downloads and file management | Progress/limit/retry/cancel, save as, folder naming and account caches | Existing; gallery starting cancellation fixed; native filesystem acceptance pending |
| Settings navigation/search | Centered bounded column, back/search outside scroll, complex rows stacked | Existing prior fix; enlarged search field now measures text height |
| Appearance, custom themes, fonts and wallpaper | System/custom font, local palettes, theme variants, text scaling | Existing: `theme.rs`, Settings; palette contrast reviewed separately |
| Notifications and sound controls | Account settings, sender/content privacy and selected audio controls | Existing: Settings/notification tests; native receipt pending |
| Storage/cache and recovery progress | Stack progress, expose counts/errors, protect archive from cache deletion | Existing prior fix; no personal destructive cache test |
| Phone history recovery | Explicit/automatic request distinction, response is not completeness | Existing; second-device/older-history acceptance pending |
| Encrypted backup/export | Disabled save has no explanation for mismatching passphrases | P2 fix: validation text and wrapped actions, `ui/archive_backup.rs` |
| Restore and resume | Restore into separate offline account, explicit resume, passphrase privacy | Existing; backup/restore accepted previously, new UI needs attended read-back |
| Merge preview/cancel/commit | Copy incorrectly promised preservation of messages despite confirmed backup deletions | P1 fix: describe confirmed deletions truthfully in every catalog; native corrected-preview acceptance pending |
| Call preparation and cancel | Immediate admission, cancellable preparation, safe generation checks | Existing call-start fix; connected latency remains unmeasured |
| Incoming/ringing/active voice calls | Answer/refuse/mute/end, local outgoing tone and device recovery | Existing; user will perform willing-person voice/video tests |
| Camera/video and device selection | Missing programmatic selector names; video previews bypassed screen privacy | P1 fix: explicit names/full values, hidden frame painting and hover reveal; hardware and peer evidence pending |
| Call outcomes and history | Locally measured entries, timestamps/duration, replay deduplication | Existing; complete cross-device history is not promised |
| Calls dismissal | Outside/Escape dismiss history; nested picker first; never dismiss live call | Existing prior fix; attended read-back pending |
| Tray, hidden residency and reopen | Account-targeted notifications, private instance channel, preserve link | Existing: shell/tray tests; native reopen previously accepted |
| Privacy and app lock | Account switcher and call names bypassed screen privacy; locked actions/read receipts already suppressed | P1 fix: mask identities without changing account routing, named password feedback and bounded lock card; native acceptance pending |
| Login, sync choice, pairing and errors | Centered card could exceed short viewport; tagline did not wrap | P1 fix: bounded scroll and paragraph, `ui/login.rs`; fresh pairing stays pending |
| Dialogs and confirmations | Many fixed widths exceed narrow viewport; tall forms lack outer scroll | P1 fix: shared viewport-bounded width/scroll, `ui/dialogs.rs` |
| Loading/reconnecting/error banners | Retry could collapse into vertical letters in a narrow layout | P1 fix: wrapped status with a separate localized action row; no claim that feedback shortens a slow operation |
| Update download/verification/restart | Verified download, explicit restart, actionable failure | Existing: `ui/update.rs`; no release or real update initiated |
| Keyboard and accessibility | Deliberate Tab/F6 routing, modal focus, named painted controls | Shared switch reveal and download-name switch label fixed; native screen reader pending |
| Localization and long labels | Existing gettext and bidi; new strings must enter catalog | Fix new strings and exercise long/RTL fixtures; untranslated locales disclosed |
| Platform conformance | Native system fonts, file pickers, resident behavior and platform cfgs | Windows captures; Linux/macOS compilation/tests supplied by matching CI |

## Verification plan and boundaries

Use focused behavior/layout checks during implementation. Freeze source,
dependencies and packaging inputs before the final six Cargo gates. Preserve
matching cross-platform CI receipts, then build the default Windows client.
Perform one organized offline capture round, batch genuine findings, and at
most one visual confirmation. No screenshots of personal content or QR payloads.

Installation requires graceful shutdown, full hash-verified profile/cache
backup, preserved executable and an executable rollback; preserve OS keyring.
Attended acceptance records user observations separately from automated tests.
Second-account/device and other-platform hardware cases remain explicit pending
prerequisites. User has agreed to test voice and video with a willing person.

## Implementation follow-up

The concurrent checkpoint reconciliation landed as `49affc6a`; it changes
development evidence only. Its work and `packaging/__pycache__/` remain separate.

Further source findings addressed in the same coherent batch:

- Emoji/reaction pickers fit a small viewport and scroll their tab row.
- Confirmation action rows wrap, including offline-resume and locked-chat setup.
- Contact names stack in narrow forms; title Close controls retain their space.
- Shared primary, secondary and destructive buttons wrap exceptionally long labels.
- Label creation places colors below the name and gives each swatch a larger target.
- Notification dismissal retains room beside long titles; previews show up to three lines.
- Gallery row budgets follow text scale, with visible keyboard focus on thumbnails.
- Poll multiple-answer and readable-download-name switches expose labels/state.
- Lock Settings errors wrap; login/lock busy states retain their named status.

Focused tests first passed modal keyboard navigation. The new bounds regression
then exposed restore confirmation, contact form, poll and locked-chat action-row
overflow at 360x440 and 150% text. Fixes preserve actions and validation rather
than hiding controls.

The organized first capture round covered 99 synthetic views, including light,
dark/custom themes, narrow windows, 150% text, Portuguese and RTL fixtures.
Inspection found later controls clipped inside ordinary modal bodies, enlarged
emoji overlapping their cells, narrow Retry layout, and action-menu overflow.
Corrections use paired minimum/maximum scroll budgets, bounded emoji artwork,
stacked banner actions and shared scrollable menus. Normal-form tests now verify
that labels and final actions are painted without an initial scroll. The full
demo suite passed 287 tests after those corrections. Additional focused privacy
tests cover account labels, call identity/video and named device selections.
The sole confirmation round captured 32 views, with all exits successful and no
foreground takeover. All six contact sheets were inspected. Ordinary forms now
show their final controls; oversized forms/menu bodies scroll within the window;
emoji stay inside their cells; call video and account identities honor privacy.
The notification fixture is suppressed while Windows routes background cards
through its native overlay, so these captures do not prove its appearance.
`notification_overlay::tests::cards_recheck_current_privacy_and_expose_keyboard_controls`
provides synthetic render/keyboard/privacy coverage; native overlay acceptance is
still pending. There will be no additional cosmetic capture rounds.

319 source, asset, dependency, CI and packaging files were hashed before the
final six gates. Screenshots and logs are retained outside Git under
`H:/Scratch/Codex/zap-ux-20261013/`. Capture wall times include an artificial
1.2-second screenshot delay and are not application latency measurements.

All six required Windows Cargo gates passed on the frozen inputs: formatting,
default/all-feature clippy with warnings denied, default/all-feature all-target
tests, and all-feature rustdoc with warnings denied. Each library run passed
1,816 tests with 13 existing ignored tests; binary and update-flag integration
tests also passed. Missing upstream OpenSSL PDB files generated linker warnings,
without changing the successful exit results. All nine new UX regressions pass.
Cross-platform CI, default release build, protected installation and attended
acceptance remain pending at this source checkpoint.

## Installed build and attended failure

Default-feature `cargo build --locked --release` passed. Installed executable
SHA256 is `15e1d6ffb2d61ac2847c6ec3dfcf4afa01b05e4c3aca8b1421a15723388a40a0`.
The fresh cold backup at
`C:/Users/Victor/AppData/Local/ZapFasterRollback/20261013-113535`
verified 5,856 files (5,910,093,668 bytes). The exact executable rollback was
rehearsed before replacement; profile and Windows keyring were preserved.
An initial backup attempt stopped before replacement on the cache junction;
the reviewed target `D:/Downloads/ZapFast/cache` was copied and verified without
changing the junction. The user reopened the installed client (PID 36312).

The user then reported an outgoing voice call with one-way audio: the other
person hears the user, but the user hears nothing; calling through the phone
works. Only call-control evidence from the voluntarily supplied screenshot was
used. Its private chat contents are not copied into reports or visual evidence.

Allowlisted numeric log extraction for 14:39:57Z through 14:41:47Z shows 12
media samples with `rtp_received=0`, `decoded=0` and `delivered=0`. Local input
and output each opened once, with no output stalls or restarts. This places
the observed failure before local speaker playback. It does not establish the
particular relay, signaling or remote endpoint cause.

Protocol receive processing belongs to whatsapp-rust, pinned at `94c53e83`;
the client uses its normal call builder and audio source/sink interface.
Current upstream `0dc38af0` is three commits ahead, with no targeted VoIP fix.
The similar [upstream issue 1098](https://github.com/oxidezap/whatsapp-rust/issues/1098)
was closed in July, so it is a lead rather than proof of this failure's cause.
User scope excludes changing WhatsApp protocol behavior, and AGENTS.md says
not to vendor, fork or patch upstream crates here. No receive-path workaround
or speculative dependency upgrade has been applied. A targeted upstream
receive-path diagnosis/fix and a successful attended duplex call are specific
prerequisites. Calling is **not release-ready**. Incoming-direction, video,
hangup/history, timing and the other requested native observations remain pending.

## Functional call follow-up

The user reports that an incoming voice call works in both directions. It has
no local ringing sound, and the peer reports echo/poor audio while the user uses
speakers; a regular WhatsApp call does not have that echo. These observations
separate the outgoing receive failure from missing local call feedback and
microphone processing.

The implementation now adds an incoming two-chime ringtone using the existing
selected-output/recovery path. It stops before answer preparation opens the
microphone, and on refusal, cancellation or ending. Outgoing ringback waits for
the peer's actual Ringing phase, rather than implying ringing during Dialing.
Focused tone/lifecycle tests passed before the final follow-up gates.

`call_processing.rs` adds the unmodified Sonora 0.2.0 pure-Rust WebRTC processing
dependency, with AEC3, high-pass filtering and low noise suppression. Automatic
gain changes are disabled. Each call has its own processor shared only by its
native microphone and speaker threads. The reference is fed immediately before
actual speaker queueing, including outgoing ringback, rather than at network
receipt. The existing 16 kHz/60 ms transport framing and protocol API remain
unchanged. No audio is saved, logged or sent to a processing service.

Controlled fixtures measure 53.2 dB echo reduction with real transport frame
sizes, exceeding noise suppression alone by over 20 dB. An independent near-end
signal survives, silence remains silent, and invalid framing exposes a cleanup
failure until a device reset. The initial moderate noise setting failed the
near-end preservation check; low suppression corrected the implementation
without weakening that check. This is synthetic evidence, not hardware acceptance.
The standalone initial experiment processed a 20-second fixture in about 92 ms;
it does not measure call startup or end-to-end delay.

Cleanup failures retain unprocessed microphone audio and publish an explicit,
localized warning. End call appears above long diagnostics, with a narrow/large
text regression. Numeric capture/render cleanup counters support attended
diagnosis without exposing samples or identities. The dependency's license
notice is included in `THIRD-PARTY-NOTICES.md`.

321 affected source/asset/dependency/packaging/license inputs were frozen before
the follow-up's final checks. Static Flatpak manifest parity passed. The full
Flatpak Python test cannot execute its POSIX helper directly on Windows
(`WinError 193`); its Linux workflow must supply that coverage. All final Cargo,
matching CI, updated default build/installation and attended ringtone/echo
acceptance remain pending at this follow-up checkpoint. There are no further
cosmetic capture rounds. The outgoing zero-inbound-audio defect remains open.

The original UI commit `47a222d3` passed all nine jobs in CI `38412467273`,
including Linux/macOS native window smoke tests and the three Nix packages.
The call follow-up passed all six Windows gates, with 1,823 library tests and
13 existing ignored tests in each configuration, plus binary/integration tests.
Two subsequent functional corrections receive focused affected checks: a direct
call-render test now explicitly applies 150% text, and bookkeeping snapshots
without an actual offer/handle cannot open a ringtone device. Audio-health
warnings now clear only after new authenticated RTP, decoded frames and delivered
frames, rather than treating a codec choice as proof of recovery. The original
zero-reception failure is not hidden by this change.

The final corrections passed both default/all-feature clippy and both focused
call suites: 97 passed, two existing ignored in each configuration. Formatting
passed and no frozen application inputs changed afterward. These focused
receipts extend the successful full matrix rather than repeating unaffected
tests. Updated all-feature rustdoc, source push/CI, default release build and
the second protected installation are the remaining delivery steps.

## Attended voice acceptance and call-screen redesign

The call follow-up `99cc9ee3` was built and installed with a fresh verified
backup and rehearsed executable rollback at
`C:/Users/Victor/AppData/Local/ZapFasterRollback/20261013-141301`.
All 5,874 profile files (5,919,236,875 bytes) were read back by hash. The linked
session, archive, cache junction and Windows keyring were preserved. Installed
executable SHA-256 is
`397367601DC217F6A6AE1AD516CEAE26DD677F95D405A927078FE72A765906B1`.
The default build passed; its added microphone processing costs about 0.53 MiB
in executable size. All-feature rustdoc passed. Flatpak run `38435118735`
completed successfully for that exact source, covering the POSIX packaging
checks that cannot execute directly on Windows.

The user reopened this installation and confirmed an incoming **voice-only**
call worked, echo improved and ringing was audible. The subsequent answer
confirmed ringing stopped on answer; other controls were not tried. Video,
device selection, mute, hangup and completed-call history are not claimed
accepted. The outgoing zero-reception defect remains open.

The user explicitly requested a better call screen after this attended test.
This is a new, bounded call-surface design batch within the established native
theme system: centered identity and status, clearly labeled large microphone,
camera and hangup targets, a red hangup action, and an expandable device section.
Video uses one main preview with a smaller self-preview. Labels and icons form
one click and keyboard target; full labels wrap. Privacy hides photos/initials
and video frames while retaining all controls. Device choices and warnings stay
reachable without competing with primary call actions. Preparation, incoming
voice/video, connecting, active/muted, stalled audio, video requests and camera
opening are included in the scoped synthetic inspection. No additional protocol
behavior is changed. Verification and replacement of this redesign remain pending.

The scoped design workflow captured 24 synthetic call states/variants, fixed
the findings together, and captured 24 confirmation views. All captures exited
successfully without taking foreground focus. Inspection covered every sheet.
There are no further cosmetic capture rounds. The confirmation exposed a real
short-window overflow, investigated afterward through numerical layout tests.
Primary call controls now stay outside the scrolling identity/video/device body.
Very short windows use compact inline controls and place the two incoming-video
answer choices above Decline. The same compact layout applies to 400x440 windows,
preserving space for caller details at larger text sizes.

The layout regression verifies incoming and active video calls across five
window sizes (including 400x300 and 400x440), both light/dark themes and 100%/150%
text. It scrolls at fine increments and requires the complete microphone,
speaker and camera selector rectangles to become visible, rather than assuming
all fields can fit together. Identity/video privacy tests count the actual video
textures independently of control icons. A separate test activates mute with
Space, tabs to the camera control, activates it with Space, expands device
settings and clicks the hangup label. Synthetic views never open call devices.
The post-confirmation changes receive functional verification, without another
visual capture round. Native acceptance of the redesigned controls is pending.

The final 324 source/asset/dependency/packaging inputs are frozen with manifest
SHA-256 `66307aaab9f3e5afe81ae49f6b3a688cf207770f0af0d65d8e4910384e84c947`.
Both default/all-feature call suites pass 98 tests, with two existing hardware
tests ignored in each. Both clippy configurations, formatting and all-feature
rustdoc pass with warnings denied. The final default checks extend the matching
all-feature/rustdoc receipts after the short-window threshold correction.
Application inputs remain unchanged. Build, installation, matching CI and native
acceptance are the remaining delivery steps for this call-screen redesign.

The default Windows release build for `20c6d89f` passed in 8m22s and is installed.
Fresh backup `C:/Users/Victor/AppData/Local/ZapFasterRollback/20261013-153516`
contains 5,877 verified profile files (5,919,429,525 bytes); executable rollback
was rehearsed before replacement. Installed SHA-256 is
`5B32B6735B0F82F89BD72C3CB8613EAD03EECAAB0E1880E4FF47C1077E58DD04`.
The UI redesign added 7,680 bytes to the previous audio build. Version remains
0.30.0; no release was tagged or published. The user has been asked to reopen
the usual shortcut and report actual new-control/video observations.

CI `38435118695` for the audio follow-up has eight successful jobs and a
cancelled macOS Nix package job. Its targeted retry resolved the winit fetch
failure, then hit the existing 90-minute job timeout. The application derivation
started at 17:52:32Z; cancellation occurred at 19:15:11Z, without builder output
between them. This proves a timeout, not its underlying compilation cause.
The final delivery changes only macOS Nix's budget to 120 minutes, retaining
90 minutes on Linux, and enables live builder logs on all three package jobs.
No check or build target is removed or weakened. This CI-only change follows
the frozen application checks; application/asset/dependency inputs and the
installed binary remain unchanged. Matching final CI is still required.
