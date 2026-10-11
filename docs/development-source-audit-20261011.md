# Original requirement source audit, 2026-10-11

This is a source audit of the exact original 153-item inventory: 66 pull requests
and 87 issues, with no duplicates. Original requirement bodies were compared
with the application source and the implementation/evidence records, rather
than treating a checklist's Covered label as proof. The public requirement
snapshots used were `H:/Scratch/Codex/zap-issues-full.json` and
`H:/Scratch/Codex/zap-prs-full.json`. No personal archive rows, chat content,
contacts, clipboard payloads, live accounts or devices were inspected.

After the additional source omissions below were addressed, the audit identified
no further actionable implementation omission inside the application's current
supported capabilities. This does **not** establish complete acceptance, a
finished backlog, a release-ready build, installed-client behavior or delivery.
Features blocked by missing supported dependency capabilities remain requested
work; they are not product exclusions. Current batch gates and visual receipts
must be recorded separately against the exact frozen inputs.

The detailed requirement inventory remains [the backlog](development-backlog-20261010.md).
The original identifiers are reproduced below to make the 153-item scope
explicit. The [active batch](development-batch-20261011.md),
[upstream API reassessment](upstream-capabilities-20261011.md),
[native integration evidence](native-integration-evidence.md) and
[attended acceptance cases](development-attended-acceptance-20261011.md)
provide the distinct implementation and external-verification records.

## Additional omissions addressed in this batch

| Original requirement | Source behavior now present | Remaining acceptance |
| --- | --- | --- |
| [Community hierarchy #244](https://github.com/crmne/zapfast/issues/244) | Expandable account-local parent/subgroup chat grouping, using supported community metadata and privacy filters. | Phone hierarchy delivery and incomplete metadata; unsupported administration is separate. |
| [Video speed #464](https://github.com/crmne/zapfast/issues/464) | Direct selection of all five speeds, including 1.25x and 1.75x, through the existing playback clock/audio path. | Actual playback and device acceptance. |
| [Full CLI surface #296](https://github.com/crmne/zapfast/issues/296) | Chats, contacts/all, messages, search, account watch, stdin text and repeated file sends, with JSON lines, errors, canonical account/chat scope and durable batch identities. Full long rows and large pages use the bounded fragments described below. | Combined gates and isolated-account interoperability. |
| [Custom notifications #163](https://github.com/crmne/zapfast/issues/163) | Optional standalone Windows corner cards, including resident/hidden operation, shared privacy snapshots, expiry, dismiss and originating-account click targets. | Native delivery, placement, privacy transitions and focus behavior. |
| [Backup/restore #252](https://github.com/crmne/zapfast/issues/252) | Authenticated same-account historical backup merge with a concrete preview, identity checks, atomic application, deletion barriers and preserved uncertainty. | Failure/power-loss cases and approved synthetic-profile acceptance. |
| [Automatic exports #363](https://github.com/crmne/zapfast/issues/363) | Persisted enable-time cutoff makes future-only export the default; historical export requires an explicit option. Private background archival remains independent. | Restart/collision/privacy/export-folder acceptance. |
| [Accessibility #434](https://github.com/crmne/zapfast/issues/434) | Keyboard actions, meaningful AccessKit control/section semantics, toolbars and live announcements with lock/privacy filtering. | NVDA, VoiceOver and platform accessibility delivery. |
| [Audio output #421](https://github.com/crmne/zapfast/issues/421) | Custom notification beeps follow the selected account output route in addition to voice/video/call playback. | Bluetooth/HDMI/default-route changes on actual devices. |
| [Additional client features #454](https://github.com/crmne/zapfast/issues/454) | Undo and read-only delivered Status viewing, with session-only bounded state, revoke/expiry/privacy filtering and explicit media downloads. Safe downloaded images are displayed inline; videos use an explicit system-player action. | Status delivery/revocation/media and keyboard/privacy acceptance. This does not claim Status publishing. |
| [Chat info panel PR #264](https://github.com/crmne/zapfast/pull/264) | Resizable side panel in wide windows, with narrow-window modal behavior. | Final synthetic layout and native usability checks. |
| [Localization PR #394](https://github.com/crmne/zapfast/pull/394) | Previously untranslated viewer/download/action labels are included in gettext extraction. | Current catalog gates and attended translated layouts. |
| [Emoji fonts #439](https://github.com/crmne/zapfast/issues/439) | Local user-supplied supported color-font import, alongside existing system/Noto choices. | Platform/font acceptance; redistribution rights are required before bundling branded assets. |
| [Camera PR #255](https://github.com/crmne/zapfast/pull/255), [camera #219](https://github.com/crmne/zapfast/issues/219) | Per-account selected camera discovery/use, missing-device refusal and aspect-preserving bounded encoding. | Permission, camera removal and actual two-device calls. |
| [Bidirectional paragraphs #248](https://github.com/crmne/zapfast/issues/248) | Explicit paragraphs and their wrapped rows retain paragraph direction while preserving logical copy/link/emoji ranges. | Reported native Hebrew/Arabic font and ink-bound cases remain separate. |
| [Cache growth #204](https://github.com/crmne/zapfast/issues/204) | Default-off age cleanup for unused managed cache files, with bounded scans, archive/job/export protection and fail-closed ownership checks. | Actual storage-policy and restart/failure acceptance, under the chosen retention scope below. |

## CLI wire limits and consistency

The original #296 contract requests full message text or captions. The shared
instance transport's 16 KiB reply limit did not justify dropping rows larger
than 12 KiB: application IPC fragments solve that problem without modifying or
forking the dependency or implementing any WhatsApp protocol.

`src/automation.rs` handles replies above 12 KiB as 6 KiB byte fragments encoded
inside bounded JSON responses. It assembles the complete original JSON page
before the CLI prints any row. This applies to all six read modes, so aggregate
chat/contact/watch pages and individual long message rows both work without
changing their paging cursors. Each page remains at most 25 archive rows and
16 MiB of serialized JSON. There are at most 32 metadata-only fragment sessions,
with random 128-bit tokens, exact original-request/account binding and a fixed
60-second expiry. No complete reply buffer or plaintext file is retained by the
server's fragment registry. Completion removes the session; server shutdown
revokes permits and clears sessions.

Each continuation independently receives current authorization and re-runs the
same bounded read. Its SHA-256 digest and byte length must still match the first
reply. Edits, deletion, expiry, newly private rows or other page changes refuse
the remainder rather than combine different snapshots. The original permit is
retained across fragments, so lock, opt-out or collection-privacy revocation is
permanent for that transfer even if the setting is subsequently restored.
Tokens cannot cross accounts or requests; offsets must advance exactly.

Malformed, oversized, expired, stale or revoked transfers return an error. The
CLI discards its incomplete page, emits no rows from that page, and reports the
error through stderr/nonzero exit status. Earlier completely printed pages remain
valid output. A highly active watch page can therefore require a deliberate
restart after a consistency refusal. The memory/time limits are explicit
resource bounds, not an upstream capability blocker. Synthetic fixtures cover
full long Unicode/control-character rows, aggregate pages for every read mode,
wire bounds, midpoint revocation, stale/cross-account/expired sessions and
invalid/nested/send fragment requests. Current batch gates must validate them.
See [local automation](local-automation.md) for the product contract and limits.

## Remaining supported-API prerequisites

These are identified source/API limitations, not hypothetical safety exclusions.
Exact pinned and latest official source evidence is in the
[dependency reassessment](upstream-capabilities-20261011.md).

| Requirement | Prerequisite |
| --- | --- |
| [Group calls PR #395](https://github.com/crmne/zapfast/pull/395) | Official initial-ACK/group-info compatibility and audio negotiation fixes, followed by real synthetic-account group-call acceptance. High-level group methods alone do not resolve the observed interoperability defects or unexplained server 427. |
| [Mention everyone PR #418](https://github.com/crmne/zapfast/pull/418) | Supported typed sender/eligibility API with current permissions and edit/quote/forward semantics. Generated fields alone do not establish the feature. |
| [Outgoing view-once #327](https://github.com/crmne/zapfast/issues/327), [#443](https://github.com/crmne/zapfast/issues/443) | Official media-builder support, then retention-safe app preparation/controls and acceptance. Current public builders lack the option. |
| [Native custom lists #371](https://github.com/crmne/zapfast/issues/371) | Supported phone chat-list identity, CRUD, membership, ordering, full-snapshot and deletion semantics. Business labels and status audience lists are different features. |
| [App-state omission convergence #437](https://github.com/crmne/zapfast/issues/437) | Authoritative replacement and applied-completion semantics, including an unambiguous empty baseline. Dispatch completion and replay of present records cannot justify deleting omitted mirror state. |
| [macOS tray hiding #457](https://github.com/crmne/zapfast/issues/457) | Shared tray status-item hide/show API with truthful resident-window availability and Dock/reopen behavior. Menu-entry visibility does not hide the privately owned status item. |
| [macOS unmarked IME commits #447](https://github.com/crmne/zapfast/issues/447) | Shared native input handling for the reported unmarked insertion path, then attended OpenVanilla tests without duplicate commits or unintended Enter. |

Community metadata/hierarchy rendering is implemented. Community creation,
administration and complete enumeration cannot be advertised from the current
metadata facade; #244's hierarchy acceptance remains open where delivery is
incomplete. Missing supported capabilities remain future implementation work
once the dependency provides their semantics. No upstream crate was patched,
vendored or forked, and no upstream messages were sent.

## Native reproduction and attended phone prerequisites

The [attended matrix](development-attended-acceptance-20261011.md) specifies
synthetic accounts/content and platform/device evidence. Compilation and offline
fixtures do not establish these behaviors:

- Shared/native input: [Windows Alt codes #432](https://github.com/crmne/zapfast/issues/432),
  [macOS accent picker #262](https://github.com/crmne/zapfast/issues/262), and #447 above.
  The first two require an exact-revision reproduction before assigning the fix
  to the responsible input layer.
- Native presentation/input: flicker [#433](https://github.com/crmne/zapfast/issues/433)
  and [#223](https://github.com/crmne/zapfast/issues/223), touch
  [#425](https://github.com/crmne/zapfast/issues/425), Hebrew/Arabic font rendering
  [#398](https://github.com/crmne/zapfast/issues/398) and
  [#328](https://github.com/crmne/zapfast/issues/328), Linux native borders
  [#397](https://github.com/crmne/zapfast/issues/397), and older-hardware performance
  [#209](https://github.com/crmne/zapfast/issues/209).
- Clipboard/drop behavior: [Windows #404](https://github.com/crmne/zapfast/issues/404)
  and [Snipping Tool recording #323](https://github.com/crmne/zapfast/issues/323).
  Native clipboard format/medium evidence must use synthetic content; personal
  clipboard bytes are not diagnostic input.
- Native delivery/availability: KDE notification history
  [#392](https://github.com/crmne/zapfast/issues/392), tray/Dock/banners
  [#375](https://github.com/crmne/zapfast/issues/375),
  [#368](https://github.com/crmne/zapfast/issues/368),
  [#346](https://github.com/crmne/zapfast/issues/346),
  signed macOS notifications [PR #472](https://github.com/crmne/zapfast/pull/472),
  macOS Focus/Do Not Disturb [#211](https://github.com/crmne/zapfast/issues/211),
  and new native Windows cards #163.
- Native services/devices: Steam Deck/KWallet
  [#261](https://github.com/crmne/zapfast/issues/261), physical audio routing #421,
  camera permissions/removal PR #255/#219, and
  TLS/dual-stack [#271](https://github.com/crmne/zapfast/issues/271).
  A configured connect timeout does not prove a non-timeout IPv6/trust failure
  repaired.
- Actual NVDA/VoiceOver/platform screen-reader delivery for #434, native
  compositor/window lifecycle, font/input/GPU reports, and power-loss behavior.
- Phone pairing/reconnect/history/media recovery, disappearing/keep expiry
  [#191](https://github.com/crmne/zapfast/issues/191), deletion/favorites/stickers/
  receipts/polls synchronization, delivered Status #454, and approved two-device
  1:1 voice/video calls. Synthetic source tests cannot substitute for phone or
  microphone/camera evidence. Group calls additionally require the API fixes above.

These are still acceptance work. A dependency-owned or reproduction-dependent
report is not automatically a permanent product boundary.

## Publication and authorized scope

Current-source gates, catalogs and final synthetic visual checks remain separate
from this audit. Earlier passing receipts apply only to their earlier source
snapshot. Root records current gate/visual results separately.

[Debian 12 packaging #202](https://github.com/crmne/zapfast/issues/202) has local
ABI/package/installation evidence, but updated public release assets remain a
publication prerequisite. Public release publication and installed-client
replacement need their distinct authorized delivery and read-back; source
completeness is not proof that users have the new binaries.

Only these explicit scope decisions are treated as exclusions:

- [PR #315](https://github.com/crmne/zapfast/pull/315): the user explicitly excluded
  non-WhatsApp network adapters. WhatsApp multi-account remains included.
- [Cache #204](https://github.com/crmne/zapfast/issues/204): the user selected
  preservation of every archived attachment and default-off cleanup of old
  **unused** managed files. Referenced-attachment expiration is outside that
  chosen policy, not an inferred exclusion from a download limit or old guide.

Branded emoji asset redistribution requires actual rights before bundling;
local supported color-font import is implemented. This prerequisite does not
exclude alternative emoji rendering as a feature. No other requirement was
removed from scope merely because it was missing, had an old Covered label,
needed a native device or had generated protocol fields without a supported API.

## Exact audited identifiers

### Pull requests: 66

- [#476](https://github.com/crmne/zapfast/pull/476), [#475](https://github.com/crmne/zapfast/pull/475), [#474](https://github.com/crmne/zapfast/pull/474), [#473](https://github.com/crmne/zapfast/pull/473), [#472](https://github.com/crmne/zapfast/pull/472), [#471](https://github.com/crmne/zapfast/pull/471), [#470](https://github.com/crmne/zapfast/pull/470), [#469](https://github.com/crmne/zapfast/pull/469).
- [#468](https://github.com/crmne/zapfast/pull/468), [#467](https://github.com/crmne/zapfast/pull/467), [#465](https://github.com/crmne/zapfast/pull/465), [#463](https://github.com/crmne/zapfast/pull/463), [#462](https://github.com/crmne/zapfast/pull/462), [#458](https://github.com/crmne/zapfast/pull/458), [#455](https://github.com/crmne/zapfast/pull/455), [#453](https://github.com/crmne/zapfast/pull/453).
- [#451](https://github.com/crmne/zapfast/pull/451), [#442](https://github.com/crmne/zapfast/pull/442), [#441](https://github.com/crmne/zapfast/pull/441), [#440](https://github.com/crmne/zapfast/pull/440), [#438](https://github.com/crmne/zapfast/pull/438), [#436](https://github.com/crmne/zapfast/pull/436), [#435](https://github.com/crmne/zapfast/pull/435), [#424](https://github.com/crmne/zapfast/pull/424).
- [#422](https://github.com/crmne/zapfast/pull/422), [#420](https://github.com/crmne/zapfast/pull/420), [#418](https://github.com/crmne/zapfast/pull/418), [#413](https://github.com/crmne/zapfast/pull/413), [#409](https://github.com/crmne/zapfast/pull/409), [#406](https://github.com/crmne/zapfast/pull/406), [#400](https://github.com/crmne/zapfast/pull/400), [#395](https://github.com/crmne/zapfast/pull/395).
- [#394](https://github.com/crmne/zapfast/pull/394), [#391](https://github.com/crmne/zapfast/pull/391), [#388](https://github.com/crmne/zapfast/pull/388), [#366](https://github.com/crmne/zapfast/pull/366), [#365](https://github.com/crmne/zapfast/pull/365), [#364](https://github.com/crmne/zapfast/pull/364), [#359](https://github.com/crmne/zapfast/pull/359), [#357](https://github.com/crmne/zapfast/pull/357).
- [#340](https://github.com/crmne/zapfast/pull/340), [#337](https://github.com/crmne/zapfast/pull/337), [#336](https://github.com/crmne/zapfast/pull/336), [#335](https://github.com/crmne/zapfast/pull/335), [#332](https://github.com/crmne/zapfast/pull/332), [#324](https://github.com/crmne/zapfast/pull/324), [#321](https://github.com/crmne/zapfast/pull/321), [#320](https://github.com/crmne/zapfast/pull/320).
- [#319](https://github.com/crmne/zapfast/pull/319), [#318](https://github.com/crmne/zapfast/pull/318), [#317](https://github.com/crmne/zapfast/pull/317), [#315](https://github.com/crmne/zapfast/pull/315), [#301](https://github.com/crmne/zapfast/pull/301), [#286](https://github.com/crmne/zapfast/pull/286), [#268](https://github.com/crmne/zapfast/pull/268), [#264](https://github.com/crmne/zapfast/pull/264).
- [#256](https://github.com/crmne/zapfast/pull/256), [#255](https://github.com/crmne/zapfast/pull/255), [#236](https://github.com/crmne/zapfast/pull/236), [#234](https://github.com/crmne/zapfast/pull/234), [#232](https://github.com/crmne/zapfast/pull/232), [#220](https://github.com/crmne/zapfast/pull/220), [#207](https://github.com/crmne/zapfast/pull/207), [#194](https://github.com/crmne/zapfast/pull/194).
- [#189](https://github.com/crmne/zapfast/pull/189), [#75](https://github.com/crmne/zapfast/pull/75).

### Issues: 87

- [#466](https://github.com/crmne/zapfast/issues/466), [#464](https://github.com/crmne/zapfast/issues/464), [#461](https://github.com/crmne/zapfast/issues/461), [#460](https://github.com/crmne/zapfast/issues/460), [#459](https://github.com/crmne/zapfast/issues/459), [#457](https://github.com/crmne/zapfast/issues/457), [#456](https://github.com/crmne/zapfast/issues/456), [#454](https://github.com/crmne/zapfast/issues/454).
- [#452](https://github.com/crmne/zapfast/issues/452), [#450](https://github.com/crmne/zapfast/issues/450), [#449](https://github.com/crmne/zapfast/issues/449), [#447](https://github.com/crmne/zapfast/issues/447), [#446](https://github.com/crmne/zapfast/issues/446), [#445](https://github.com/crmne/zapfast/issues/445), [#444](https://github.com/crmne/zapfast/issues/444), [#443](https://github.com/crmne/zapfast/issues/443).
- [#439](https://github.com/crmne/zapfast/issues/439), [#437](https://github.com/crmne/zapfast/issues/437), [#434](https://github.com/crmne/zapfast/issues/434), [#433](https://github.com/crmne/zapfast/issues/433), [#432](https://github.com/crmne/zapfast/issues/432), [#429](https://github.com/crmne/zapfast/issues/429), [#425](https://github.com/crmne/zapfast/issues/425), [#421](https://github.com/crmne/zapfast/issues/421).
- [#412](https://github.com/crmne/zapfast/issues/412), [#411](https://github.com/crmne/zapfast/issues/411), [#410](https://github.com/crmne/zapfast/issues/410), [#408](https://github.com/crmne/zapfast/issues/408), [#405](https://github.com/crmne/zapfast/issues/405), [#404](https://github.com/crmne/zapfast/issues/404), [#398](https://github.com/crmne/zapfast/issues/398), [#397](https://github.com/crmne/zapfast/issues/397).
- [#396](https://github.com/crmne/zapfast/issues/396), [#392](https://github.com/crmne/zapfast/issues/392), [#389](https://github.com/crmne/zapfast/issues/389), [#387](https://github.com/crmne/zapfast/issues/387), [#386](https://github.com/crmne/zapfast/issues/386), [#383](https://github.com/crmne/zapfast/issues/383), [#382](https://github.com/crmne/zapfast/issues/382), [#381](https://github.com/crmne/zapfast/issues/381).
- [#380](https://github.com/crmne/zapfast/issues/380), [#379](https://github.com/crmne/zapfast/issues/379), [#378](https://github.com/crmne/zapfast/issues/378), [#377](https://github.com/crmne/zapfast/issues/377), [#376](https://github.com/crmne/zapfast/issues/376), [#375](https://github.com/crmne/zapfast/issues/375), [#373](https://github.com/crmne/zapfast/issues/373), [#371](https://github.com/crmne/zapfast/issues/371).
- [#368](https://github.com/crmne/zapfast/issues/368), [#363](https://github.com/crmne/zapfast/issues/363), [#362](https://github.com/crmne/zapfast/issues/362), [#361](https://github.com/crmne/zapfast/issues/361), [#351](https://github.com/crmne/zapfast/issues/351), [#346](https://github.com/crmne/zapfast/issues/346), [#331](https://github.com/crmne/zapfast/issues/331), [#328](https://github.com/crmne/zapfast/issues/328).
- [#327](https://github.com/crmne/zapfast/issues/327), [#326](https://github.com/crmne/zapfast/issues/326), [#325](https://github.com/crmne/zapfast/issues/325), [#323](https://github.com/crmne/zapfast/issues/323), [#316](https://github.com/crmne/zapfast/issues/316), [#313](https://github.com/crmne/zapfast/issues/313), [#312](https://github.com/crmne/zapfast/issues/312), [#311](https://github.com/crmne/zapfast/issues/311).
- [#309](https://github.com/crmne/zapfast/issues/309), [#308](https://github.com/crmne/zapfast/issues/308), [#296](https://github.com/crmne/zapfast/issues/296), [#295](https://github.com/crmne/zapfast/issues/295), [#292](https://github.com/crmne/zapfast/issues/292), [#291](https://github.com/crmne/zapfast/issues/291), [#271](https://github.com/crmne/zapfast/issues/271), [#265](https://github.com/crmne/zapfast/issues/265).
- [#262](https://github.com/crmne/zapfast/issues/262), [#261](https://github.com/crmne/zapfast/issues/261), [#252](https://github.com/crmne/zapfast/issues/252), [#248](https://github.com/crmne/zapfast/issues/248), [#247](https://github.com/crmne/zapfast/issues/247), [#244](https://github.com/crmne/zapfast/issues/244), [#223](https://github.com/crmne/zapfast/issues/223), [#219](https://github.com/crmne/zapfast/issues/219).
- [#211](https://github.com/crmne/zapfast/issues/211), [#209](https://github.com/crmne/zapfast/issues/209), [#204](https://github.com/crmne/zapfast/issues/204), [#202](https://github.com/crmne/zapfast/issues/202), [#191](https://github.com/crmne/zapfast/issues/191), [#187](https://github.com/crmne/zapfast/issues/187), [#163](https://github.com/crmne/zapfast/issues/163).
