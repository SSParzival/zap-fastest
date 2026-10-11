# Official dependency reassessment, 2026-10-11

Read-only remote ref checks and source inspection, with no protocol fork, patch,
live account operation or upstream contact. This assessment supersedes older
statements about whether a currently supported API can unblock the backlog.

## Current revisions

`git ls-remote` returned these public refs during this continuation:

| Repository/ref | Revision | Relationship to tested input |
| --- | --- | --- |
| oxidezap/whatsapp-rust HEAD | `6b29f52b8f3f0c1b7c7614b99ab3a86cb2a5554d` | One commit ahead of the retained Cargo.toml pin `94c53e8` |
| crmne/egui apps-0.36 | `ba6790fe7cf46e58e8d27ce1524cbfdee745e938` | Same as Cargo.toml |
| crmne/winit apps-0.30 | `ed7caa9023f10b397f5b6ec8284a840cbd8a6f65` | Same as Cargo.toml |
| crmne/fastframe HEAD | `bb79dbddef01e660f9cfc37ccd9dff1c299a8d47` | Latest source inspected separately; application remains on v0.4.1 |
| crmne/fastframe v0.4.1 tag object | `a1fe11d036c0b491c6665ad99a308c404ffd9a06` | Cargo checkout resolves to `23d87e0` |

## Required capabilities

Fresh continuation read-back finds exactly one additional official protocol
commit: [share immutable signing keys independently](https://github.com/oxidezap/whatsapp-rust/commit/6b29f52b8f3f0c1b7c7614b99ab3a86cb2a5554d).
The compare API lists only `wacore/libsignal/src/protocol/sender_keys.rs` as
changed. It does not change any of the capability APIs assessed below. The
egui/winit/fastframe refs remain unchanged. No dependency upgrade or build was
performed for this read-only reassessment.

| Backlog | Current evidence | What unblocks implementation |
| --- | --- | --- |
| Group calling, PR #395 | Official [group-call parser](https://github.com/oxidezap/whatsapp-rust/blob/94c53e833805c58c9f5966b08d08c990650a651e/wacore/src/stanza/group_call.rs#L277) still routes initial ACKs through the snapshot parser, whose group-info parser requires transaction-id and connected-limit at lines 383-384. InitialGroupOfferParams has one audio_rate at line 42. High-level group APIs exist, but their presence does not resolve the documented interoperability defects. | Official upstream fixes for initial ACK compatibility and audio negotiation, followed by synthetic-account group-call acceptance. Server error 427 remains unexplained; it is not evidence of an account restriction or a successful call. |
| Mention everyone, PR #418 | [AB property definitions](https://github.com/oxidezap/whatsapp-rust/blob/94c53e833805c58c9f5966b08d08c990650a651e/wacore/src/iq/abprops.rs) include sender/receiver/syncd flags and admin-only group-size threshold. The public feature source has no supported helper combining mention context and current eligibility. | Typed official sender/eligibility API, with edit, quote and forward semantics. Generated fields alone are insufficient. `@all` remains ordinary text. |
| Outgoing view-once, issues #327/#443 | [media.rs](https://github.com/oxidezap/whatsapp-rust/blob/94c53e833805c58c9f5966b08d08c990650a651e/src/media.rs#L24) ImageOptions, VideoOptions and AudioOptions still have no view-once option. Incoming handling and inline protobuf send fixtures do not establish a supported outgoing builder. | Official typed builder plus retention-safe preparation, then app controls and tests that prevent ordinary archive/media export. |
| Native custom lists, issue #371 | Feature/event source has no native chat-list CRUD and snapshot facade. [labels.rs](https://github.com/oxidezap/whatsapp-rust/blob/94c53e833805c58c9f5966b08d08c990650a651e/src/features/labels.rs#L1) expressly calls label_sublist inbound-only. Business labels and status audience lists are distinct capabilities. | Official list identity, membership, ordering, CRUD, full-snapshot and deletion semantics. Local labels/favorites cannot substitute for phone lists. |
| Snapshot omission convergence, issue #437 | The official [AppStateResyncMode::Snapshot contract](https://github.com/oxidezap/whatsapp-rust/blob/94c53e833805c58c9f5966b08d08c990650a651e/src/features/app_state_resync.rs#L48) expressly says snapshots replay present records without reconciling omitted ones. Completion reports dispatch, not handler completion, and an empty collection is ambiguous. | Authoritative collection replacement/completion semantics with ordered delivery and an unambiguous empty baseline, or equivalent supported API. Clearing mirrors first could erase valid state after failure and pending local changes; do not do it speculatively. |
| macOS status-item hiding, issue #457 | Even current [fastframe-tray lib.rs](https://github.com/crmne/fastframe/blob/bb79dbddef01e660f9cfc37ccd9dff1c299a8d47/crates/fastframe-tray/src/lib.rs) exposes menu-entry set_visible, set_icon, is_shown and item creation/attachment, without a public native status-item hide/show API. The item remains privately owned in macos.rs. | Supported status-item visibility API with truthful resident-window availability and retained Dock/reopen behavior. No upgrade is justified solely by this request. |
| macOS unmarked IME commits, issue #447 | Current shared [winit view.rs](https://github.com/crmne/winit/blob/ed7caa9023f10b397f5b6ec8284a840cbd8a6f65/src/platform_impl/macos/view.rs#L392) insertText emits Commit only when marked text exists and IME is enabled. The reported unmarked insertion path is still omitted. | Shared winit/egui input fix, then attended OpenVanilla synthetic-text acceptance without duplicate commits or unintended Enter. |
| Windows Alt codes #432 and macOS accent picker #262 | Shared fork refs have not advanced. Native keyboard/IME handling is present; source alone does not establish the precise cause or prove those behaviors repaired. | Attended synthetic numpad/Alt-code and press-and-hold reproduction on the exact shared revisions, then a fix in the responsible input layer. These remain reproduction-dependent reports, not proven absent protocol APIs. |

No newly supported capability was found for these dependencies. The application's
tested dependency pins and product source remain unchanged. This finding does not
exclude any requested capability from scope.
