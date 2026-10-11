# Development resume checkpoint, 2026-10-10

The original chat was **01a12758-7287-74d2-89b8-2079851d3a4d**, titled
**Review ZapFast and history sync**. A computer restart interrupted its running
implementation. The chat projection in `thread_history_1.sqlite` retained the
conversation; the associated rollout file was zero bytes. The recovered
conversation, surviving source and current compiler output establish this
checkpoint. It is a work record, not a release or completion claim.

## Recovered task contract

The four original user messages authorized the following:

1. Audit new and unmerged upstream PRs and issues, and investigate the connection
   and protocol backend to maximize background history and media recovery as a
   WhatsApp second brain.
2. Include everything found, with a Full versus Recent history choice before
   first linking/opening, and an honest warning that Full can take a long time.
3. Implement all identified PRs, issues, improvements, quick wins and
   optimizations; persist the Full/Recent choice; organize user Downloads by
   account, chat and timestamp with readable filenames and storage settings.
   The user granted creative implementation freedom and explicitly authorized
   subagents.
4. Keep the product WhatsApp-only. Calling and optional appearance work remain
   included; adapters for other messaging networks are excluded.

The full contract remains active. This is not an MVP or a first phase substituted
for the requested work. The [development checklist](development-backlog-20261010.md)
retains all 66 audited PRs, 87 issues and the archive/download checklist, including
unchecked investigations, platform acceptance and upstream dependencies.
Unrelated dirty work must be preserved. This authorization does not itself
publish a release, send external messages, install a new client or act on a live
WhatsApp account.

## Surviving implementation and resumed corrections

Modified and new source survived across account/app/model/settings/backend,
archive storage and encryption, history recovery, UI, notifications, window and
appearance modules. Cargo.toml and Cargo.lock also retained the deliberate
upgrade to official oxidezap/whatsapp-rust revision
`94c53e833805c58c9f5966b08d08c990650a651e`.

The surviving source includes Recent/Full pairing settings, fair durable all-chat
history recovery, encrypted pre-acknowledgement compressed-history capture and
replay, persistent background media jobs, progress/storage counts, managed
storage budgets and organized future downloads. New modules include
`archive/history_capture.rs`, `archive/media_jobs.rs`, `archive/storage.rs`,
`backend/worker/background_media.rs` and
`backend/worker/download_organization.rs`. Automatic media recovery retains the
64 MiB limit; explicit downloads retain their separate supported limit. Expired
motion clips can request re-upload when their original message identity exists;
older clips without that identity have an explicit recovery limitation.

The batch also includes native 1:1 voice calls and encrypted local call history
(`calls.rs`, `call_audio.rs`, `backend/worker/calls.rs`, `archive/call_log.rs`),
macOS notification/Dock integration, niri existing-window focus, pairing-camera
QR prefix, narrow chat-list headers, voice duration-row footers and a sticker
viewer. Optional Messages appearance and a consolidated #335/#75 custom font
import flow are present. Source coverage is recorded per item in the checklist;
these additions have not all passed current tests or attended acceptance.

Resumed work repaired compile errors and strengthened backup/restore. Full
account backups now include an independent encrypted archive/key snapshot,
linked-device identity and SQLite side files, account settings, wallpaper,
stickers, caches and referenced external media. Export pauses the account and
requires the device-store release fence. Restore validates the package and
SQLite integrity, binds a verified keyring key to a newly allocated account,
preserves preferences and initially opens offline. Settings offers explicit
resume confirmation, warning that the original installation must be closed and
that pending operations can send after reconnecting. Existing account folders
are refused untouched. Ordinary installation/registration failures clean up the
new destination and credential. State/cache/key publication now has a durable ownership transaction and startup
reconciliation before workers: unregistered installs roll back their own data,
registered installs finish marker cleanup, and conflicts fail closed. Synthetic
boundary coverage is added; platform power-loss behavior remains unverified.

No real chat content, contact rows, archive message bodies, device credentials or
QR payloads were inspected for this recovery. Fixtures must remain synthetic.
No live calls or resumed restored sessions were exercised.

## Where execution stopped and current verification

The original interruption left a compiler run reporting **11 errors**, with
source edits intact. The recovered chat contract and those edits were resumed;
no reset, release, installation, live call or real-user archive inspection was
performed.

The final local gate sequence completed successfully on Windows after the
`synced_call_log` logout-cleanup regression was corrected (2026-10-10,
18:48 local time). All six commands below exited zero. Default tests passed
1,659 library, eight executable and two updater tests. All-feature tests passed
1,659 library, ten executable and two updater tests, **1,671 total**. Twelve
hardware/manual tests remained ignored. Synthetic fixtures exercise history
replay, deletion barriers, account isolation, background cancellation, backup
round trips, explicit restored-session confirmation, call/video surfaces and
camera framing without opening devices or connecting real accounts.

Captured logs are `H:\Scratch\Codex\zap-resume-gate-{NAME}.log` and the
machine-readable outcome record is `H:\Scratch\Codex\zap-resume-gates.json`.
The bundled OpenSSL objects emitted MSVC LNK4099 debug-symbol warnings during
linking; these did not fail the test commands. Strict Clippy and rustdoc checks
passed without suppressing warnings or weakening hooks.

- [x] `cargo fmt --all --check`
- [x] `cargo clippy --locked --all-targets -- -D warnings`
- [x] `cargo clippy --locked --all-targets --all-features -- -D warnings`
- [x] `cargo test --locked --all-targets`
- [x] `cargo test --locked --all-targets --all-features`
- [x] `RUSTDOCFLAGS='-D warnings' cargo doc --locked --all-features --no-deps`
- [ ] Synthetic offline demos verify new layouts/settings and accessibility from
      the actual client, including Messages light/dark, narrow controls,
      sticker viewer and restored-session confirmation.
- [ ] Platform compilation and attended Windows/Linux/macOS limits are recorded,
      including audio devices, phone-camera pairing, niri focus and signed macOS
      notifications/Focus behavior.

For PowerShell, set `$env:RUSTDOCFLAGS='-D warnings'` before the documented Cargo
rustdoc command. Preserve output/errors/exit status with hidden, captured
processes. Do not weaken lints or hooks to make these gates pass.

## Remaining scope and protocol constraints

Every unchecked checklist entry remains requested scope. The local source gates
above pass. The synthetic suite covers supported replay/deletion barriers,
scheduler fairness/restart, download path/account isolation and backup/restore
round trips. Completing the broader task still requires actual-client and
platform evidence, representative process-crash/rollback validation and measured
startup/sync/memory/recovery performance. Actual
phone history/media availability and cross-device synchronization need attended
synthetic-account proof; phone progress percentage alone cannot prove completion.

Native 1:1 voice/video and encrypted local call outcomes are implemented. Video
uses nokhwa native Windows/macOS/Linux backends and the official library facade,
with explicit camera consent, initial video offers, answers and token-bound
upgrades. A private own-executable helper encodes bounded 640x480/15fps H.264;
stop/lock/cancel terminates that helper instead of leaving a blocked capture
thread inside the application.
The UI retains only latest local/peer previews. No real camera or call was used.
Official call-history events, chronological entries, missed-call stubs and narrow
placeholder backfill are implemented with passing synthetic fixtures. Remaining
work includes attended platform permissions and camera removal, two-device
voice/video and missed-call acceptance, group interoperability and
localization/accessibility polish. The default camera is used; camera selection
is not exposed. Native API availability is not hardware interoperability proof.

Group calling must use official whatsapp-rust. The audited official group parser
requires transaction-id and connected-limit on the initial response, and the
initial offer has one audio_rate. Submitted #395 depends on a protocol fork to
change those behaviors and reports server refusal 427 for group creation. Its
meaning is unproven. Required protocol fixes must land upstream; do not vendor,
patch or adopt a protocol fork here, or advertise working groups from the mere
presence of the official facade.

Mention-everyone (#418) is also still requested but depends on upstream support.
Generated ContextInfo fields and experiment flags do not establish a supported
sender helper or eligibility/admin semantics. Do not wire a protobuf bit directly
in this client to claim @all support. A supported upstream helper must define
current group eligibility and edit/quote/forward semantics before localized
composer suggestions and delivery claims can be enabled. Current @all text is
ordinary text. See [the support requirement](mention-everyone-support.md).

The remaining inventory also preserves investigations and requested features
such as CLI opt-in/private account IPC, focus/waiting filters, communities,
disappearing-message handling, platform input/rendering issues, packaging
compatibility and broader localization. Source fixes with unreproduced reports
remain mitigations or unverified evidence, not confirmed resolutions. The only
explicit network exclusion is the non-WhatsApp adapter proposal #315.

Continue from the current dirty tree, reconcile each implemented item with
verification, and stop only when the authorized scope and required gates are
satisfied or an explicit upstream/product dependency is concretely recorded.

## Next continuation

Continue from this saved dirty checkout. The six local gates above are verified;
do not rerun them without a source change or new evidence. Reconcile the broader
checklist rather than declaring the whole original request complete.

Known remaining archive behaviors include terminal unavailable jobs not
requeueing automatically when later history supplies a missing motion identity,
and completed jobs not auditing managed files deleted outside the app. Explicit
retry is separate. These are recovery gaps, not permanent product exclusions.
Startup/sync/memory performance remains unmeasured against a representative
synthetic archive. Restored state/cache publication has ordinary failure rollback
and now uses durable ownership transactions with restart reconciliation across
state/cache/key publication. Cross-platform compilation, power-loss behavior and attended
phone/camera/audio/notification behavior remain unverified.

## Continued recovery corrections

The earlier terminal-job and missing-managed-file gaps are now implemented in
`src/archive/media_jobs.rs` and `src/backend/worker/background_media.rs`.
Unavailable motion jobs retry when a later delivery supplies a changed original
child identity or raw clip, only for the parent photo's sender. Changed ordinary
attachment raw bytes can retry unavailable jobs too. Identical history and
explicit user cancellations preserve queue state. Downloads always resolve the
current archive raw source rather than retaining stale job source bytes.

Completed-job auditing processes at most 32 eligible rows per tick, persists its
cursor, and waits five minutes after a complete round. Confirmed missing managed
files atomically lose their stale path and requeue after rechecking lock, left-chat,
individual deletion, pending deletion and whole-chat removal barriers. Files
outside the account's managed directory, symbolic/reparse links, unexpected
directories and unreadable paths are preserved. The worker gates auditing on
background recovery, connection, privacy readiness, lock pause and completed
logout cleanup. It does not promise to recover a file the phone cannot supply.

New synthetic fixtures cover refreshed and duplicate motion identity, sender
ownership, cancellation, changed ordinary raw, bounded audit/restart cursor,
missing-path snapshot validation, lock/deletion barriers and managed/external
filesystem probes. No user archive content was inspected. Root-owned Cargo gates
must validate this source snapshot before these fixtures are called passed.
## Conversation continuation

Issue #450 now bounds distinct reaction chips to three (two at narrow widths),
retains per-emoji counts and original reaction data, and opens a local results
dialog from overflow or the details control. The dialog lists every reactor,
hides private names, refuses locked-chat details outside their folder and offers
our own reaction removal. Grouping preserves source order and uses a lookup table
instead of rescanning distinct emoji for every reactor.

Issue #378 now measures transcript body height independently of leading space,
bottom-aligns short transcripts and requests at most one corrected layout pass
before presentation. Long transcripts retain row virtualization and existing
scroll, unread, jump and selection behavior. Synthetic fixtures cover reaction
group preservation, result privacy/locked states, short-chat resize, growth into a
long transcript and explicit jump resolution. These are source changes awaiting
root-owned test acceptance, not live-device or screenshot proof. Impeccable
context read PRODUCT.md and scoped native layout/craft guidance; the bounded
mechanical layout detector returned no findings. Existing dark/light committed
screenshots informed styling, but do not validate the new rendered snapshot.

## Restore, portable preferences and local pictures continuation

The next implementation batch adds restart reconciliation for encrypted restore.
A synced journal under state/accounts records the exact generated account id,
state/cache staging names, random ownership token and new archive-key fingerprint
before copying account data or publishing a key. Publication refuses competing
destinations atomically. Account-roster save uses a synced temporary file and
platform publication barrier. On restart, reconciliation runs before account
workers: registered installs complete marker cleanup; unregistered installs
remove only validated token-owned folders and their matching key. A corrupt
record, unowned destination, changed key or inaccessible keyring preserves data
and stops startup. Owned abandoned decrypted preparation folders are removed too.
No overwrite flow or automatic deletion of unrelated destinations is introduced.

Synthetic fixtures cover interruption after journal, credential, cache and state
publication; retained registered accounts; unowned conflicting destinations;
changed tokens/traversal; and abandoned preparation cleanup. Key helper tests use
mock credentials. Filesystem/keyring persistence through a real power loss,
cross-platform compilation and attended restore remain unverified. These changes
require fresh gates and are not covered by the earlier successful gate set.

Issue #381 now has an explicit preferences-only opt-in: the executable-adjacent
zap-faster-portable-settings.txt containing zap-faster-portable-settings-v1 routes
only process settings.json beside the executable. Ordinary and existing updater
portable markers leave standard paths unchanged. Accounts, archives, keyring,
roster, runtime, caches and themes retain their standard location. A synthetic
marker/default/isolation fixture verifies that distinction; installation-folder
write permissions and attended portability still need acceptance.

Issue #459 adds local contact-picture import/reset in contact information. The
account-owned avatar-overrides directory stores a hashed contact index and
content-addressed scaled PNG images. Separate account display maps prefer them
without changing the fetched WhatsApp cache or uploading another person's photo.
Worker import/reset events update only their originating account, and app lock
drops picker/reset actions. Full-account backups include this state directory.
Synthetic tests cover persisted imports, reset, original/cache preservation,
account isolation, invalid paths, avatar precedence, hidden-account events and
locked actions. Native picker acceptance remains unverified.

### Touch, Linux middle-click, and attachment rendering checkpoint

Touch transcript targets now leave dragging to the enclosing touch scroll area:
text does not enter egui label selection during touch, row selection and Windows
native attachment drags do not capture touch pans, and touch cancellation releases
the mouse selection leash. Explicit message selection still permits tap targets.
The synthetic egui fixture drives touch start/move/cancel together with their
pointer events, checks an actual scroll offset change, checks no text selection,
and checks that mouse drag sensing returns. Native long-press menus use the
existing platform secondary-click delivery; attended touchscreen acceptance is
still required.

Linux middle-click starts autoscroll only over empty conversation space, keeping
middle paste over message text, media and the composer. State belongs to the
account and chat, has a dead zone and bounded speed, and ends on wheel/touch,
other pointer presses, Escape, focus loss, pointer departure, or a gap in drawing
the conversation. The synthetic pointer fixture verifies restricted starts,
movement, account/chat replacement and pointer departure. Linux attended input
and rendering still require validation.

Document attachments show an already archived thumbnail in their existing icon
slot and expose the document name and open/download action to accessibility.
Chat accessibility names obey screen privacy, including compact rows. The
remaining accessibility work includes F6 section navigation, complete action-menu
keyboard coverage and NVDA/VoiceOver acceptance. Search-term highlighting and
remaining chat media-preview icons are not complete. These are implementation
checkpoints, pending the root agent's Cargo gates and synthetic visual review.


## CLI and macOS menu-bar continuation

Issue #296 now has bounded opt-in local read/watch/send/status. AccountSettings
cli_access defaults false and is cleared on restored installs. A separate
runtime/automation-v1 fastframe-instance slot uses the supported Slot::claim/send
String request/reply API, authenticated independently of the old fixed port.
App account/lock authorization precedes worker dispatch; pending replies are
scrubbed when locked/cancelled. Worker archive reads refuse locked/disappearing
conversations and filter phone-only/message-specific expiry rows. Read pages
can move backward; watch polls local archive arrival order with explicit cursor
for a bounded duration and refuses deleted cursors. It never fetches phone
history or sends read receipts. Replies cap at 12 KiB and pages at 25 rows.

CLI text sends reserve a stable request id, digest, chat and SDK-generated
message id inside the encrypted archive before enqueueing the existing normal
send path. A duplicate request returns the prior identity/status rather than
sending again; changed content is refused. Missing reserved message rows remain
Unconfirmed after interruption. Client transport timeouts explicitly instruct
querying the same id, not creating a new send. Existing typed pre-transmission
rate-limit retries remain supported; ambiguous acknowledgements and transport
failures do not automatically retry. Synthetic tests cover request bounds,
redacted debug, lock/cancellation scrub, preference defaults/roundtrip, durable
idempotency, worker privacy filtering and CLI parsing. See local-automation.md
for exact usage and limits. Live archive reads/sends were not run by tools.
This latest code needs new root gates and attended private IPC validation.

Issue #457 remains an explicit upstream API dependency, not excluded scope.
The pinned fastframe-tray v0.4.1 (23d87e0) exposes set_visible for menu entries
and set_icon for an existing status item. The actual macOS NSStatusItem and Dock
reopen router live in a private ITEM/REOPEN thread-local in the crate, with no
supported hide/show/native-handle API. Native status-item discovery through
private runtime details is unsuitable and patching the crate conflicts with
AGENTS.md. Required upstream work: expose item visibility with truthful is_shown
and preserve the Dock/application reopen route while hidden, then integrate the
requested per-platform preference and resident-window tests.


### CLI privacy and acceptance review

A concrete post-dispatch race was corrected: the original global lock boolean
could not revoke queued work after account opt-out/chat lock and could revive a
stale reply after lock then unlock. Requests now carry monotonic authorization
permits. App actions, backend events, immediate account settings toggles and app
lock synchronize a weak permit registry. Revocation never re-authorizes the old
request. Worker and final private transport handler recheck it; worker validates
the explicit account id too. A shared fence orders send acceptance against
revocation. A send already accepted may continue, while revoked-before-acceptance
requests cannot reserve/store/enqueue. Existing request-id lookup binds account
through its independent archive and verifies chat plus any repeated text digest;
status with no text cannot rebind or dispatch a send.

Synthetic coverage now includes actual app authorization/owning-account dispatch,
opt-out/local/remote-lock delayed replies, lock/unlock buffered reply withholding,
misrouted requests and revoked reads. An official SDK Bot is built with a fixture
SQLite session and never run/connected; the existing outgoing synthetic hook
intercepts transport. This fixture verifies SDK-generated id reservation,
archived outgoing text, one queue acceptance, duplicate suppression and typed
uncertain completion retained as Unconfirmed without retry. Live WhatsApp server
acceptance remains attended verification. CliWatchPage and tracked-permit aliases
address narrow clippy complexity findings. This review changes no catalog strings
and requires root verification of the latest source.

### Notification, accessibility, archive export, and reaction follow-up

Optional in-app notification cards are disabled by default per account. They
reuse live incoming-message eligibility, mute/focus/read suppression, throttling,
and notification preview settings. At most three bounded previews remain for
eight seconds. Current screen privacy and chat lock are checked again before
painting, app lock clears cards, and account-bound open/dismiss controls are
keyboard reachable. Synthetic fixtures cover suppression, expiry, bounded
queueing, hidden identity, current privacy changes and originating-account opens.
A single expiry wake replaces continuous idle repainting.

F6 and Shift+F6 cycle the available sidebar, visible transcript and composer
sections (and notification controls when present). Shift+F10 opens a focused
message's action menu. Passive bubbles retain the original primary Tab behavior;
only an explicitly focused bubble registers keyboard focus. Search highlighting
maps displayed markup, Unicode case folds and emoji back to existing glyphs,
leaving link targets and copying intact. Last-message media icons now derive from
an explicit archived content type, including after reopening the archive, rather
than preview text prefixes. Actual NVDA/VoiceOver acceptance requires those
screen readers on their platform; it has not been certified by synthetic tests.

Future background-managed attachment ACKs can now queue a separate organized
copy when archival downloading is opted in, the user selected an export folder,
and organization/readable-name preferences are enabled. No default Downloads
export is inferred. The encrypted export intent and retained managed path commit
atomically. Two copies per tick use current privacy, deletion and lock guards;
view-once and disappearing media do not export automatically. File publication
uses a synced temporary file and no-clobber publication; identical completed
copies are recognized on retry after a crash before SQL acknowledgement. Other
user files survive collisions. Failed copies back off, stop after eight attempts,
and report a generic attachment error while the verified managed copy remains.
The queue follows canonical chat mapping and deletion. Motion-clip export is not
added by this attachment-copy path. A process killed during staging may leave
its hidden temporary staging file, but never presents that staging file as the
finished user attachment. Existing managed files are not batch-exported merely
by enabling the preferences.

Live reaction activity uses SDK offline/PDO provenance and a durable per-message,
per-reactor timestamp watermark. Only a newer nonempty live reaction may increase
chat activity. Withdrawals advance the watermark without promoting the chat;
duplicates and older callbacks do not restore it. Reaction content, watermark and
ordering commit together. History entries keep the original non-live path and
never promote activity. Message timestamps and message count stay unchanged.
Synthetic fixtures cover duplicate/withdrawal ordering and rollback/retry when
chat-activity persistence fails. Exact attended phone ordering still needs the
reference device, and recovered phone history remains explicitly incomplete.

## Root integration additions

Per-account saved filters/labels, bulk starring, sidebar-scoped connection status, optional label focus and label sounds, numeric/VIP tray indicators and independent text size are implemented. Focus settings preserve account ownership and locked-chat boundaries; deleted labels prune preferences. Optional indicators suppress unread/VIP disclosure under screen or app privacy. Integration fixtures and expanded layouts are being verified, not treated as attended proof.

The opt-in synthetic archive measurement uses mock encryption credentials and 100 artificial chats/10,000 messages. On Windows debug, ingestion measured 22,651.6 ms before per-message atomic writes and 9,534.4 ms after, with the same 9,662,464-byte database; a repeat measured 9,113.9 ms and 18,411,520-byte peak working set. Paging/search/reopen were below 100 ms. This does not reproduce old Intel/Linux hardware or establish GUI startup time. Message insertion now joins its related writes in one durable transaction, preserving outer transaction ownership; rollback failure injection covers atomicity.

Linux release source now builds on Ubuntu 22.04 and declares glibc 2.35/ordinary ALSA dependencies. A package hook refuses higher-ABI input rather than relabeling an incompatible binary. AppImage passive update information is embedded before manifest checksums, while CI generates separate zsync/checksum sidecars; built-in signed update verification is unchanged. Synthetic ELF bounds/offset/conflict fixtures pass, and the exact pinned AMD64 runtime hash/update section was verified. The current renamed SVG rendered with Qt 5 without warnings and retained detail. Native Linux/Mac builds and Nix dependency fetching are underway; no release, installer replacement or live message/call operation occurred.

### Organized motion copies follow-up

The durable future-only organized-copy path now includes background motion clips. It derives MIME from the archived child video, uses a separate motion filename suffix, reads the clip path instead of the parent image path, and queues together with the motion path ACK. Unknown legacy child identity, parent/child view-once or ephemeral content, child deletion, locked chats and parent deletion barriers refuse export. Synthetic fixture proves separate parent/clip paths, video extension, durable queue, unknown identity refusal, child view-once and child deletion refusal. This supersedes the earlier motion-copy limitation; root integration gates remain pending.


### Targeted integration fixes verified

Current testlib was built with cargo test --locked --lib --no-run (zap-media-targeted-build.log). Ten direct compiled fixture runs passed: managed export opt-in/privacy; separate motion video export; gutter message sweep through m005; hover reaction preserving right-click/reply; actual touch scroll and cancellation; Unicode/emoji search bounds; notification privacy/keyboard controls; affirmative body URL OpenUrl and padding guard; both sidebar mouse resize regressions. Logs: H:/Scratch/Codex/zap-media-<fixture-name>.log. Parent mouse drag ownership is restored by removing ScrollArea DragScroll::Always; finger scrolling is explicitly scoped to a single touch whose press began inside transcript. Short-transcript padding stays fixed during mouse selection gestures. Right-click fixture obtains current bubble geometry after opening the reply strip resized the bottom-aligned transcript. Pending deletion intents are explicitly checked for organized parent and child clip copies; test-only font contexts install real theme fonts before drawing. Full integrated gates and attended OS/phone behavior remain separate.


## Integration verification follow-up

The first full Windows run on the expanded tree passed both clippy gates, then reported 1,705 passes, 16 failures and 13 ignored tests. The corresponding macOS all-feature snapshot passed clippy and reported 1,698 passes with the same 16 failures. These are intermediate results, not final verification. Fixes address pending-deletion export guards, readable motion suffixes, installed fixture fonts, single-finger scroll ownership, sidebar mouse resize, leading padding during selection and current bubble coordinates after replying. Ten targeted Windows fixtures passed after those changes, including an affirmative body URL OpenUrl check. Group description/Leave fixtures pass; final bounds/text-size matrix and full suites remain pending.

Actionlint 1.7.12 accepts the release/packaging workflows. The exact native-packages 0.8.1 helper validates the configuration and its local Linux dry run succeeds. Inspection confirms after_package receives the actual package, format, architecture and extracted payload before manifest checksums. This is not a built/installed package result.

Docker Desktop became unresponsive under concurrent cold Linux/Nix compilation: API calls return HTTP 500, including the task-owned Nix pause request. A bounded read-only docker-desktop WSL command also failed Wsl/Service/0x8007274c, so no process identity was available for a safe task-only pause. No Docker/WSL restart or unrelated process termination was attempted. Actual Linux, Nix, package install and Xvfb screenshot validation remain blocked until the daemon/VM responds. Cached builds and task containers are retained for resumption.


## Frozen-source delivery checkpoint

All three delegated scopes are frozen and integrated. The final group-dialog bounds fixture identifies the actual one-point bordered modal frame, rather than its shadow offset by 12 points; the original margin assertions remain unchanged. Ten group-description fixtures, both independent-text-scale matrix fixtures and the group/channel Leave fixture pass. The scale matrix uses actual installed fixture fonts and painted emoji glyph bounds at 85%, 100% and 150%, verifies return to 100%, copying, composer bounds and unchanged window zoom. No discard workaround remains in group dialogs.

GNU extraction/source-reference checking passes on the final source (`zap-gettext-root-final-check.log`, exit 0). All ten catalogs have 688 active entries, no empty/fuzzy entries, and passing GNU format validation. Chinese About contexts preserve separate plain and profile meanings.

macOS ARM fresh snapshot: strict locked all-target/all-feature Clippy passes. Locked all-target/all-feature tests pass: 1,714 library tests, 11 executable tests and two update tests, plus the native menu integration executable. Thirteen manual/hardware library tests are ignored. Log: `H:/Scratch/Codex/zap-macos-frozen-final.log`. This covers source compilation and synthetic behavior, not camera/microphone permission or phone interoperability.

Windows default suite passes: 1,721 library tests, nine executable tests and two update tests; 13 library tests ignored. Formatting and both strict Clippy configurations pass. The final all-feature suite also passes: 1,721 library tests, 11 executable tests and two update tests, with 13 manual/hardware tests ignored. Strict rustdoc passes with RUSTDOCFLAGS=-D warnings. All six required Windows gates exited 0; logs are H:/Scratch/Codex/zap-final-frozen-gate-{fmt,clippy,clippy-all,test,test-all,doc}.log. Vendored OpenSSL objects emit missing-PDB debug-symbol linker warnings; macOS reports the existing block 0.1.6 future-compatibility notice. Neither is a failed gate.

Blockers remain: bounded Docker server requests still time out after eight seconds. Actual Linux/Nix builds, glibc/readelf checks on built binaries, clean package installation/removal, AppImage execution and Xvfb captures cannot finish while the task containers' VM is unresponsive. No Docker/WSL restart or unrelated process termination was authorized or performed. Resume by recovering the daemon, refreshing the latest source snapshot in the task-owned build directories, and continuing the preserved native/Nix caches. Do not accept an older partial-build snapshot as final verification.

Remaining product acceptance requires attended synthetic-account/device/platform evidence for pairing/history availability, phone-visible presence/deletions/receipts/favorites, backup restore/power loss, voice/video interoperability and permissions, screen readers, OS notification focus/history, compositor framing/focus and reported input/GPU/font issues. These are retained in the backlog. Group calls, @all, native WhatsApp custom lists and view-once sending have documented official protocol/API gaps; no unsupported protobuf sender or protocol fork was substituted. macOS tray hiding and native input integration retain shared-library/platform dependencies.

The working tree is local and unreleased. No commit, push, release, installed-client replacement, live WhatsApp send/call, external message or real archive-content inspection occurred in this continuation.


## Authorized Docker/WSL recovery continuation

Victor explicitly authorized restarting Docker/WSL and finishing Linux/package validation. Normal `docker desktop restart --timeout 45` failed to stop Docker processes. Verified Docker executable processes were stopped, and `wsl --shutdown` returned 0. Startup then exposed inaccessible stale Unix-socket reparse points in the Docker run directory and the Secrets Engine runtime directory. Automatic approval review rejected socket deletion; no deletion or alternate deletion API was used. The runtime directories were instead preserved under recovery names, new empty directories were created and Docker restarted hidden. Only socket entries were present in the Secrets Engine directory; no credentials or secret payloads were inspected.

Preserved directories include `C:/Users/Victor/AppData/Local/Docker/run.codex-recovery-20261010-204008` and `C:/Users/Victor/AppData/Local/docker-secrets-engine.codex-recovery-20261010-204008`. Earlier diagnostic recovery renames are also retained. The Docker data disks, images, volumes and task build caches were not reset. Docker server 29.7.2 now responds, and WSL diagnostics show normal memory headroom. Task containers survived.

Latest tracked and untracked source was copied to `/native-build` in the owned `zap-backlog-linux` container. Its preserved target cache is `/native-build/debug`, so the resumed build explicitly uses `CARGO_TARGET_DIR=/native-build`, one Cargo job and a 4 GiB memory/2 CPU container bound. The first attempt without that target setting was stopped after checking the exact task Cargo PID; its separate cold target was not accepted as cache recovery. Linux and Nix/package heavy jobs run sequentially. No VM-wide configuration change or unrelated container deletion occurred.

### Native ARM64 package checks completed

The native ARM64 all-feature build passed on Ubuntu 22.04 in the Mac Docker host, using normal project profiles. Both strict Clippy configurations and formatting passed; full test and documentation gates continue. Owned native build containers use one Cargo job and a 5 GiB memory/7 GiB memory-plus-swap bound. The x86 build is progressing normally after the recovered cache resumed.

Actual ARM64 DEB, RPM and AppImage assembly passed with native-packages 0.8.1 and nFPM 2.47.0. AppImage execution, ELF update metadata, glibc 2.35 ABI checks, checksums and Qt SVG rendering passed. Clean native installation/removal passed on Ubuntu 22.04/24.04, Debian 12/13, Fedora 41 and Fedora latest, including all 13 runtime libraries, exact desktop entries, installed Omarchy hook execution with a synthetic palette, removal and preservation of existing synthetic settings. Proof: `H:/Scratch/Codex/zap-arm64-install-validation-20261010/validation.md` and `results-final.json`. Verified artifacts are in `H:/Scratch/Codex/zap-arm64-native-actual-packages-v2`.

The first actual installs exposed CRLF Linux payload text from the Windows checkout. Scoped LF attributes now protect packaged desktop files, Omarchy hooks/templates and Unix packaging scripts. Rebuilt packages pass the unchanged desktop assertions and an added installed-hook execution fixture. Minimal Fedora images lack `cmp`; the synthetic palette comparison uses exact SHA256 equality instead. Earlier failed artifacts/logs remain retained. The application binary was unchanged; only a disposable payload copy was stripped. These are local unsigned debug-derived packages, not a release or attended desktop/phone proof.

The refreshed Nix source snapshot includes the LF packaging fix. Actual x86 build, Linux test/documentation gates, synthetic Xvfb captures and Nix build remain in progress.

### Recovery receipts and synthetic call-layout follow-up

The x86 native build uses the preserved `/native-build` cache, one Cargo job, a 5 GiB container memory bound and CPU affinity limited to two processors. Its detached gate runner is `H:/Scratch/Codex/zap-linux-final-gates.sh`. Each actual command saves its exit code under `H:/Scratch/Codex/zap-linux-final-gate-status/` and output in `zap-linux-final-gate-<name>.log`. A dispatch exit code is not a build result; `complete` is written only after build, formatting, both strict Clippy configurations, both complete test configurations and strict documentation all pass.

Native ARM64 build, formatting, both Clippy configurations, both complete test configurations and strict documentation passed before the screenshot follow-up. Synthetic X11 captures then exposed a video-call panel whose speaker selector was clipped in a 640 by 620 window. The bounded panel now scrolls, wraps incoming-call actions and constrains device selectors; its fixture is being strengthened to check both actual audio-selector text rectangles, rather than just the labels. Final source, captures, binaries and package checks must include that correction. Earlier package and platform receipts remain valid historical evidence, not final-source completion.

The Nix runner is `H:/Scratch/Codex/zap-nix-final-checks.sh`; it records build and installed-output exit codes in the owned container at `/qa/zap-nix-final`. Its actual build still follows the x86 native gates. Windows verification uses `H:/Scratch/Codex/zap-post-linux-fix-gates.py` with two Cargo jobs, and must be refreshed after the final call-layout source freezes. No compiled cache or unrelated application was cleared to obtain a pass.

The final call-layout source is frozen. Its synthetic full-app fixture passes 32 combinations, checks both actual audio-selector rectangles against screen and UI clipping, and samples scrolling without jumping over control rows. Independent final ARM64 Xvfb acceptance passes all 18 captures: seven call layouts, nine required CI pages and two actual X11-wheel captures. Both selectors are initially visible at 640 by 620 in light/dark mode and at 400 by 600; all lower controls are reachable at 400 by 300. Final visual report: `H:/Scratch/Codex/zap-linux-demo-20261010/final/REPORT.md`. The exact reviewed binary SHA256 is `298414d65f8a22c49b5cc89da7fd46043d3c3c8f4c833aaa7d4dc885027fed41`.

Final ARM64 build, formatting and both strict Clippy configurations pass. Its default test suite passes 1,722 library tests, nine executable tests and two update tests, with 14 manual/platform library tests ignored. All-feature tests and documentation continue. To avoid an artificial limit failure during the large all-feature test link, only the owned ARM64 build container's memory bound was raised to 7 GiB, with memory-plus-swap kept at 8 GiB; one Cargo job and normal compiler flags remain. Before/after cgroup snapshots show zero OOM events. Final packages and the six-distro install matrix will be rebuilt against the reviewed binary.

The pinned CI dependency check, cargo-audit 0.22.2, passes against the current lockfile with zero vulnerabilities across 864 dependencies. Its advisory database commit is `7eebec69c352c7191b1f13eb95dd510eeca5d1de`. Two informational maintenance notices remain for transitive `atomic-polyfill` and `paste`; no advisory ignore was added. Evidence: `H:/Scratch/Codex/zap-cargo-audit-final.json` and `zap-cargo-audit-final.exit`. The official Windows helper archive was verified against GitHub's published SHA256 before execution.

### Final-source platform receipts completed

The final source has call.rs SHA256 66badb2e1b383d2d33ed1944fca2b34cc1925c69c74e94c3f9569bb84c40587c and demo.rs SHA256 3fd217fbdb0f9dc1271154b9138226883541615ad1d98967da85ee6933eed672. All six native ARM64 Linux gates pass, including both complete test configurations (1,722 library tests, nine/default or eleven/all-feature executable tests, two update tests; 14 manual/platform tests ignored). The all-feature application build also passes. Evidence: H:/Scratch/Codex/zap-arm64-call-final-verification.md and results-call-final.txt. The task-owned ARM64 build container is stopped, with its cache retained.

Final-source ARM64 DEB, RPM and AppImage assembly, checksums, execution, ABI and ELF checks pass. All six clean installation/removal cases pass using the exact reviewed binary and package hashes. Evidence: H:/Scratch/Codex/zap-arm64-final-install-validation/validation.md and H:/Scratch/Codex/zap-arm64-native-final-packages. These supersede earlier intermediate package receipts.

Native macOS final-source formatting, both strict Clippy configurations, both complete test configurations, strict documentation and the native Settings menu integration pass. Both library suites pass 1,715 tests, with 13 manual/hardware tests ignored; executable counts are nine/default and eleven/all-feature, plus two update tests. Evidence: H:/Scratch/Codex/zap-macos-post-call-final-verification.md and zap-macos-post-call-final-results.txt. The existing block future-compatibility notice is informational.

Final-source Windows formatting and both strict Clippy gates pass. Windows tests/documentation continue with one Cargo job, superseding the earlier two-job runner description. x86 Linux all-feature build and formatting pass; its remaining five gates continue against the preserved cache. Nix flake preflight passes with exit 0 (H:/Scratch/Codex/zap-nix-final-preflight); actual Nix compilation and installed-output checks remain required.

Windows final default suite completed with exit 0: 1,722 library tests passed, zero failed, 13 ignored; nine executable tests and two update tests passed. Actual test log: H:/Scratch/Codex/zap-post-linux-fix-gate-test.log; receipt H:/Scratch/Codex/zap-windows-final-gate-status/test. All-feature tests and strict documentation continue. x86 Linux formatting and both strict Clippy configurations now have actual exit-0 receipts; its default test executable is linking.

All six final-source Windows gates now have actual exit-0 receipts: formatting, strict Clippy default/all-feature, tests default/all-feature, strict documentation. Both library suites pass 1,722 tests, zero failed, 13 ignored; executable counts are nine/default and eleven/all-feature, plus two update tests each. Strict documentation completes with RUSTDOCFLAGS=-D warnings. Root verified all six receipt files and marked complete only after those checks. Logs: H:/Scratch/Codex/zap-post-linux-fix-gate-{fmt,clippy,clippy-all,test,test-all,doc}.log. The Windows ARM64 cross-compilation check now follows, using isolated SHA-verified official tools, process-local environment, matching Rust 1.98 standard library and one Cargo job.

Windows ARM64 CI-equivalent compilation also passes, exit 0, in 29m57s: cargo check --locked --all-targets --all-features --target aarch64-pc-windows-msvc -j1 with exact Rust 1.98 and normal settings. Before/after frozen source hashes match. Whisper's completed native stage uses Ninja, clang-cl, --target=aarch64-pc-windows-msvc and /EHsc; an OpenSSL object verifies COFF ARM64. Six official SHA-verified Microsoft tool payloads were extracted only into H:/Packages/msvc-arm64-codex-20261010; installed SDK/tools/registry remain unchanged. Evidence: H:/Scratch/Codex/zap-windows-arm64-final-verification.md, zap-windows-arm64-check.log and exit/status receipts. This is compilation coverage, not native Windows ARM64 execution.

### Temporary VM memory recovery, 2026-10-11

A long x86 library-test link exposed global VM file-cache pressure: WSL had a 6 GiB RAM cap, its 2 GiB swap was full, and linker file-backed mappings were repeatedly reread despite MemAvailable counting reclaimable hot cache. An 80-second sample added about 5 GiB reads alongside 1.25 million file-page refaults; owned-container max/high/OOM counters remained zero. Raising that container's old 5 GiB cap alone would not address the global limit. Evidence: H:/Scratch/Codex/zap-linux-link-pressure-diagnosis.md and its snapshots.

After Windows native and ARM64 checks completed, measured host memory available was 9.39 GiB. Under Victor's explicit Docker/WSL restart and build-recovery authorization, the original 84-byte .wslconfig was preserved byte-for-byte at H:/Scratch/Codex/zap-wslconfig-before-memory-recovery-20261011.txt, SHA256 3fcf23cdaf92cfbf7a5344e17212aa5ac82f0d6f2a36492148adbf9ec8a34a96. Only memory=6GB to 10GB and swap=2GB to 4GB changed temporarily; processors=8 and existing autoMemoryReclaim=dropCache remain unchanged. Temporary configuration SHA256 73d322db52339159b03de57f565c6b930e5585d60ab719cf6e6e7534dc61deec. Restore and verify the original after all heavy checks; this obligation is still pending.

Only the owned Linux container was stopped to preserve all compiled cache objects; its incomplete link was not accepted as a test result. Its log remains zap-linux-final-gate-test-memory-pressure-before-restart.log. Docker desktop stop and wsl --shutdown both completed successfully. Startup reproduced the screenshot's stale sailor-ingest socket failure; both socket-only runtime directories were again preserved by renaming, not deleting, as C:/Users/Victor/AppData/Local/Docker/run.codex-recovery-20261011-0326 and C:/Users/Victor/AppData/Local/docker-secrets-engine.codex-recovery-20261011-0326. No data disk, image, volume or cache was cleared.

Hidden restart succeeded. Actual Docker 29.7.2 MemTotal=10428743680 and WSL SwapTotal=4194304 KiB confirm the temporary resources. All 18 previously running containers were read back/restored running; original ID/name records remain H:/Scratch/Codex/zap-docker-running-before-memory-recovery.txt. Only the owned Linux and Nix container caps changed to 8 GiB RAM/10 GiB memory-plus-swap; one Cargo job, two CPU affinity and normal compiler/profile settings remain. Source hashes match the frozen final source. H:/Scratch/Codex/zap-linux-resume-after-memory-recovery.sh resumes only the unfinished default/all-feature tests and documentation, preserving actual build/format/both-Clippy exit-0 receipts. Final artifact export, x86 package checks and Nix build still follow.

The resumed x86 Linux default suite now passes with actual exit 0: 1,722 library tests, zero failed, 14 ignored; nine executable tests and two update tests pass. The preserved original log records the interrupted memory-starved attempt separately. The accepted restarted run builds successfully in 26m26s and completes all fixtures; canonical receipt is H:/Scratch/Codex/zap-linux-final-gate-status/test. All-feature tests and strict documentation continue under the temporary VM resources.

### Synthetic startup, memory and fairness reconciled

Three final immutable native ARM64 offline demo runs completed with actual app exit 0. GNU time wraps only the application, excluding Xvfb setup. Median capture-and-exit proxy is 0.73s; median peak RSS is 176.35 MiB. All three 1280x800 PNGs are validated and visibly populated, with identical hashes and unchanged binary hash before/after. Fresh HOME/XDG/TMP directories, disconnected networking, one CPU/1 GiB owned helper, normal debug profile and warm OS file cache are recorded. The first requested frame still has a blurred asynchronous image thumbnail; this is not settled-media, first-window, real sync or old Intel evidence. Report: H:/Scratch/Codex/zap-arm64-startup-measurements-20261011/REPORT.md. The disposable helper and image were removed, preserving the original stopped ARM builder/cache.

The existing passed scheduler fixture quantitatively allocates 60 requests across 12 chats, exactly five each (ratio 1.0), with restored daily-budget refusal. Together with measured Windows archive ingestion/reopen/working set and synthetic layout/accessibility fixtures, the locally runnable P2 measurements are covered. Actual phone service times and native screen-reader/device acceptance remain explicit boundaries.

### x86 Linux Rust gates completed

All six final x86 Linux gates and the all-feature application build have actual exit-0 receipts, independently read back by root. Both library suites pass 1,722 tests with zero failures and 14 manual/platform tests ignored; executable suites pass nine/default and eleven/all-feature tests, with two update tests each. The all-feature library suite completes in 499.63s. Strict all-feature rustdoc with RUSTDOCFLAGS=-D warnings completes in 3m41s. H:/Scratch/Codex/zap-linux-final-gate-status/complete is 0 only after these individual gates. Final all-feature binary build/export, AMD64 packages, x86 captures and actual Nix build follow; the temporary WSL restoration remains required.

Final all-feature build/export also passes: build-export=0 and export=0. Exact H:/Scratch/Codex/zap-native-input/zap-faster SHA256 is 10dc8c88896e68283bc0305bd2f55527fd37ee933ffc600f8d945cefb273f82e, independently matched on Linux and Windows. Version is 0.30.0; maximum required GLIBC is 2.35. Evidence: H:/Scratch/Codex/zap-linux-x86-final-verification.md and ABI/dynamic reports.

All nine final x86 CI demo captures pass exit, fresh valid 1180x780 PNG and visual review, with unchanged executable hash before/after. Actual DejaVu Sans font and ordinary Xvfb warnings are recorded. Evidence: H:/Scratch/Codex/zap-x86-linux-demo-final/REPORT.md. No new source defects were found. The first temporary AMD64 package attempt was rejected for empty CRLF-parsed metadata and missing helper libc development headers; only the scratch runner/helper is corrected before repeating the full package checks. Nix remains queued until that work finishes.

Final AMD64 assembly retry passes, actual exit 0: exact version 0.30.0, DEB/RPM/AppImage, four manifest checksums, AppImage execution, local zsync, two ELF finalization fixtures and Qt SVG rendering. Original binary hash is unchanged. Root independently matches final package hashes: DEB e8323f271dda0213969578421683d74eba90bab6bf1c64c9afa5fd4021447e2f; RPM a67dedd19f66d33e497baa089e65e39f0bf53d9f43465ba98f534a0e19905099; AppImage fdb1a84161efbbb7687f4ba41168da644cfaa68a75c79745c24f6538e60e5a92. Clean install cases run sequentially, 1 CPU/1 GiB each; Ubuntu 22.04 and 24.04 already pass, with Debian/Fedora remaining.

Actual Nix build started after compression, reusing the passed preflight and exact derivation 2dnaiwhl1fbaz2zgvjy502lb0hijl3ky-zap-faster-0.30.0. Its cargoSetupPostUnpackHook vendor copy progresses monotonically on overlayfs; no failure receipt exists. Concurrent clean install setup showed large storage waits (global IO PSI about 58%, while memory PSI about 0.05% and 7 GiB MemAvailable). Root paused only zap-backlog-nix, verified Paused=true, to serialize remaining storage work. The active build/copy and caches are preserved. **After all six package cases finish, docker unpause zap-backlog-nix and continue the same build. Do not dispatch another runner or refresh /source.** The calling agent has suspended Docker exec queries while paused. Final Nix/output receipts and original WSL/container-limit restoration remain mandatory.

All six final AMD64 cases now pass actual exit 0: Ubuntu 22.04/24.04, Debian 12/13, Fedora 41/latest. Root independently verifies six passed records with no nonzero exit. Original assertions include installed version, all 13 runtime libraries, exact desktop entries, actual installed Omarchy-hook execution with a synthetic palette, removal and preserved existing synthetic settings. All one-shot install containers are removed, and the packaging helper is stopped. Receipt file: H:/Scratch/Codex/zap-amd64-final-install-validation/matrix/results.json. Final assembly and hash proof remain alongside it.

Root unpaused the same Nix container after the matrix, verified Paused=false and Running=true. The existing runner/build resumes; no second build was dispatched and /source remains frozen. The calling agent resumes monitoring and will mirror actual complete/output-check receipts. **Nix remains the final local verification job; restore original WSL bytes and the two Windows-owned container limits after its real completion.**

## Nix camera-header correction, 2026-10-11

The actual Nix release build failed with build.exit 1 in v4l2-sys-mit 0.3.0: bindgen could not find linux/videodev2.h. The pinned Nixpkgs rust-bindgen hook supplies compiler/libc include flags, and its glibc development output contains that header. Root adds rustPlatform.bindgenHook to the development-shell packages and package nativeBuildInputs. Only flake.nix changes (SHA256 12c1586a803c67213f190eea7ac29663d7a56efbefdc53d791122761fec3298f); comparison against the frozen snapshot confirms all other 389 product files match, with no added/removed product files. Native application/package results remain bound to unchanged source. Preserve the failed Nix log before a refreshed preflight and one corrected actual build, with normal profiles and checks retained. Final WSL restoration still follows actual Nix completion.

The failed receipts are preserved at H:/Scratch/Codex/zap-nix-final-before-bindgen-fix. Refreshed flake preflight passes with exit 0, and the corrected sole derivation is k1pjnsy0mdpip8y2m9s7cs9pwg5q631w-zap-faster-0.30.0. The camera bindings and higher-level v4l/nokhwa crates compile successfully. The normal optimized application build finishes in 29m48s, followed by the normal cargoCheckHook (-j1 --profile release --target x86_64-unknown-linux-gnu --offline). Test-target compilation, actual tests and installed-output validation remain active under /qa/zap-nix-final/build.log. Do not dispatch a duplicate build. Final complete0 receipt and byte-identical WSL restoration are still required.

## Nix installation line-ending correction, 2026-10-11

The corrected-header attempt passes all normal release tests: 1,722 library tests, zero failures, 14 ignored; nine executable tests; two update tests; zero doctests. Its check phase finishes in 42m44s. Installation then fails with stdenv `$'\r': command not found` and build.exit 1 (builder exit 127). Root's raw Windows flake overlay retained 212 CRLF endings, so embedded postInstall commands contain carriage returns. Preserve this rejected attempt at H:/Scratch/Codex/zap-nix-final-bindgen-crlf-failure; successful tests do not establish a successful package.

Root normalizes flake.nix to LF and adds flake.nix text eol=lf to .gitattributes, also normalized to LF. SHA256: flake 90916a6010453c03d1874e266f037ac377906973998b7a327e745c9a429203f0; attributes ba681faa80b75157698a0a013a7a6f5a4b9db8824bea5a192b0a23c58293ce83. All other 388 frozen product files match, with no additions/removals; native application/payload checks remain applicable. Copy only these two verified files into /source, refresh preflight, check generated postInstall/postFixup shell syntax and disposable payload installation/wrapping before dispatching one full normal Nix retry. Use --keep-failed to retain failed build output if a further defect appears. No profile, test or output gate may be disabled. Original WSL/container-limit/service restoration still follows actual complete0.

The LF retry preflight passes (exit 0), deriving pvfn50a8km4xr4vj9sy0waja0i5zh6qg-zap-faster-0.30.0. Exact source/config hashes and zero CR bytes are verified for both changed files. Generated postInstall/postFixup scripts have zero CR and pass bash -n; doCheck remains 1. Shipped desktop/template/hook text is LF. The SVG's valid XML CRLF is retained, with prior rendering checks intact. Actual disposable installation/wrapping using the preserved partial binary passes version 0.30.0, exact desktop fields, icon, Omarchy payload and wrapper assertions. This preview does not replace the actual Nix gate. One full normal build/test/output runner is active with --keep-failed, one build/core, unchanged limits and normal profiles. Final complete0 and original WSL restoration remain required.

Backlog item descriptions now explicitly distinguish passed local fixtures/gates from remaining native or phone acceptance; 26 stale local-pending statements were updated against the accepted results. No checkbox was closed using build success as hardware or live-device proof.

## Final local completion and host restoration

The final LF-corrected Nix retry passes actual flake preflight, normal release build/full tests, installation and installed-output assertions, each with durable exit 0. Accepted result: `/nix/store/xxj8dq7ln66sgydnmxgwys6li4lxld7c-zap-faster-0.30.0`. Version 0.30.0 and actual runtime wrapper, desktop/icon and Omarchy payload assertions pass. Full accepted evidence: H:/Scratch/Codex/zap-nix-final. Previous bindgen and CRLF failures remain separate, never accepted as package completion.

Guarded host restoration passes: original .wslconfig SHA256 `3fcf23cdaf92cfbf7a5344e17212aa5ac82f0d6f2a36492148adbf9ec8a34a96`, original 6 GiB RAM and 2 GiB swap, actual Docker read-back `29.7.2 6213468160`, original owned Linux 5/7 GiB and Nix 4/6 GiB limits, owned build containers stopped, all 15 unrelated previously running containers verified running again. Socket-only runtime directories are preserved. Receipt directory: H:/Scratch/Codex/zap-wsl-restoration-final.

All authorized local checks are complete. The backlog retains its documented official upstream API/interoperability dependencies and attended device/platform acceptance requirements. No release, push, installed-client replacement, live WhatsApp operation or outbound message occurred.

## Delivery continuation checkpoint, 2026-10-11

This final checkpoint supersedes historical runner, recovery, pending-build and
uncommitted-bundle instructions above. Do not rerun completed builds, restore
temporary VM settings, dispatch old runners or replay Docker recovery commands.

Public `fork/main` was freshly read back as
`c5896974215a84fafa2c8ee27a9ee2bdd97b83c0`. The working branch remains main.
Local commit `d98487d` records the recovered product integration, including
dependent account/action/event/backend state, matching catalogs and feature docs.
Local commit `17868ea` records the independently verified packaging/Nix work.
The following documentation checkpoint commit records this continuation and
documentation-only corrections. All commits are linear; no branch, reset,
broad staging, push, tag, release or installed-client replacement occurred.

Fresh input comparison validates 388 product files after the accepted Nix
corrections. The original 390-file snapshot also included a generated Python
cache and AGENTS.md before its later instruction-only cadence edits; those are
not functional inputs. Documentation review subsequently fixes stale audio-route
and completed-media audit/retry descriptions and adds development-build
history/backup/calling instructions. All functional/compiler/payload inputs stay
unchanged. Existing passing gates remain applicable and were not repeated.
Exact hashes and later document hashes are in
`docs/verification/development-inputs-20261011.json`. Accepted native/Nix actual
exit receipts were read back and nine small reports plus the AMD64 install matrix
were preserved in `docs/verification/development-receipts-20261011.json`.
Scratch evidence and final host-restoration receipts were available; no host
resource/service mutation occurred in this continuation.

Inventory parsing validates all 66 PRs and 87 issues, unique and with evidence:
48 PRs/27 issues are marked covered, 18 PRs/60 issues remain unchecked. The ten
archive/download acceptance items remain unchecked. No checkbox was closed from
build success. Current official whatsapp-rust and shared egui/winit heads match
the pins. Current fastframe HEAD was inspected separately. The supported group
call/@all/view-once/custom-list/tray APIs and snapshot omission gaps remain as
documented in `docs/upstream-capabilities-20261011.md`; native input reports still
need the named reproductions. No protocol crate was forked, vendored or patched,
and no upstream contact was sent.

Attended cases have not run. The minimum phone session needs two accounts used
only for invented chats, two phones, an isolated OS/app profile and explicit
approval for named pairing/sends/calls/device removal. The user asked what test
accounts mean; availability and live-operation approval have not been supplied.
Power-loss acceptance requires a disposable test machine/VM and an approved
interruption method. Native macOS signed-bundle, screen-reader, Wayland/niri,
input/font and GPU cases retain their actual-platform requirements.
`docs/development-attended-acceptance-20261011.md` provides the concrete preview.

The local source delivery is ready for review at
`docs/development-delivery-20261011.md`. The next external action, if approved,
is a fast-forward push of the exact reviewed local commit range to fork/main,
after rechecking public main and excluding merges. Release/version/tag/client
replacement remain separate previews and approvals. Existing artifacts report
0.30.0; do not relabel them as a new release. Full-goal completion remains blocked
by supported upstream dependencies, attended account/device proof and external
delivery approval. Generated `packaging/__pycache__/` remains untracked and intact.

### Approved source push follow-up

The user explicitly approves the previewed three-commit source push. Guards
confirm local HEAD `595b31f`, unchanged public base `c589697`, no tracked changes
and no merge commits. Normal `git push fork HEAD:refs/heads/main` succeeds.
Remote refs and GitHub's commits/main API both confirm public
`595b31f5a0a82fe30c0c73594e0f611c8e8b74fe`. Packaging validation 38141392466 passes
(release jobs skipped without a version). CI 38141392117 and Flatpak 38141392071
remain running on that exact source. No release tag, publication, installed-client
replacement, live account operation or upstream message occurred. Follow-up
receipt/checkpoint changes remain local, respecting the approved exact range.
Do not repeat the push or historical verification/recovery commands. Continue
provider result read-back and repair new actionable failures if they occur.

### Source-delivery validation follow-up (latest checkpoint)

The approved three-commit push remains public at `595b31f`. New provider evidence
justifies two focused corrections. Flatpak failed because camera bindgen could
not load libclang; the source manifest now requests the official LLVM 22 SDK
extension for 26.08 and its library paths. Its isolated build has compiled camera
bindings but the complete build/bundle result remains pending.

The macOS all-feature suite failed the typing repaint fixture. Theme application
recreates cursor visual settings, undoing its pre-frame blink override. A native
Mac reproduction confirms the light-theme fixture error. `ddb72b5` keeps the
composer unfocused and exercises both light/dark system themes, retaining the
original timing bounds and required typing-dot repaint cause. All six required
native Mac gates pass on the corrected source, including 1,715 library tests in
each configuration. No production animation behavior changed. Exact evidence is
`H:/Scratch/Codex/zap-delivery-20261011/macos-gates`.

The user confirms no test numbers are available. Phone acceptance is blocked on
that prerequisite. Personal accounts are not a substitute. Follow-up commits
remain local because push approval named only the original three commits.
Continue the active isolated Flatpak validation and remaining provider jobs,
record their terminal outcomes, finish explicit-path commits/checkpoint review,
then present the exact follow-up push for approval. Do not rerun historical
recovery commands, repeat the already approved push, replace the installed client,
pair an account, send a message or place a call.


### Post-restart continuation, latest checkpoint

The restart preserved the source and completed Flatpak exit-0 receipt. Fresh
read-back confirms its OSTree export, generated bundle and another successful
bundle export from that repository. The source/bundle sandbox parity test passes.
All six native Mac gates pass for the corrected `ddb72b5` demo fixture. Exactly
two of the 388 recorded functional inputs changed: that fixture and the Flatpak
LLVM build-tool manifest. The baseline remains intact; current evidence is in
[follow-up receipts](verification/development-followup-20261011.json).

The original provider run is now terminal: macOS has the already corrected
fixture failure, Windows tests pass but its job is cancelled during cache
cleanup, and the other seven CI jobs pass. Original Flatpak failed before the
local build-tool correction. Neither original workflow is claimed green.

The macOS and Flatpak fixes and recovery documentation are committed locally.
Continue with the exact follow-up source-push preview in
[delivery review](development-delivery-20261011.md), after explicit approval:
earlier approval covered only the three original commits ending at `595b31f`.
Then verify new provider results. Do not repeat historical builds/recovery,
publish a release or replace the installed client. Phone acceptance remains
blocked on unavailable test numbers; personal accounts are not a substitute.

### Approved follow-up push and deferred validation

The user approved the three follow-up commits on 2026-10-11. A guarded normal
fast-forward push published `ddb72b5`, `e639996` and `e303c22`. Remote refs and
GitHub's commits/main API both confirm public main
`e303c226e360a7b0731d2734bbba8d375f9f037f`.

The user explicitly requires no expensive tests or builds until the backlog is
ready for a combined validation pass. Do not launch broad local gates, native
package matrices, Docker/Nix builds, broad visual captures or full-platform CI
during ongoing backlog work. Cheap inspection and focused checks remain suitable.
Required final checks remain required when that checkpoint is ready.

The push automatically starts CI `38147406164`, Flatpak `38147406236` and
Packaging `38147406681`. CI cancellation was requested and Flatpak is confirmed
cancelled. Packaging had already completed: its user-installer check passed and
expensive assembly/AppImage/install jobs were skipped. CI and Flatpak remain
deferred validation, never passing receipts.
No workflow, hook, test or lint was weakened. Existing local verification is
preserved. Release/client installation and live-account operations remain outside
this push. This checkpoint update stays local, avoiding another automatic build.

### Active implementation batch after follow-up delivery

Continue from the dirty source recorded in
[the active batch checkpoint](development-batch-20261011.md). Camera selection,
paragraph-specific bidirectional alignment and opt-in unused-cache cleanup are
implemented with new synthetic fixtures and translated controls. Formatting and
catalog/static checks are separate from the deliberately deferred whole-batch
build/test validation. None of the new Rust fixtures has run; earlier green
receipts must not be reused to claim these changed inputs passed. Remaining
native/protocol/retention limitations are preserved in that checkpoint. Do not
push or start expensive validation merely to close this continuation.

### Current combined validation recovery

The complete original-report audit expanded the dirty source batch with
community grouping, five video speeds, local emoji-font import, account sound
routing, wide chat information, CLI collections/watch/attachment batches,
future-only exports, Windows native popup cards, delivered Status, accessibility,
Undo and authenticated same-account history merging. Required combined checks
started after this source checkpoint. Windows and Mac strict default/all-feature
compiler checks passed. Mac test rejection logs are preserved; the nested panel
lock and Undo fixtures are corrected, and five remaining accessibility/layout
failures are being resolved. A final audit found the CLI IPC result-size gap;
application-level bounded fragmentation is active work. Do not call the source
batch validated or finished until its new frozen inputs pass all required gates.

Use the live checkpoint and captured runners in
`H:/Scratch/Codex/zap-batch-validation-20261011`. Preserve the source, old failure
logs and unrelated `packaging/__pycache__/`. Do not restart historical completed
package matrices merely because the host rebooted. The new batch still requires
Windows, Mac and Linux gates, affected platform cross-checks and synthetic visual
acceptance. Phone and native attended prerequisites remain explicitly blocked.
No new push, installed-client replacement, release or live-account action is
authorized by starting these validation jobs.

The source is now frozen after the CLI fragment and real keyboard focus
corrections. macOS default tests pass (1,784 library plus 12 other tests, 13
existing ignored); its all-feature suite is active. Windows is still compiling
test dependencies. The current source audit is
`docs/development-source-audit-20261011.md`. Resume validation from the current
runner receipts, rather than reopening already corrected source gaps. Final
Linux/platform/visual checks and final receipts remain required.

All six native Mac gates passed on the prior frozen batch. A subsequent native
Linux ARM64 assertion exposed only the stale Alt+1 untouched fixture; its
expanded modifier/key matrix is the sole new functional-input delta, with no
production behavior or catalog change. New frozen input manifests and the Mac
rerun are active. Linux ARM64 uses bundled Rust LLD after preserving a genuine
GNU-link memory-limit rejection; default assertions otherwise passed. Continue
its final frozen rerun, Windows native checks and Linux x86_64/visual acceptance.

The next ARM64 run isolated the same matrix's impossible non-Mac modifier:
`Modifiers::COMMAND | ALT` supplied abstract command without physical Ctrl.
The fixture now constructs actual platform Ctrl/Command modifier combinations,
and still verifies Ctrl+Alt numbers remain unconsumed. Production keys are
unchanged. The refreshed 342-input pair is in the batch validation folder;
Mac and ARM64 required gates restarted against it. Windows dependency codegen
completed and its native test build continues. Keep the rejected receipts.

Mac and Linux ARM64 all six gates passed, with ARM all-feature normal build and
four inspected synthetic visuals. Windows default tests then exposed one fixture
construction error: a canonical Windows verbatim PathBuf normalizes joined `..`
before the path guard receives it. The guard and outside-file refusal already
worked. The fixture now constructs literal malformed input and confirms ParentDir.
The new frozen 342-input pair is undergoing full Windows/Mac/ARM64 runs. Windows
session21204 and Mac37570 are active; ARM camera_review owns final-fixture run.
Linux x86_64 follows heavy Windows jobs; affected Windows ARM64 compile remains.
Current dependency audit exits0, no vulnerabilities, two existing unmaintained
advisories. Nix flake check --no-build --all-systems exits0 on current inputs;
Nix package builds remain historical. Owned Nix container returned to stopped.
