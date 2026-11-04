# ZapFastest 0.34.0 audit

Target: `5f0e37accbff5248240a0a98a9906d19fd18ceaf`, 49 changed files,
8,233 insertions and 320 deletions relative to `709be3e`. Review used repository
source, dependency source already present in the Cargo cache and synthetic tests.
No personal archive, chats, contacts, production logs, session keys or QR payloads
were inspected. The release receipt records the final commands and input hashes.

## Findings corrected

| Finding | Consequence and correction | Regression coverage |
| --- | --- | --- |
| Proxy bypass of the preview DNS guard | Remote proxies could resolve public-looking names to private addresses. Previews now fail closed when a proxy applies, without bypassing that proxy. Direct requests retain guarded resolution, literal-host checks and redirect checks. | Existing local-name/IP/resolver tests; capped HTTP body regression. Proxy configurations are rejected before client creation, including HTTP_PROXY, malformed values and NO_PROXY exemptions that could redirect outside the exempt host. A synthetic environment lookup covers every supported variable without changing process environment. |
| Privacy update did not stop pending work; failed storage kept old permission | Disabling, dismissing, locking, switching accounts or logging out cancels relevant pending tasks. Storage errors and corrupt stored values disable previews. | Disable/dismiss task cancellation and unrecognized stored-value tests. |
| Lottie limits did not bound renderer preparation | A small JSON document can expand compositions or decode a huge embedded raster. Preflight bounds JSON nodes/depth and expanded layers, rejects cyclic assets and parent chains, and refuses raster assets, repeaters and expressions. Existing ZIP, JSON, canvas, duration, frame and output limits remain. | Recursive-asset, parent-cycle, raster/repeater and compressed ZIP-bomb fixtures. |
| Hidden-account export completion touched window state | Export paths could appear in another account's toast. Only the originating progress is cleared while hidden. | Hidden completion preserves the visible dialog and toast list. |
| Export filenames bounded characters but not bytes | Long Unicode names could exceed filesystem component limits. Names now cap UTF-8 bytes and replace traversal sequences. Numeric prefixes retain uniqueness and prevent Windows reserved basenames. | Unicode, traversal, control and separator cases; existing ZIP and cancellation tests. |
| Texture eviction needed another animation draw | Closing the last sticker left frames resident in the context. End-of-frame maintenance drops unseen entries and deliveries; parked pasted images drop their texture handles. | Last-animation closure verifies egui emits texture release. |
| Decoder-slot reservation used a separate load and increment | Concurrent contexts could exceed the decoder limit. Reserve slots with an atomic compare-and-exchange loop; the existing RAII guard still releases them on every exit. | Animation suite and compiler/lint checks. |
| Voice resume converted the saved position through f32 | Long clips could resume with rounding drift. Resume now passes Duration directly and calculates sample offsets with f64. | Long-clip sample offsets and existing pause/resume tests. |
| Uncertain block writes left a stale list until reconnect | A failed acknowledgement can conceal a successful server write. Refresh the server list after failure; reconnect already invalidates generations and rereads it. Never blindly replay a block write. | Existing generation/offline/block boundary tests and full worker suite. |

## Subsystem conclusions

Broadcast support in this commit handles received list messages and migrates
legacy rows. It does not introduce the roster catalog or outgoing broadcast
management described in the previous release notes. Parameterized SQL and an
account-owned connection provide isolation; migration, duplicate reconciliation,
metadata transfer and deletion-barrier handling run in one transaction. Unknown
senders remain in the list. No protocol or recipient-roster parser is duplicated.

Chat export uses a separate verified read-only SQLCipher connection and a
transactional snapshot. It pages 128 messages at a time and copies attachments in
64 KiB chunks into temporary files, publishing only after success and cancellation
checks. Cancellation checks also protect account removal, app lock and chat
removal/locking. The text is Android-style date/time/name output with explicit UTC
and local-only coverage headers, not a byte-for-byte clone of every WhatsApp
platform/locale export. Raw protobufs and media keys are not exported.

Blocking retains the last confirmed list while a request is pending, disables
editing while busy, checks send/call boundaries and resolves privacy IDs inside
the owning worker. It does not optimistically claim server success or maintain a
persistent queue of writes. Reconnection rereads server truth; an uncertain write
now triggers a read immediately too. Blocking views dispatch Actions.

Audio output drops on pause without a grace timer; the saved clock remains in
memory. Chat navigation preserves voice playback; account changes stop it.
Hardware suspension and audible continuity require attended device testing.

## File inventory

The table below covers every file changed by the target commit. It identifies
what was reviewed; successful tests do not prove the absence of all bugs, races,
memory leaks or dependency renderer faults. No fuzzing, Miri, sanitizer run or
cross-platform runtime test is claimed.

| File | Review focus |
| --- | --- |
| `Cargo.lock` | Locked dependency revisions, registry checksums and package version. |
| `Cargo.toml` | Renderer features, approved mp4 fork pin and package identity. |
| `docs/verification/release-0.33.0.json` | Historical receipt coverage and limits; preserved as published evidence. |
| `packaging/flatpak/io.github.eternalwaitt.ZapFaster.metainfo.xml` | Release entry, display identity and stable Flatpak ID. |
| `packaging/release-notes/v0.33.0.md` | Historical claims compared with actual support; clarified in new notes. |
| `src/account.rs` | Account-owned blocklist, preview privacy and export request state. |
| `src/animation.rs` | Off-thread dispatch, slot lifetime, frame budget and unseen texture eviction. |
| `src/app.rs` | Account routing, export callbacks, composer preview lifecycle, audio navigation and actions. |
| `src/archive.rs` | Broadcast index migration, duplicate reconciliation and monotonic preview privacy metadata. |
| `src/archive/broadcast_lists.rs` | Transactional moves, duplicate media, reactions, polls and deletion barriers. |
| `src/archive/chat_export.rs` | SQLCipher snapshot, bounded paging/copy, ZIP names and atomic publication. |
| `src/archive/encryption.rs` | Read-only keyed open verifies the archive without creating or migrating it. |
| `src/audio.rs` | Output release, saved position, playback speed and resumption precision. |
| `src/backend.rs` | Commands/events bind callbacks to worker/account state. |
| `src/backend/picture_edit.rs` | Bounded preview dimensions and premultiplied-alpha conversion. |
| `src/backend/worker.rs` | Privacy event ordering, cancellation, broadcast adoption and protocol classification. |
| `src/backend/worker/automation.rs` | Existing call-site adapts the new preview argument; no new automation capability. |
| `src/backend/worker/blocking.rs` | Generation guards, offline behavior, send refusal and uncertain-write reconciliation. |
| `src/backend/worker/calls.rs` | Blocked-recipient rejection before call setup. |
| `src/backend/worker/chat_export.rs` | Picker/worker lifetime, cancellation, account data destination checks and stale callbacks. |
| `src/backend/worker/link_preview.rs` | DNS/literal/redirect SSRF guards, proxy behavior, body and image decoding caps. |
| `src/backend/worker/link_preview_setting.rs` | Unknown privacy state, snapshot ordering, storage failures and logout generations. |
| `src/backend/worker/outgoing.rs` | Final destination-writability check includes canonical blocklist identity. |
| `src/blocking.rs` | Account-local confirmed list, pending/generation transitions and person eligibility. |
| `src/demo.rs` | Synthetic attachment layout assertions updated to the new send placement. |
| `src/demo/queued.rs` | Synthetic command construction supplies no preview. |
| `src/demo/tour/session.rs` | Synthetic command matching remains compatible with preview fields. |
| `src/image_preview.rs` | Receive-only Lottie detection avoids reading ZIP data on the UI thread. |
| `src/lib.rs` | Module exports for blocking and Lottie. |
| `src/lottie.rs` | ZIP/JSON bounds and pre-render document complexity checks. |
| `src/markup.rs` | Quote marker requires a following space; literal greater-than regression cases. |
| `src/model.rs` | Local capability types, media-path merge and user actions. |
| `src/notify.rs` | Account label honors sender/content masking. |
| `src/proxy.rs` | Proxy helper visibility; removed unused raw-credential URL accessor. |
| `src/safety.rs` | First-link punctuation handling and external URL validation. |
| `src/ui/accounts.rs` | Wheel account actions and gesture ownership checks. |
| `src/ui/attachment_preview.rs` | Bounded pasted-image texture, selected-index clamp and action dispatch. |
| `src/ui/blocking.rs` | Account-specific controls, locked-name masking and action dispatch. |
| `src/ui/chat_export.rs` | Sensitive export confirmation, progress and action dispatch. |
| `src/ui/chats.rs` | Block contact menu action. |
| `src/ui/conversation.rs` | Header folding, voice bar, attachment review, composer context menu and quote deduplication. |
| `src/ui/dialogs.rs` | Blocking/export routes and group-description draft controls. |
| `src/ui/focus.rs` | Attachment review tab stops. |
| `src/ui/image_preview.rs` | Receive-only Lottie actions and preview close hit regions. |
| `src/ui/mod.rs` | New view module wiring. |
| `src/ui/picker.rs` | Lottie is detected as animated; unsupported send/favorite/pack actions are withheld. |
| `src/ui/settings.rs` | Blocked-contact manager dispatch. |
| `src/ui/voice_bar.rs` | Current clip controls, origin navigation and bounded progress fraction. |
| `src/util.rs` | Ordered word search and bounded unread badge rendering. |

## Release and evidence limits

New package resources use the current name. Existing keyring services, user data
paths, gettext domain, Rust library imports, Flatpak ID and Windows installation
identities remain stable and are documented in AGENTS.md and PACKAGING.md.
Published old receipts/notes and upstream credits retain their historical names.
The documentation download menu remains on existing artifacts until publication.

The local receipt hashes its inputs and records actual exit codes; it is not an
Ed25519 publisher signature. The existing release workflow signs downloadable
checksums separately. No signing secret was accessed. GitHub publication, platform
builds, installer acceptance and attended WhatsApp/audio tests are separate from
this local preparation. No screenshots or recordings were added to the repository.
