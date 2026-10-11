# Active backlog batch, 2026-10-11

## Recovery after the second restart

All six required source gates pass on the corrected 342-input snapshot on
Windows x86_64, macOS ARM64, Linux x86_64 and Linux ARM64: formatting, strict
default/all-feature Clippy, default/all-feature all-target tests, and rustdoc with
warnings denied. Windows ARM64 all-target/all-feature cross-compilation also
passes; execution on Windows ARM64 is untested.

| Native target | Library tests passed | Existing ignored | Executable tests default/all | Updater tests |
| --- | ---: | ---: | ---: | ---: |
| Windows x86_64 | 1,790 | 13 | 10 / 12 | 2 |
| macOS ARM64 | 1,784 | 13 | 10 / 12 | 2 |
| Linux x86_64 | 1,791 | 14 | 10 / 12 | 2 |
| Linux ARM64 | 1,791 | 14 | 10 / 12 | 2 |

The normal Linux all-feature executable build and all nine refreshed synthetic
captures pass. Every image was reviewed. Two visible defects were corrected:
cache-cleanup controls now stack and wrap within their column, and the Status
explanation wraps instead of truncating. Source hashes still match after checks;
older passing receipts are preserved separately and do not validate newer edits.

The Windows restart at 15:26 interrupted validation without losing source. A
corrupt application incremental cache was quarantined after an explicit compiler
warning and fingerprint panic. Dependency caches and build/debug profiles were
retained. Low-memory linking was resumed with the pinned toolchain's bundled LLD
and one linker worker, retaining every required check.

The user approved temporarily raising WSL memory from 6 to 10 GiB and the owned
build container from 5 to 6 GiB, with Docker/WSL restart and restoration. Original
configuration bytes are now restored and hash-verified. Runtime readback confirms
WSL MemTotal 6,067,840 KiB, swap 2,097,152 KiB, and the stopped build container's
5,368,709,120-byte memory / 7,516,192,768-byte memory-plus-swap limits. All fifteen
media containers are running again, and all fourteen configured health checks
are healthy; fetcharr has no health check.

Docker's inaccessible runtime sockets required preserving checked socket-only
directories and recreating their empty paths while stopped. Images, volumes and
data disks were retained. The other task's temporary `agency-trello-20261011`
container no longer existed after the first restart and could not be restarted.

The user also approved restarting unresponsive MacBook Docker. Normal stop timed
out; remaining Docker application processes were stopped before a background
launch. Docker API readback and the other running services recovered. Finished
native application incremental caches were preserved in verified Windows
archives before removing only those folders. Dependencies, source and logs were
retained; MacBook Docker resource limits were unchanged. Fresh Linux ARM64 gates
passed and all 342 source hashes matched again afterwards.

Detailed receipts and exact hashes are in
[the current verification receipt](verification/development-batch-verification-20261011.json).
Captured logs, frozen source, rejected/interrupted attempts, synthetic images,
verified cache archives and original resource settings remain under
`H:/Scratch/Codex/zap-batch-validation-20261011/restart-20261011`, with corrected
snapshot results under `visual-fix`. GNU catalog checks and pinned translation
extraction pass. Previous package/Nix receipts remain evidence for their earlier
snapshots, not refreshed package or release claims for this source.

The feature batch remains uncommitted, unpublished and uninstalled. Attended
phone/native cases and documented official API prerequisites remain open. No
personal account, archive contents, live calls or user clipboard were exercised.

## Implemented source in the validated snapshot

- PR #255 and issue #219: per-account camera selection, persisted native identity,
  explicit refresh and selected-device refusal. Selection applies at the next
  deliberate camera start, avoiding an implicit restart/capture. Camera discovery
  uses the isolated own-executable helper, bounded private pipes and timeout/drop
  cleanup. Audio heartbeats preserve the camera list and incoming calls ring
  without waiting for enumeration. Portrait/landscape encoding retains aspect
  ratio under the existing bounded frame budget. Native V4L node/name identity
  cannot distinguish identical replacement cameras at the same node.
- Issue #248: explicit paragraphs align independently according to their full
  paragraph direction; wrapped continuation rows inherit that direction.
  Logical text indices, links, emoji, decorations and selection bounds remain
  part of the regression fixtures. Reported Arabic/Hebrew native font appearance
  and ink clipping remain separate attended requirements.
- Issue #204, user-selected policy: default-off, per-account age cleanup for old unused managed
  cache files. Referenced attachments and every durable background media job
  remain protected, including unpublished crash leftovers. Exports, linked
  paths, staging and subfolders are preserved. Privacy/transfer gates and bounded
  reference/entry scans fail closed. This does not expire referenced attachments
  or turn the retained background archive into disposable cache. The user
  explicitly chose to preserve every archived attachment and clean unused files
  only; general referenced-attachment expiry is outside that chosen policy. See
  [the cache policy](cache-retention.md) for scope and remaining work.
- Camera/cache controls are extracted and translated across all ten catalogs.
  Existing translations are preserved. Camera settings and cache policy remain
  account-local; legacy preferences default to the default camera and no cleanup.

Synthetic regressions were added for selected/missing/reordered camera identity,
bounded helper IPC and portrait encoding, camera-list preservation, mixed and
wrapped paragraph alignment, link ranges and repeated-layout bounds, cache age
and ownership limits, references/background jobs/exports, privacy/defaults and
account preference round trips. The call demo carries an invented camera; its
layout fixture checks camera reachability after scrolling and independently
retains actual audio-selector visibility checks. These fixtures are included in the active whole-batch runs; final receipts are pending.

## Cheap verification and dependency reassessment

Rust formatting/syntax parsing and Git whitespace checks pass. GNU extraction
and catalog format checks pass. No compilation, runtime or visual acceptance is
inferred from those checks. Static call-site/schema review confirms camera preferences reach start, answer and
upgrade paths, and new account settings/commands have default and mock wiring.
[Source hashes and cheap-check receipts](verification/development-batch-inputs-20261011.json)
identify this continuation separately from the earlier validated snapshot.

Official whatsapp-rust HEAD advanced by one sender-key performance commit; its
only changed file does not add the missing group/@all/view-once/custom-list APIs.
The tested dependency pin remains unchanged. Shared native input/tray refs did
not advance. [Dependency evidence](upstream-capabilities-20261011.md) records the
fresh read-back. No upstream communication was sent.

## Continue from here

The subsequent original-report audit found additional source gaps behind earlier
coverage labels. Active integration now also includes community grouping,
five video speeds, local color emoji font importing, account output routing for
custom notification sounds, resizable wide-window chat information, broader
CLI commands and attachment batches, future-only automatic export policies,
Windows native notification cards, read-only delivered Status viewing, and
keyboard/AccessKit accessibility, Undo and authenticated same-account historical
backup merging. These additions now pass the combined source gates on the corrected frozen snapshot.
Early Windows/Mac validation attempts were interrupted when these omissions
were found; those logs are preserved separately from the final passing evidence.

The corrected frozen source has completed all six required source gates on the four native targets, Windows ARM64 cross-compilation and nine reviewed synthetic visuals. See the current receipt above.
Preserve the dirty integration until source is frozen and the batch is ready;
do not commit it as releasable or push it to trigger expensive CI prematurely.
At that checkpoint run required source gates, the affected platform coverage,
catalog checks and the relevant native/visual matrix together. New input hashes
must accompany those results; older successful receipts cannot validate changed
camera, call, storage or bidi source.

Keep every original inventory item tracked. Group calling, @all, view-once sends,
native phone custom lists and snapshot omission convergence still require the
documented official APIs. Referenced attachment-cache expiration is outside the
user-selected retention policy. Phone numbers/devices are unavailable for attended
acceptance, and personal accounts are not substitutes. Native permission,
screen-reader/compositor/input/font/GPU and power-loss cases remain open.

Public main is `e303c226e360a7b0731d2734bbba8d375f9f037f`; local checkpoint commit
`c9b5672` records the approved push and deferred builds. Current source changes
have not been committed, pushed, installed or released. No live account, call,
camera, user archive or personal clipboard was exercised.

## Approved main delivery

After all source checks and resource restoration passed, the user explicitly
authorized committing the validated batch to `main`, pushing to
`eternalwaitt/zap-faster`, and checking CI. The 342 canonical validated input
hashes were verified again before staging. The unrelated `packaging/__pycache__/`
remains excluded. Validation statements above describe the pre-publication
checkpoint; this authorization does not tag a release or install the client.

The push must remain linear and include the unpublished checkpoint commit
`c9b5672e`. CI results must be read back for the pushed batch commit, including
fresh source, Nix and Flatpak coverage. No older package receipt establishes
current artifact readiness. Phone/native acceptance and the official dependency
prerequisites above remain open regardless of CI outcome. A release also needs
its version/metainfo/lock updates, release notes and media, release builds,
artifact verification and downstream packaging in the AGENTS.md order.