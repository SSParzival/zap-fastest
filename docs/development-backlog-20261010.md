# Development checklist, 2026-10-10

Full authorized scope: 66 open upstream PRs, 87 open issues, and the archive/download work below. Covered means capability exists in audited source, not attended device proof. Every unchecked item remains requested scope, including investigations and external dependencies. Competing proposals are consolidated in implementation while preserving individual tracking. Never inspect real message contents or user archives for diagnosis.

**Current continuation:** the full corrected batch now passes all six required
source gates on Windows x86_64, macOS ARM64 and Linux x86_64/ARM64, plus Windows
ARM64 all-target/all-feature cross-compilation. Nine refreshed synthetic images
were reviewed, and the cache controls and Status explanation layout defects were
corrected. Exact current hashes, receipts and restored resource limits are in
[the active batch checkpoint](development-batch-20261011.md). Attended acceptance
and documented upstream prerequisites remain open. No commit, push, installation
or release of this batch has occurred. Older package, Nix and visual receipts
below describe their earlier source snapshots.
The earlier integration snapshot passed all six required gates on Windows x86_64, Linux x86_64/ARM64 and macOS ARM64, plus macOS Settings-menu integration and Windows ARM64 all-target/all-feature cross-compilation. Final AMD64 and ARM64 native packages pass twelve clean distro installation/removal cases. Independent synthetic X11 acceptance passes 27 captures. Actual Nix release build, full tests, installation and output checks also pass. Original WSL settings and owned-container limits are restored, with unrelated running services verified again. [The verification checkpoint](development-verification-20261011.md), the final verification sections and development-resume-20261010.md supersede earlier per-item statements that local Rust gates are pending. Unchecked items retain their explicit platform, attended or upstream acceptance requirements.

## Archive and download checklist

- [ ] P1: Per-account pre-pairing Recent / Full history choice, persisted preference, honest duration, network, disk and availability warning; preserve existing sessions. Implemented in account settings and linking UI; local synthetic gates pass. Attended pairing validation remains.
- [ ] P1: Supported full pairing profile and permissive history admission; fixtures verify properties, attended synthetic-account pairing verifies actual behavior. Implemented against the pinned official protocol revision with its documented 365-day initial profile; phone availability still limits actual history.
- [ ] P1: Fair durable all-chat recovery, restart-safe progress, configurable sustained budgets, retry/backoff, no-progress and gap repair; preserve locks, deletion barriers, read state, viewport and poll coordination. Durable scheduler and account-local progress pass restart/fairness fixtures, including the measured 12-chat budget distribution. Attended phone tests remain.
- [ ] P1: Durable account-local background media jobs for live/history, carousel and motion attachments; bounded concurrency, restart/resume, integrity, cancel and expired-media reupload. Unavailable files differ from pending. Implementation in archive/media_jobs.rs and backend/worker/background_media.rs passes local Rust gates (two jobs, persisted leases and backoff, deletion/lock/account gates, cache destination); attended phone/media validation remains. Expired motion clips request a re-upload when their original message identity is retained; older clips without that identity remain an explicit recovery limitation.
- [ ] P1: Deliberate protocol upgrade with encrypted compressed-history capture before acknowledgement, crash-safe replay and deletion/idempotency checks. Official revision 94c53e833805c58c9f5966b08d08c990650a651e and encrypted capture/replay pass crash/deletion/idempotency fixtures and local gates. Attended phone interoperability remains.
- [ ] P1: Storage and recovery progress with counts, oldest recovered date, pending/failed media and phone availability; never claim complete from phone percentage alone. Counts and managed storage pass synthetic gates and the background-history capture. Attended phone availability/progress validation remains.
- [ ] P1: Organized future user downloads, account/chat/date folders, original or timestamp names; Unicode, reserved Windows names, collision, traversal and path-length defenses. Keep managed media identity separate; never silently move existing files. backend/worker/download_organization.rs and persisted process settings pass local gates; attended client downloads remain.
- [ ] P1: Settings for download root, organization, filename policy, media scope and storage budget; archival retention must not silently become disposable cache. Recovery scope/budgets and download organization settings pass local fixtures; archival files are retained rather than silently evicted. Attended client acceptance remains.
- [ ] P1: Encrypted backup/restore covering archive key, identity, SQLite side files and media; quiescent snapshot, password protection, integrity, overwrite preview and rollback. Full account package and quiescent export are implemented. Restore refuses existing accounts and opens a new account offline, with explicit resume and ordinary failure rollback. No overwrite flow is offered. Durable transaction ownership and startup reconciliation pass interrupted state/cache/key publication fixtures and local gates. Cross-platform power-loss and attended acceptance remain.
- [ ] P2: Local synthetic measurements and UI fixtures completed; attended sync and screen-reader acceptance remain. Windows 100-chat/10,000-message archive ingestion improved from 22.65s to 9.53/9.11s after measured atomic-write optimization, with 18.4 MB fixture peak working set. Three final native ARM64 demo runs give a 0.73s median capture-and-exit startup proxy and 176.35 MiB median application peak RSS (fresh app directories, warm OS cache, normal debug profile, software X11). The passed durable scheduler fixture allocates 60 requests across 12 chats, exactly five each, ratio 1.0, and preserves the budget after restoration. These measure synthetic ingestion, startup/RSS and virtual-clock fairness, not phone sync latency or old Intel hardware. Synthetic layouts, keyboard/focus and AccessKit labels pass their fixtures; individual native screen-reader delivery still needs attended verification. Detailed numeric receipts: H:/Scratch/Codex/zap-arm64-startup-measurements-20261011/REPORT.md.

## Product boundaries and dependencies

Calling remains requested, but adopting the submitted protocol fork conflicts with AGENTS.md. Required protocol work must land in oxidezap/whatsapp-rust first. PR #315 introduces seven messaging networks and is excluded by the user's explicit WhatsApp-only instruction. Calling and optional appearance work remain included. CLI sending requires explicit opt-in and private account/lock-aware IPC. View-once and disappearing media must retain WhatsApp privacy behavior. No missing capability is excluded merely for being absent.

## PR inventory

- [x] **P3, covered: [PR #476](https://github.com/crmne/zapfast/pull/476), Group phone numbers by their own country code (#449).** Covered independently in 0.30.0: phonenumber parser, full-digit guard, Brazilian formatting.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [PR #475](https://github.com/crmne/zapfast/pull/475), Show the account offline while the window sits in the tray (#461).** PresencePolicy::Manual is configured in start_bot; the account owns online presence with serialized acknowledgements, retries and connection generations. Source no longer relies on automatic library online announcements. Phone-visible online state while in the tray still needs attended verification.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [PR #474](https://github.com/crmne/zapfast/pull/474), Focus the existing window on niri instead of reopening it.** Adapted in src/window.rs: bounded, off-thread niri IPC focuses the existing window on its workspace; failure retains the Wayland reopen fallback. Synthetic focus-selection fixtures exist. Actual niri compositor focus remains unverified.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [PR #473](https://github.com/crmne/zapfast/pull/473), fix: keep a voice message's time on its duration line.** Adapted in src/ui/conversation.rs: voice_player returns duration-row space for the message time and ticks when they fit; longer download failures retain a separate footer. Local layout fixtures and gates pass; attended playback acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [PR #472](https://github.com/crmne/zapfast/pull/472), feat(macos): Open the announced chat when a macOS notification is clicked.** Adapted in src/notify/macos.rs using cancellable Notification Center handles, retaining AccountId and app lock, native sounds, read cancellation and release limits. Signed bundled macOS attended verification remains required.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [PR #471](https://github.com/crmne/zapfast/pull/471), feat: let a phone camera open Linked devices from the pairing QR.** PairingQrCode now uses the https://wa.me/settings/linked_devices# prefix without logging the QR payload. A synthetic worker fixture checks the exact prefix. Native phone-camera pairing remains unverified.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [PR #470](https://github.com/crmne/zapfast/pull/470), Leave the chat list's title out when it does not fit.** The chat-list header measures remaining width and hides Chats when necessary, retaining this fork's Starred messages control. Local narrow-layout fixtures and gates pass; attended client verification remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #469](https://github.com/crmne/zapfast/pull/469), feat: allow sending image files and clipboard images as documents.** Covered by d9f0bb5 / 0.30.0: original image documents and lossless pasted PNG.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #468](https://github.com/crmne/zapfast/pull/468), Fetch the picker's chat stickers a few at a time, without re-uploads.** Covered/adapted. 6b6f6c4, sticker pacing and no picker-only reuploads
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #467](https://github.com/crmne/zapfast/pull/467), Skip the window attention request on GNOME.** Covered/adapted. 5b63f7a, GNOME attention suppression
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #465](https://github.com/crmne/zapfast/pull/465), Play videos at 1.5x and 2x, like voice messages.** Covered/adapted. fa3f0aa, video speed
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #463](https://github.com/crmne/zapfast/pull/463), Move to whatsapp-rust ee89f5ad so expired downloads stop failing sends.** Covered/adapted. a684d1c, protocol ee89f5ad
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #462](https://github.com/crmne/zapfast/pull/462), Play the sound of WhatsApp's videos.** Covered/adapted. 46b29be, AAC video audio
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #458](https://github.com/crmne/zapfast/pull/458), fix(ui): hide read ticks when account receipts are off.** Covered/adapted. 8def7ab, disabled-read-receipt handling
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #455](https://github.com/crmne/zapfast/pull/455), Open WhatsApp chat links in ZapFast.** Covered/adapted. 11d1219 and dcc81c4, account-scoped chat links
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #453](https://github.com/crmne/zapfast/pull/453), Image crop rotate.** Covered/adapted. d5b72f7, crop/rotate
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #451](https://github.com/crmne/zapfast/pull/451), Add Screen privacy: hide chats from onlookers until hovered.** Covered/adapted. 352fc7f, screen privacy
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #442](https://github.com/crmne/zapfast/pull/442), feat: convert typed emoticons in composer.** Covered/adapted. e00e562, typed emoticons
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #441](https://github.com/crmne/zapfast/pull/441), feat: choose emoji rendering style.** Covered/adapted. d452076, emoji renderer
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #440](https://github.com/crmne/zapfast/pull/440), Log privacy-safe app-state recovery outcomes by collection.** Covered/adapted. a014690, safe app-state diagnostics
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #438](https://github.com/crmne/zapfast/pull/438), Step decorative animations while a screen reader is attached.** Covered/adapted. 1806deb, screen-reader animation stepping
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #436](https://github.com/crmne/zapfast/pull/436), Add on-demand image navigation to the image viewer.** Covered/adapted. d336543 and cd74e55, image navigation/gallery
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #435](https://github.com/crmne/zapfast/pull/435), Fit context submenus to their labels.** Covered by d9f0bb5: measured submenu widths bounded to viewport.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #424](https://github.com/crmne/zapfast/pull/424), feat: show the disappearing-message timer in chat info.** Covered/adapted. cc6f482, timer display
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #422](https://github.com/crmne/zapfast/pull/422), Show the chip row scroll bar below the chips instead of hiding it.** Covered/adapted. c3659df, chip scrollbar
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #420](https://github.com/crmne/zapfast/pull/420), Add a Quit ZapFast action to the Linux launcher.** Covered/adapted. 3cfcb4d, launcher quit
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, upstream dependency: [PR #418](https://github.com/crmne/zapfast/pull/418), Support @all mentions in group chats.** Required productivity capability remains pending. The audited official protocol revision exposes generated mention-everyone flags but no supported sender helper that establishes eligibility/admin thresholds and context semantics. Repository instructions prohibit encoding a protobuf flag here as a protocol implementation. Obtain a supported whatsapp-rust API upstream, then adapt localized aliases, permission checks, edits, captions, history and forward/quote clearing. Typing @all remains ordinary text, never advertised as notifying everyone.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #413](https://github.com/crmne/zapfast/pull/413), Keep text and attachment drafts at the top of the chat list.** Covered/adapted. 6e225cc, draft ordering
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #409](https://github.com/crmne/zapfast/pull/409), Copy selected messages with Cmd/Ctrl+C.** Covered/adapted. 8c38489, copy selection
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #406](https://github.com/crmne/zapfast/pull/406), Limit repeat desktop notifications per chat.** Covered/adapted. 6b758ac, notification throttling
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #400](https://github.com/crmne/zapfast/pull/400), Show a group created on the phone at once.** Covered/adapted. bdb4f84, phone-created groups
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, upstream dependency: [PR #395](https://github.com/crmne/zapfast/pull/395), Add group calling.** Upstream dependency verified against official 94c53e833805c58c9f5966b08d08c990650a651e: wacore/src/stanza/group_call.rs requires transaction-id and connected-limit in parse_group_info even for the initial ACK (lines 375-384); InitialGroupOfferParams carries one audio_rate (line 42). Submitted #395 depends on alitura1/whatsapp-rust d70d2d05 to alter both. The exact official high-level group facade exists but does not remove these interoperability defects. Our guide forbids adopting protocol forks: land required protocol fixes upstream first. Author reports working live 1:1 voice/video, but group-call creation still receives server error 427. Meaning of that error is unproven; do not advertise working group calls.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [PR #394](https://github.com/crmne/zapfast/pull/394), Localized recovered UI and synchronized catalogs.** Linking/history recovery, settings, archive progress, backup/restore consent and status, calls/devices, app lock and privacy/focus controls now use fresh gettext entries and plural summaries. Added previously omitted login/backup/call source modules to extraction. All ten catalogs contain 688 active translated entries with zero empty/fuzzy entries; GNU msgfmt format checks pass. Existing valid translations are preserved, and malformed legacy placeholders/plural declarations repaired. Windows Git Bash extraction resolves GNU gettext and Rust paths without changing the host. The final GNU source-reference extraction check passes. Local build/catalog gates pass; attended language/layout acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #391](https://github.com/crmne/zapfast/pull/391), Show group descriptions in group info.** Covered/adapted. 6c458c2, group descriptions
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #388](https://github.com/crmne/zapfast/pull/388), Add option to start without restoring the last chat.** Covered/adapted. fd8e95d, startup chat preference
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #366](https://github.com/crmne/zapfast/pull/366), Add an option to keep the chat list position after sending.** Covered/adapted. 0613145, list position
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #365](https://github.com/crmne/zapfast/pull/365), Distinguish audio files from recorded voice notes.** Covered/adapted. c24688f, audio-file distinction
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #364](https://github.com/crmne/zapfast/pull/364), Keep rate-limited messages waiting and send them when the limit ends.** Covered/adapted. dc178dc and 1aeffc4, durable queue adapted
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #359](https://github.com/crmne/zapfast/pull/359), Add private replies from group messages.** Covered/adapted. 484c1d9, private group replies
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #357](https://github.com/crmne/zapfast/pull/357), Open a chat at its end without flashing its oldest messages.** Covered/adapted. 3e012a8, open at newest message
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #340](https://github.com/crmne/zapfast/pull/340), Keep attachment drafts in their originating chat.** Covered/adapted. f4199c8, attachment draft ownership
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #337](https://github.com/crmne/zapfast/pull/337), Validate message edits and preserve originals on failed sends.** Covered/adapted. e83f4e7, edit validation/recovery
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #336](https://github.com/crmne/zapfast/pull/336), Select and confirm multiple forwarding recipients.** Covered/adapted. 03f2206, multiple forwarding recipients
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [PR #335](https://github.com/crmne/zapfast/pull/335), Choose a custom interface font in Settings.** Consolidated with #75 into one process-wide font import flow in src/custom_font.rs, src/theme.rs and Settings. Background selection validates and copies a font, retains the system/emoji fallback stack, and persists an account-independent choice. Multilingual metrics, clipping and all-platform attended verification remain required.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [PR #332](https://github.com/crmne/zapfast/pull/332), Add a Messages theme for macOS.** Messages appearance is adapted into current native components: optional palettes and bubble/run geometry, privacy covers, selection, pinned banners and albums, with a single reverse pass for run tails. Synthetic messages/messages-light demos and fixtures exist. Native materials, GPU performance and attended Windows/macOS/Linux contrast remain unverified.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #324](https://github.com/crmne/zapfast/pull/324), Add local voice message transcription with yapsnap.** Alternative already covered. Local transcription is already implemented with embedded Whisper in commit 7e8c547 and documented in our guide. This PR instead shells out to yapsnap and ffmpeg. Do not add a second engine; retain our account isolation, bounded work and lock/privacy behavior. PR build fails.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #321](https://github.com/crmne/zapfast/pull/321), Delete all selected messages from the context menu.** Covered/adapted. d8252d3, bulk delete
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #320](https://github.com/crmne/zapfast/pull/320), Start conversations with unsaved phone numbers.** Covered/adapted. 0fb020a and dcc81c4, unsaved numbers
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #319](https://github.com/crmne/zapfast/pull/319), Keep the visible message date while scrolling.** Covered/adapted. ba697e8, sticky date
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #318](https://github.com/crmne/zapfast/pull/318), Drag downloaded attachments to other Windows applications.** Covered/adapted. 5b06850, Windows file dragging
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #317](https://github.com/crmne/zapfast/pull/317), Stage clipboard images reliably and keep Enter in the composer.** Covered/adapted. b99e55f, clipboard staging
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, product decision: [PR #315](https://github.com/crmne/zapfast/pull/315), Hold Telegram, Matrix, Slack, Zulip, Discord, X, and Delta Chat accounts in one window.** User explicitly excluded non-WhatsApp networks. Keep WhatsApp multi-account support; do not adopt these adapters.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #301](https://github.com/crmne/zapfast/pull/301), Send a location, from coordinates or a map link.** Covered/adapted. 33de708, location sharing
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #286](https://github.com/crmne/zapfast/pull/286), Group WhatsApp media albums.** Covered/adapted. eedebf1, albums
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [PR #268](https://github.com/crmne/zapfast/pull/268), Show a clicked sticker on its own, with a favorite toggle.** Downloaded conversation stickers open a native viewer with phone-synced favorite controls. Source reconciles animation detection, simultaneous thumbnail/viewer draws, account switches, locks and deletion dismissal. Local synthetic fixtures and gates pass; attended favorite synchronization remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #264](https://github.com/crmne/zapfast/pull/264), Show chat info in a side panel, with its media, links and docs.** Covered/adapted. cd74e55, archive-backed gallery; the current dirty batch adds a resizable wide-window side panel and retains the narrow modal, with fixtures awaiting validation
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, partial, validation pending: [PR #256](https://github.com/crmne/zapfast/pull/256), Add call history and chat call entries.** Encrypted local call history and live call outcomes are implemented in archive/call_log.rs with account-scoped UI routing. Official CallLogSync/CallLogHistory ingestion, chronological entries, missed-call stubs and a narrow placeholder backfill are now implemented and covered by synthetic fixtures. Attended missed-call interoperability remains required; video/group dependencies are still tracked separately.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [PR #255](https://github.com/crmne/zapfast/pull/255), Add 1:1 video calling.** Native nokhwa capture uses Media Foundation on Windows, AVFoundation on macOS and V4L on Linux. User-initiated start/answer/upgrade uses the official video facade and request tokens, OpenH264 encodes locally at 15fps with a 640-pixel maximum side, retaining portrait/landscape aspect ratio, and the view keeps only latest local/peer pictures. An isolated own-executable helper is killed on stop/lock/cancel even if native capture blocks. Hardware-free encoder/pipe/preview fixtures are included. Attended camera permissions, camera removal, two-device interoperability and packaged platform behavior remain unverified. The current continuation adds persisted per-account camera selection with isolated discovery and refusal when the selected device disappears. New identity, portrait encoding and layout fixtures await whole-batch validation; earlier passing gates do not cover these changes.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #236](https://github.com/crmne/zapfast/pull/236), feat: star a message and list the starred ones in the left panel.** Covered by d9f0bb5: durable account-local stars and paged starred-message list.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #234](https://github.com/crmne/zapfast/pull/234), feat: pin a message in a chat for everyone.** Covered by d9f0bb5: pins, notices, cycling and 24h/7d/30d durations.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #232](https://github.com/crmne/zapfast/pull/232), feat: show how much space the downloaded attachments take in Settings.** Covered/adapted. b14f5f6, storage estimates
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [PR #220](https://github.com/crmne/zapfast/pull/220), Add 1:1 voice calling.** Native 1:1 voice adapted with official protocol voip feature, account/lock-aware routing, nonblocking handshake, cancellable generation guards, audio device preferences and local call history. Local fixture compilation and gates pass; attended two-account cross-platform audio/network validation remains required.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #207](https://github.com/crmne/zapfast/pull/207), Ask before removing a sticker pack.** Covered/adapted. 95231e6, sticker deletion confirmation
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [PR #194](https://github.com/crmne/zapfast/pull/194), feat: fetch older history and its files in the background.** The resumed source adds fair durable all-chat history recovery and account-local background media jobs, replacing the old top-ten limitation. Request budgets, persisted retry progress, locks/deletion protections, carousel and motion jobs and expired-media recovery are implemented. Local synthetic/restart fixtures and gates pass; attended phone acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [PR #189](https://github.com/crmne/zapfast/pull/189), feat: open photos and videos in a full-window viewer with the chat's album.** Covered/adapted. cd74e55, full-window media viewer adapted
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, consolidated implementation, validation pending: [PR #75](https://github.com/crmne/zapfast/pull/75), feat: custom fonts.** Consolidated into the #335 custom font import and system-reset flow. No second competing font configuration was added. Local font fixtures and gates pass; attended multilingual/platform acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

## Issue inventory

- [x] **P3, covered: [Issue #466](https://github.com/crmne/zapfast/issues/466), On GNOME, every notified message also shows “ZapFast is ready”.** Covered in source. GNOME duplicate attention requests addressed by #467 / 5b63f7a; still verify on actual GNOME desktop.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #464](https://github.com/crmne/zapfast/issues/464), Play videos at 1.5x and 2x, like voice messages.** Covered in source. 1x/1.5x/2x video playback addressed by #465; The current dirty batch also provides 1.25x/1.75x through direct speed selection, with its fixtures awaiting validation.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [Issue #461](https://github.com/crmne/zapfast/issues/461), Shows online everytime even when minimised.** The #475 adaptation selects manual protocol presence while preserving acknowledgements, retries and generations. Minimized/tray online status still needs phone-visible attended proof.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #460](https://github.com/crmne/zapfast/issues/460), Add a feature to change ctrl W behavior.** Process-wide Settings, System preference lets Ctrl+W close the visible chat; its default still closes the window on every platform. macOS Command+W and the native menu use the same resolver. No visible chat, overlays or app lock make the optional close-chat shortcut a no-op. Synthetic key/default/settings fixtures and local gates pass; attended desktop acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #459](https://github.com/crmne/zapfast/issues/459), per-contact custom profile picture (local-only override).** Contact information imports/reset account-owned display overrides, separate from fetched cache and uploaded profile photos. Content-addressed images and hashed contact index persist across restart and full account backups. Synthetic import/reset/account isolation/source-and-cache preservation/path-rejection fixtures and local gates pass; attended image-picker acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, upstream API required: [Issue #457](https://github.com/crmne/zapfast/issues/457), Option to hide menu bar item on OS X.** Pinned fastframe-tray v0.4.1 exposes menu-entry set_visible and set_icon, but no status-item hide/show or native item handle. Its private macOS ITEM thread-local owns the NSStatusItem and Dock reopen router. App-side private runtime discovery or crate patching would violate supported API reuse/no upstream patch boundary. Required upstream addition: status-item visibility API with truthful is_shown plus retained Dock/reopen handling; then add the app preference without stranding resident windows. Full requested behavior remains pending.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #456](https://github.com/crmne/zapfast/issues/456), Edit Messages.** Covered in 0.30.0 source: edit caret at Unicode end.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #454](https://github.com/crmne/zapfast/issues/454), visual refresh (as an experimental Appearance option) + a few features I'd like to help implement.** Missed-call history, displayed-text search highlighting, semantic document thumbnails and typed last-message media icons are implemented. Optional Messages appearance uses the existing native views. Synthetic layout, copy and navigation checks are in integration verification; attended accessibility remains pending.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #452](https://github.com/crmne/zapfast/issues/452), Message send fails with "IQ request failed: client is not connected" during reconnect window, no retry.** Covered in source. Durable per-account sends wait through reconnect/rate limits. In-flight uncertainty remains Unconfirmed, never blind resend. Already in v0.21.0 source/release notes.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #450](https://github.com/crmne/zapfast/issues/450), Non-compact Reactions.** Reaction rows now show at most three distinct chips (two in narrow space), counts and an overflow/details control. The local results dialog retains every reactor, respects name privacy and locked chats, and allows removing our own reaction. Synthetic grouping/dialog-privacy fixtures and local compilation pass; attended reaction-dialog acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #449](https://github.com/crmne/zapfast/issues/449), Phone numbers are formatted using the account's country code instead of the contact's country code.** Covered in 0.30.0 source: contact-specific international formatting.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, upstream dependency: [Issue #447](https://github.com/crmne/zapfast/issues/447), macOS: text an input method inserts without marked text is dropped; OpenVanilla associated phrases type "!" instead.** Upstream input investigation. macOS OpenVanilla commit without marked text. Handle in winit/egui upstream, not a vendored patch in this repo. Validate the exact installed/shared revision first.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #446](https://github.com/crmne/zapfast/issues/446), Zombie (defunct) children from clicked links: webbrowser background spawn never reaped (#132 follow-up).** Covered in 0.30.0 source: browser launcher child reaping.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, locally verified, release publication pending: [Issue #445](https://github.com/crmne/zapfast/issues/445), Linux: Embed AppImage update information.** Linux finalization writes passive stable-release update metadata into the pinned runtime reserved ELF section before checksums. Separate zsync sidecars/checksums are generated by packaging CI. Synthetic AMD64/ARM64 fixtures preserve offsets/filesystem and reject malformed/conflicting inputs; actual pinned AMD64 runtime metadata verified. Built-in signed updater trust is unchanged. Final local AMD64/ARM64 AppImage assembly, metadata, version and checksum checks pass; release publication remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #444](https://github.com/crmne/zapfast/issues/444), Chat filter chips are clipped in narrow sidebar.** Covered core, verify layout. Horizontal chip scrolling/visible scrollbar already addressed by #422. Verify at narrow widths, high scaling and long translated labels before calling every clipping variant fixed.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, upstream dependency: [Issue #443](https://github.com/crmne/zapfast/issues/443), Include 1x visualization option for photos.** New privacy feature, not image zoom. The issue means sending view-once photos, not 1:1 pixel visualization. Duplicate of #327. Our incoming phone-only placeholders do not implement sending. Verify a library API and never archive view-once media as ordinary persistent content.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #439](https://github.com/crmne/zapfast/issues/439), 💔 Add different Emoji libraries.** Covered core, optional expansion. System/Noto renderer choice exists via #441. The current dirty batch also imports user-supplied CBDT/sbix color fonts, with restart and format limitations. Branded bundled assets still require redistribution rights.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, partial: [Issue #437](https://github.com/crmne/zapfast/issues/437), Archive state stays stale on one linked Windows client after snapshot recovery failures.** Partial: durable local archive changes fixed; snapshot omission convergence still needs upstream protocol fix.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #434](https://github.com/crmne/zapfast/issues/434), Better accessibility support for ZapFast.** F6 and Shift+F6 navigate registered sections; Shift+F10 opens the focused message action menu. List/media semantics preserve lock and screen-privacy boundaries, while ordinary Tab navigation remains intact. Synthetic keyboard fixtures are in integration verification. NVDA and VoiceOver acceptance still require those screen readers.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, needs evidence: [Issue #433](https://github.com/crmne/zapfast/issues/433), Screen flickers while using the app.** Investigate, platform reproduction. Windows flicker/high-refresh complaint. Group with #223; reproduce a specific GPU/monitor/VRR/HDR configuration. Event-driven repaint intentionally does not run at monitor refresh while idle.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, upstream dependency: [Issue #432](https://github.com/crmne/zapfast/issues/432), Is there support for Alt codes?.** Upstream input investigation. Windows Alt-code input, also discussion #431. Reproduce numpad/Alt handling in winit/egui and keep shortcuts from swallowing committed text.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #429](https://github.com/crmne/zapfast/issues/429), 0.19.0: --start-hidden still opens the window at login (shown is false before spawn() returns).** Covered in 0.30.0 source: delayed tray startup with fallback; attended verification remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #425](https://github.com/crmne/zapfast/issues/425), Touch input misbehaves.** Single-finger gestures scroll the transcript without mouse text selection or attachment dragging. Explicit message selection remains available through taps; cancellation restores mouse behavior. Local touch and sidebar-resize regression fixtures pass; attended touch/compositor acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [Issue #421](https://github.com/crmne/zapfast/issues/421), Wrong audio output.** Audio-device settings now select the output for voice and video playback as well as calls. Active playback follows preference/default-route changes, preserves paused voice state, and reports missing or failed selected outputs when falling back to the system default. No idle device polling. Physical Bluetooth/HDMI routing and platform acceptance remain attended checks.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, layout verification pending: [Issue #412](https://github.com/crmne/zapfast/issues/412), Add app-wide font size options.** Persisted independent text size (85 to 150 percent) scales custom and built-in fonts within the window draw, resets account layout caches and preserves UI zoom. Finite bounds and scoped font helpers pass synthetic fixtures. Enlarged narrow call layouts pass the 32-case fixture and final X11 captures; native selected-font/RTL ink acceptance remains separately tracked.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, attended verification pending: [Issue #411](https://github.com/crmne/zapfast/issues/411), Add upload/download progress bar/circle.** Uploads use the official streaming encryption/upload API and report encrypted bytes consumed by the HTTP upload source, with token/account cancellation and normal delivery states. This is byte progress, not server acceptance. Download byte progress remains covered; synthetic transfer fixtures exist, actual network transfers pending.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #410](https://github.com/crmne/zapfast/issues/410), Chat gallery.** Covered in source. Archive-backed per-chat media/documents/links gallery in cd74e55; released v0.21.0.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #408](https://github.com/crmne/zapfast/issues/408), Show the whole number in group chat.** Covered in 0.30.0 source: complete sender digits.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #405](https://github.com/crmne/zapfast/issues/405), Opening the sticker picker still hits 429 rate-overlimit on 0.19.0, so sending a sticker fails (follow-up to #298).** Covered in source. Protocol ee89f5ad coalesces media-connection refresh and removes expired-download refresh storms; #468 pacing also integrated. Confirm real send behavior on affected accounts only with consent.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, needs evidence: [Issue #404](https://github.com/crmne/zapfast/issues/404), [Bug]: Cannot attach files using drag-and-drop or Ctrl+V on Windows.** Existing paths, runtime unresolved. Windows dropped-file and clipboard file-list handling already exists. Do not confuse Explorer file lists with image pixels or video clipboard formats. Reproduce installer/elevation/window drag behavior before redesigning clipboard.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, needs evidence: [Issue #398](https://github.com/crmne/zapfast/issues/398), Hebrew fonts doesnt look right.** Font-stack investigation. Hebrew metrics/fallback quality. Check current system font and shared fastframe-fonts scaling; not fixed merely by adding a custom font picker.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, native ownership evidence pending: [Issue #397](https://github.com/crmne/zapfast/issues/397), GNOME rectangular outer border.** Source confirms native decorations remain enabled; titlebar hiding is macOS-only. Wayland winit chooses an Adwaita/fallback client frame or drops it for negotiated server decoration. No public native corner-radius override exists; disabling all decorations also removes native controls/resizing and is not an equivalent fix. Determine Wayland/X11/XWayland and frame/shadow/content ownership on a synthetic GNOME window, then fix the responsible shared stack without dependency patches. See [native integration evidence](native-integration-evidence.md).
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P2, verified locally: [Issue #396](https://github.com/crmne/zapfast/issues/396), Linux icon SVG uses a filter that Qt rejects.** The shipped renamed SVG contains no unsupported filter. Qt 5 QSvgRenderer accepted it without warnings and produced a detailed, nonempty image. A packaging check now exercises that renderer. Desktop shell icon caching remains a separate installation behavior.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, needs evidence: [Issue #392](https://github.com/crmne/zapfast/issues/392), [Bug]: Notifications disappear immediately and do not persist in KDE Plasma system tray history.** Core fix present, verify KDE. DesktopEntry now names zap-faster or Flatpak ID. Notifications include chat category and desktop mapping. Verify persistence on KDE; our fork-specific IDs mean upstream AUR reports are not direct proof.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #389](https://github.com/crmne/zapfast/issues/389), Show group descriptions in group info.** Covered in source. Group descriptions via #391.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #387](https://github.com/crmne/zapfast/issues/387), Allow starting ZapFast without restoring the last open chat.** Covered in source. Restore-last-chat preference via #388.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #386](https://github.com/crmne/zapfast/issues/386), [Bug] Videos with more than 64mb can't be played.** Covered download path, decoder bounded. Manual downloads up to 2 GiB implemented; automatic cap 64 MiB and bounded preview/decoder limits remain. A successful large download does not imply every codec/large video plays inline.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, partial: [Issue #383](https://github.com/crmne/zapfast/issues/383), Only sends messages to phone numbers, people using account names do not receive messages.** Partial: recipient-device refresh for known pre-send failure plus attachment restoration; live recipient delivery not proven.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [Issue #382](https://github.com/crmne/zapfast/issues/382), Microphone recording negotiation and errors.** Replaced the pinned rodio iterator path with public CPAL input streams, negotiated device formats, bounded callback queues, explicit permission/disconnection/overflow/stall errors and cancellation checks. Recording uses the selected microphone and refuses an unavailable selection. Synthetic fixtures cover cancellation, failure without partial sends, stereo resampling and 50 ms levels. Native permission, device removal and recording acceptance remain attended checks; backend initialization can still wait on the OS.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #381](https://github.com/crmne/zapfast/issues/381), Portable settings.** Separate exact opt-in marker routes only process settings.json beside the executable. Standard paths retain account roster/state/cache/keyring/runtime/themes. Existing updater portable marker alone changes no data location. Synthetic marker/default/isolation fixture added; writable executable-folder and attended acceptance pending.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, local verification passed: [Issue #380](https://github.com/crmne/zapfast/issues/380), Remember last-open list on startup.** Last chat filter and label persist per account, including older/unknown-settings compatibility. Switching accounts preserves independent choice, and deleted/unknown labels are pruned. Synthetic settings round-trip, account-local list persistence and deleted-label fixtures pass on all native gate configurations. Attended account startup remains unverified.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, local verification passed: [Issue #379](https://github.com/crmne/zapfast/issues/379), Assign notification sound per List/Label.** Per-label sounds follow label-manager order for overlaps. Chat overrides win, urgent mentions retain mention sounds, mute/privacy gates remain ahead of delivery. Account-owned settings and label-deletion pruning fixtures pass local gates; attended notification-sound delivery remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #378](https://github.com/crmne/zapfast/issues/378), Align small number of messages to the bottom.** Short conversations use measured leading space, corrected in a discarded layout pass before presentation. Row heights exclude that space, and long conversations keep virtualization, selection, unread divider and explicit message jumps. The resize/short-to-long/jump fixture passes local gates, and final synthetic conversation captures pass. Native client acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, attended tray verification pending: [Issue #377](https://github.com/crmne/zapfast/issues/377), Change tray icon color on user-defined VIP chats.** Optional VIP chats produce a high-contrast unread tray indicator aggregated across accounts. Archived/muted/locked chats and screen/app privacy cannot reveal VIP activity. Pixel fixtures cover16through64-pixel icons; actual host trays pending.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #376](https://github.com/crmne/zapfast/issues/376), Option to hide notification content (Windows).** Covered in 0.30.0 source: account-specific sender/content notification masking.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, attended tray verification pending: [Issue #375](https://github.com/crmne/zapfast/issues/375), Shown number on tray icon (Windows).** Optional tray digits show aggregate unread chats with 99+ overflow, distinct from taskbar/Dock badges. App/screen privacy clears indicators; Linux uses pixel icons so themed-icon preference cannot hide changing badges. Synthetic pixel bounds verified; actual tray delivery pending.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #373](https://github.com/crmne/zapfast/issues/373), A reaction should send the chat to the top.** Newer nonempty live reactions advance chat activity using persisted per-reactor timestamp watermarks. History, offline/PDO delivery, removals and stale replays cannot promote chats. Reaction content and activity commit atomically. Synthetic replay/rollback fixtures exist; phone ordering remains unverified.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, upstream dependency: [Issue #371](https://github.com/crmne/zapfast/issues/371), Support/import Whatsapp-native custom lists.** Library capability investigation. Native WhatsApp lists are not local labels/favorites. Verify pinned library events/snapshots/writes and fix missing protocol integration upstream before implementing sync.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #368](https://github.com/crmne/zapfast/issues/368), Show red circle with number of notifications in Dock icon.** Covered in 0.30.0 source: macOS Dock unread-chat count; attended verification remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [Issue #363](https://github.com/crmne/zapfast/issues/363), Add an option to use real filenames for automatic downloads.** Future opted-in background attachments and motion clips queue separate organized copies under the explicitly selected export folder. Managed archive identities remain unchanged. Durable intents, current privacy/deletion checks, collision-safe publication and restart retries are implemented. Local durable export/privacy/deletion/retry fixtures and gates pass; attended client export acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #362](https://github.com/crmne/zapfast/issues/362), If I send a photo using the file option, it sends them as a photo resulting in compression and no additional photo options..** Covered in 0.30.0 source: original images as documents.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #361](https://github.com/crmne/zapfast/issues/361), Audio files render as voice notes, hiding the audio type.** Covered in source. Audio files distinguished from recorded voice notes via #365.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #351](https://github.com/crmne/zapfast/issues/351), Put a sender's picture beside the last message of their run, as WhatsApp on iOS does.** The optional Messages appearance places the avatar beside the final message in a sender run, including bare photos. Sender and day changes end the run. The existing WhatsApp appearance remains the default. Synthetic run-boundary fixtures and final local layout gates pass; attended native appearance acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, partial, attended evidence pending: [Issue #346](https://github.com/crmne/zapfast/issues/346), No red bubble notification.** Native macOS Dock unread badge and Notification Center delivery/click routing are in source. The original Dock badge request and later banners-missing-while-sounds-play reports need separate attended verification; source does not prove notification permissions or delivery.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #331](https://github.com/crmne/zapfast/issues/331), A Messages theme for macOS.** The #332 Messages appearance adaptation is in source with synthetic demo modes. Native material behavior, low-end GPU measurement and actual desktop contrast/privacy acceptance remain pending.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, needs evidence: [Issue #328](https://github.com/crmne/zapfast/issues/328), Bug Report: Arabic font clipping issue in text input & search fields.** High-value multilingual fix. Arabic text clipping in fields and previews. Reproduce shared font ascent/baseline/layout metrics; fix upstream fonts where appropriate.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, upstream dependency: [Issue #327](https://github.com/crmne/zapfast/issues/327), Send view once media.** Later privacy feature. View-once sending, same as #443. Pinned media builder options omit view-once controls. Official send tests accept inline flags, but wiring generated flags directly conflicts with the repository protocol boundary. Upstream typed sender option and retention-safe preparation are required; incoming placeholders already supported.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, attended sync verification pending: [Issue #326](https://github.com/crmne/zapfast/issues/326), Multi-select messages options.** Existing multi-select copy/delete/forward now includes bulk Star and Unstar through normal supported SetStar commands. Local, revoked, missing and stale-chat selections are refused; synthetic command fixtures verify this. Phone sync pending.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #325](https://github.com/crmne/zapfast/issues/325), Continuous "Your phone did not send older messages. Check that it is online" message pop up.** Covered core, verify. Explicit-only older-history warning once per chat until answer/reconnect already implemented. Confirm requested old-version behavior against v0.21.0.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, supported file/image path implemented, native-format evidence pending: [Issue #323](https://github.com/crmne/zapfast/issues/323), Windows Snipping Tool video clipboard.** Current paste prefers arboard file lists (CF_HDROP), including existing video files, then images. Arboard does not expose OLE virtual FileDescriptor/FileContents streams. Microsoft documents those supported Windows APIs, but the reported Snipping Tool format has not been established with a synthetic attended capture. Need format names/IDs and medium type only before a bounded private staging adapter or upstream arboard integration. This is not a product exclusion or proven protocol blocker. See [native integration evidence](native-integration-evidence.md).
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #316](https://github.com/crmne/zapfast/issues/316), Allow manual downloads of attachments larger than 64 MiB.** Covered in source. Explicit up-to-2-GiB attachment downloads implemented, with progress/cancellation/integrity and bounded concurrency.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, source mitigation, reproduction pending: [Issue #313](https://github.com/crmne/zapfast/issues/313), App Crash.** The narrow group rename field now guarantees positive inner text width, eliminating a source-evidenced invalid-size path. The historical Windows crash was not reproduced; do not claim its cause or complete resolution from this mitigation.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #312](https://github.com/crmne/zapfast/issues/312), Middle click autoscroll for Linux.** Linux middle-click autoscroll is bounded to the originating account, chat and transcript gutter, with a dead zone and capped speed. Keyboard, mouse, touch, focus and account changes cancel it. Synthetic checks exist; compositor acceptance remains pending.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, verified locally, attended browser handoff pending: [Issue #311](https://github.com/crmne/zapfast/issues/311), Clicking on links doesn't open them.** The existing real-bubble fixture clicks a painted synthetic URL and observes exactly one OpenUrl, while selection padding does not activate links. It passed on the integrated Windows test binary. Opening the actual platform browser remains an attended check; the historical report itself was already resolved by its reporter on 0.18.2.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, layout verification pending: [Issue #309](https://github.com/crmne/zapfast/issues/309), Connection banner reflows the whole window; scope it to the sidebar.** Connection/history/update status lives inside the sidebar, with compact rail and narrow detail fallback. It preserves the wide conversation viewport and macOS traffic-light strip; local native gates and background-history captures pass; attended narrow-status/native client acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #308](https://github.com/crmne/zapfast/issues/308), Arrow-key navigation between images in a chat from the image viewer.** Covered core. Gallery/viewer navigation exists. Keyboard semantics intentionally include comma/full stop and focus traversal; arrow shortcut differences should be treated as polish.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #296](https://github.com/crmne/zapfast/issues/296), CLI access to WhatsApp.** Explicit per-account opt-in enables bounded read/watch/send/status on a separate authenticated fastframe-instance slot, never legacy port. App/chat locks and disappearing/phone-only privacy restrict export. Stable send ids are durably reserved before normal send, duplicate payloads return existing status and Unconfirmed is preserved without unsafe retry. Restore disables this host authority. Synthetic permit/privacy/transport/delivery fixtures and local gates pass; docs/local-automation.md describes the opt-in interface. Attended local transport/read/watch/send acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, notification verification pending: [Issue #295](https://github.com/crmne/zapfast/issues/295), Add a focus/waiting for mode which filters the available chat to chats inside a certain kind of list.** Optional label focus mode filters chats and notifications per account, permits urgent group mentions when chosen, and offers an explicit exit. Locked chats remain hidden; stale labels disable focus safely. Synthetic account/label/lock fixtures pass local gates; attended notification filtering remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [Issue #292](https://github.com/crmne/zapfast/issues/292), Incorrect read receipt.** Zero historical receipt timestamps remain unknown. Durable per-message/per-recipient receipts, group minimum aggregation and receipts arriving before their message have synthetic coverage; account read-privacy suppresses read ticks without suppressing delivered/played state. Local receipt suites and gates pass; attended phone-visible receipt acceptance remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #291](https://github.com/crmne/zapfast/issues/291), Dragging images from whatsapp chats to desktop/explorer directly.** Covered Windows scope. Drag downloaded attachments to other applications via #318. Finder/Linux drag-out parity is separate work.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, partial implementation, attended evidence pending: [Issue #271](https://github.com/crmne/zapfast/issues/271), Windows startup network failures.** Version fetch uses the official custom ureq HTTP client, separate from WebSocket Happy Eyeballs. Added a supported 15-second HTTP connect budget so ureq distributes time across resolved addresses instead of waiting unbounded on a blackholed first address. Configuration fixture retains dual-stack and certificate verification. Non-timeout IPv6 failure continuation remains a ureq integration issue; corporate TLS requires sanitized stage/trust/revocation evidence, not profile deletion or disabled validation. Supported platform trust API exists but is not enabled speculatively. See [native integration evidence](native-integration-evidence.md).
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #265](https://github.com/crmne/zapfast/issues/265), Video Playback without sound.** Covered in source. WhatsApp AAC sound via custom MP4 track decoding, #462. Unsupported codecs remain system-player fallback.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, upstream dependency: [Issue #262](https://github.com/crmne/zapfast/issues/262), accent menu does not appear when holding down key.** Upstream macOS input work. Press-and-hold accent picker, separate from #447. Needs native text-input integration in winit/egui and attended OS tests.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #261](https://github.com/crmne/zapfast/issues/261), Steam Deck KDE archive keyring.** Pinned zbus-secret-service-keyring-store documentation explicitly requires UTF-8 encoding for KDE Wallet binary secrets. New Linux keys use versioned hexadecimal credentials; legacy raw keys remain readable and existing credentials are preserved. Copy/restore ownership checks compare decoded key bytes. Errors explain the Secret Service provider/session/unlock requirements and existing Try again retries archive opening. Synthetic legacy/text/malformed-key fixtures added; Steam Deck/KWallet attended acceptance remains pending. No disposable or plaintext fallback.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [Issue #252](https://github.com/crmne/zapfast/issues/252), Data Backup and Restore.** Encrypted full account backup/restore is in archive/backup.rs: independent archive/key snapshot, linked-device identity and SQLite side files, account settings and media, password authentication and integrity checks. Export pauses the account and fences device-store release. Restore creates a separate account offline, verifies a new keyring key and rolls back ordinary failures; existing destinations are refused rather than overwritten. Explicit resume confirmation protects against duplicate linked-device connections. Durable transaction ownership, key fingerprints, non-replacing publication, roster commit and startup reconciliation now cover interrupted publication, with synthetic crash-boundary/conflict fixtures. Local backup/restore and crash-boundary fixtures pass all native gates; cross-platform power-loss behavior and attended restore acceptance remain.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [Issue #248](https://github.com/crmne/zapfast/issues/248), Bidirectional text support.** Mixed-direction paragraphs now align independently; wrapped rows inherit their full paragraph direction. Added logical link/selection, emoji, decoration and repeated-layout fixtures await whole-batch validation. Native Arabic/Hebrew font and ink acceptance remain open.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #247](https://github.com/crmne/zapfast/issues/247), Image grouping.** Covered core, bulk actions optional. Media albums (#286) and viewer/gallery implemented. Save-all/album bulk actions are additional requests.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #244](https://github.com/crmne/zapfast/issues/244), Support WhatsApp community structure.** Supported group metadata now persists account-local community parent/member roles and announcement/general subgroup markers. Group info shows known visible parent and child links; unavailable/locked groups are not exposed. Community creation, administration, and full enumeration remain upstream/API or attended-validation gaps.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, needs evidence: [Issue #223](https://github.com/crmne/zapfast/issues/223), Screen flickers black.** Investigate with #433. Windows black flicker persists in older reports; need current build reproduction, GPU/driver/multi-monitor evidence and shared egui fix where appropriate.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, partial, validation pending: [Issue #219](https://github.com/crmne/zapfast/issues/219), Add 1:1 voice and video calling.** Native 1:1 voice/video and local call history are implemented with the official protocol facade, including camera helpers and supported call-entry ingestion; no live calls were placed. Windows/Linux/macOS device and two-phone acceptance remain required. Group interoperability depends on upstream protocol fixes.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, attended validation pending: [Issue #211](https://github.com/crmne/zapfast/issues/211), In MacOS, the app doesn't respect Do Not Disturb. Notification sounds keep playing..** Notification sounds use native macOS delivery. Actual Focus/Do Not Disturb suppression and signed bundled notification behavior remain unverified.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, measured, older hardware reproduction pending: [Issue #209](https://github.com/crmne/zapfast/issues/209), performance drop.** Repeatable synthetic encrypted archive benchmark added:100chats/10,000messages. Windows debug ingestion22.65s before and9.53s after atomic per-message writes; same9,662,464-byte database. Paging/search/reopen under100 ms. Repeat after measured9.11s with18.4MB peakworking set. Actual old Intel/Linux startup report still needs matching hardware evidence.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, partial, validation pending: [Issue #204](https://github.com/crmne/zapfast/issues/204), Media cache grows unbounded.** Background media recovery now has per-account managed-storage accounting and a configurable budget that pauses new background downloads without deleting archived files. Explicit/manual downloads remain independent. Synthetic accounting/budget fixtures pass local gates. The current continuation adds optional startup/daily age cleanup of unused cache files only, preserving every archived attachment, durable recovery job and export under the user-selected policy. New safety fixtures await whole-batch validation; attended storage-policy acceptance remains. Referenced-attachment expiry is outside the chosen scope.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, locally verified, release publication pending: [Issue #202](https://github.com/crmne/zapfast/issues/202), Debian 12: .deb requires glibc 2.39 and libasound2t64.** Linux release builds use actual Ubuntu 22.04 containers with glibc 2.35, matching declared Debian 12-compatible libc/ALSA names. Finalization rejects binaries requiring newer GLIBC, so dependencies cannot be loosened on old incompatible assets. Both final binaries have maximum GLIBC 2.35. All twelve clean AMD64/ARM64 installation/removal cases pass, including Ubuntu 22.04 and Debian 12, with exact runtime dependencies. Updated public release assets remain unpublished.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P1, implemented, validation pending: [Issue #191](https://github.com/crmne/zapfast/issues/191), Disappearing messages: show, change, and expire the timer.** Supported direct/group timer editing now acknowledges official library methods before changing local state. Message-specific expiry tracks authenticated history/live deadlines and timestamp-ordered keep/undo decisions, waits for sync, cancels matching transfers, clears account-managed cached attachments, and leaves replay barriers. Existing rows with no message-specific expiry observation are preserved; historical absence is never replaced by the current chat timer. Attended timer/keep synchronization remains unverified.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [x] **P3, covered: [Issue #187](https://github.com/crmne/zapfast/issues/187), Add support for pinned msgs in chats.** Covered in 0.30.0 source: message pins.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

- [ ] **P2, implemented, validation pending: [Issue #163](https://github.com/crmne/zapfast/issues/163), Option for custom in-app notifications (like Telegram Desktop)?.** Optional account-local notification cards share the native notification privacy, mute and throttle checks, then recheck privacy when drawn. Cards retain the originating account, expire with a scheduled wake and expose keyboard controls. Defaults remain off; local privacy/keyboard fixtures and gates pass. Attended card delivery remains.
  - Acceptance: Focused synthetic regression, relevant project gates, affected platform/device verification where needed.

## Delivery gates

- [x] Frozen-source Windows formatting, both strict Clippy modes, both locked all-target test modes and strict all-feature rustdoc pass. Library tests: 1,722 passed, 13 manual/hardware ignored in each mode; executable/update suites pass too.
- [x] Windows, Linux and macOS compilation plus attended limitations are recorded.
- [x] Synthetic offline client layout, keyboard/focus, AccessKit-label and settings fixtures pass, with 27 final X11 captures. Native screen-reader delivery and platform/device behavior still require attended acceptance.
- [x] Account isolation, hidden-account events, locks, deletion barriers and Unconfirmed sends remain protected by passing synthetic fixtures.
- [x] Every item has implementation, consolidated evidence, or explicit external/product dependency.
- [x] Delivery inventory read-back confirms all 66 PRs and 87 issues, without duplicates: 75 marked covered and 78 unchecked, plus the ten unchecked archive/download acceptance items.
- [x] Official dependency reassessment against current refs and exact source is recorded in [upstream capabilities](upstream-capabilities-20261011.md); no newly supported capability was found for the listed blockers.
- [x] Tested product integration and packaging committed directly to local main, with exact input/receipt read-back and durable evidence. See [delivery review](development-delivery-20261011.md).
- [ ] Execute [attended acceptance](development-attended-acceptance-20261011.md) on approved synthetic accounts and native devices; no case has run in this continuation.
- [x] Source push explicitly approved and public-main read-back confirms the three reviewed commits at `595b31f5a0a82fe30c0c73594e0f611c8e8b74fe`. Release publication and installed-client replacement remain separate approvals.

## Calling implementation evidence

1:1 voice uses the official oxidezap/whatsapp-rust revision with voip, native rodio input/output, bounded audio channels and real signaling/media state. Start and answer prepare on background tasks with timeouts, cancellation and per-account generation guards. Hidden accounts keep their own snapshots, notification targets carry AccountId, screen lock ends capture, and call log outcomes reside in the encrypted archive. No live calls were placed or answered during development.

Native 1:1 video is also implemented through nokhwa camera backends and the official video facade, including initial offers, explicit answers and token-bound upgrades. The own-executable camera helper uses private bounded H.264 pipes and is killed on cancellation or screen lock; no archive or account data enters it. Source and sink queues hold two frames, local/peer views retain only their newest picture, and OpenH264 encoding stays local at 15fps with a 640-pixel maximum side and preserved aspect ratio. Per-account camera selection and portrait encoding are new source changes awaiting batch validation. No camera device was opened during development.

Required remaining work: attended Windows/Linux/macOS camera permissions and removal, two-device voice/video acceptance, group upstream protocol fixes and documented refusal evidence, attended language and screen-reader acceptance. Catalog and synthetic accessibility checks are covered by the final verification below. Native platform APIs and source compilation are not attended hardware or interoperability proof. Selected-camera platform identity and decoder internal allocation limits remain practical limitations.

Primary sources: [official call facade](https://github.com/oxidezap/whatsapp-rust/blob/94c53e833805c58c9f5966b08d08c990650a651e/src/client/voip.rs), [official group stanza parser](https://github.com/oxidezap/whatsapp-rust/blob/94c53e833805c58c9f5966b08d08c990650a651e/wacore/src/stanza/group_call.rs), [upstream client camera implementation](https://github.com/alitura1/zapfast/blob/d65a079a7f595f37e711744dfc6a0bf4184af79b/src/camera.rs), [group PR live refusal report](https://github.com/crmne/zapfast/pull/395).

Call-entry ingestion: official `CallLogSync` and `CallLogHistory` events now retain raw metadata in the encrypted account archive and project known creator/direction/start-time/peer terminal outcomes into call history and chronological transcript entries. Explicit `CallLogMessage.durationSecs` displays seconds; raw `CallLogRecord.duration` remains unconverted because its units are unspecified. Named missed voice/video history stubs are displayed. A narrow versioned backfill upgrades existing unedited Unsupported(call) rows that retain CallLogMessage raw bodies. Unknown creator/direction/time, ambiguous outgoing participants, and ongoing/scheduled outcomes remain raw-only. No sends or incoming notifications are triggered by this ingestion; synthetic fixtures and local Windows gates pass, with attended interoperability remaining open. Group call history metadata is displayable, but group calling itself remains blocked by the documented upstream capability.

Remaining protocol dependencies verified against pinned whatsapp-rust94c53e8: outgoing view-once has no public ImageOptions/VideoOptions/AudioOptions option ([builders](https://github.com/oxidezap/whatsapp-rust/blob/94c53e833805c58c9f5966b08d08c990650a651e/src/media.rs)); generated-field send fixtures are not a supported builder API. Native chat custom lists have no typed list event/snapshot/CRUD facade in the pinned feature/event source; LabelSublistUpdate is explicitly inbound-only Business label metadata, and status-privacy custom_lists are audience lists, neither implements native chat lists. Community metadata supports read-only hierarchy roles through [Groups](https://github.com/oxidezap/whatsapp-rust/blob/94c53e833805c58c9f5966b08d08c990650a651e/src/features/groups.rs), while no community administration facade was found. No upstream messages or PRs were sent.


## Final verification and blockers

Final-source ARM64/x86 Linux and macOS required gates all pass. Both Linux architectures have 1,722 passing library tests per configuration and 14 ignored; macOS has 1,715 passing library tests and 13 ignored. Default executable tests: nine passed; all-feature executable tests: eleven passed; update tests: two passed in both modes. The native macOS Settings-menu integration also passes. All six Windows gates pass with 1,722 passing library tests per configuration and 13 ignored. Its ARM64 all-target/all-feature cross-compilation also passes. Detailed logs and recovery commands are in [the checkpoint](development-resume-20261010.md).

Synthetic suites cover history/media recovery, account/lock isolation, durable backup publication and reconciliation, CLI permit revocation and Unconfirmed send handling, deletion/export barriers, mouse/touch/keyboard ownership, actual body URL activation, calls without opening devices, enlarged/narrow layouts, logical text/emoji copy and localized controls. They do not establish phone availability, hardware behavior, screen-reader acceptance or installed-client proof. GNU source-reference extraction and format checks pass for all ten catalogs, with 688 translated entries and no empty/fuzzy entries each. Exact native-packages 0.8.1 validation and three-target dry runs, actionlint, synthetic ELF finalization tests, exact AMD64/ARM64 runtime metadata inspection and Linux Qt SVG rendering pass; these are not actual package installation results.

Docker/WSL recovery succeeded under Victor's explicit restart authorization, preserving inaccessible runtime directories and retaining data disks, images, volumes and caches. Both final Linux binaries pass compilation, ABI inspection, DEB/RPM/AppImage assembly/execution and twelve clean distro installation/removal cases. Independent final synthetic X11 acceptance passes 18 ARM64 and nine x86 captures. Actual Nix flake preflight, release build/full tests, installation and installed-output checks all pass. All runners retain actual per-command exit receipts; dispatch success alone is never accepted as a completed check. Original WSL memory/swap settings are restored byte-for-byte; original owned-container limits and all 15 unrelated containers running before the restart are verified restored. Owned build containers are stopped.

Outstanding upstream dependencies include group-call interoperability, supported @all and view-once sender APIs, native chat custom-list CRUD/snapshots and shared native input/tray integration. Attended synthetic-account/device acceptance remains required for the explicitly tracked phone, calling, permissions, notification, compositor, font/input and GPU reports. The whole backlog is not declared complete or release-ready. No release, push, installed-client replacement or live-account operation occurred.

The 2026-10-11 delivery continuation commits the tested integration (`d98487d`)
and packaging (`17868ea`) locally, preserves [durable receipts](verification/development-receipts-20261011.json),
and prepares exact attended cases. Documentation-only corrections match existing
audio routing and completed-media audit behavior. The [dependency reassessment](upstream-capabilities-20261011.md)
and [delivery review](development-delivery-20261011.md) supersede earlier statements
that the source bundle remains wholly uncommitted. Implementation/local checks,
attended proof and public delivery remain distinct.
