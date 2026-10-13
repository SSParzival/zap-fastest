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
