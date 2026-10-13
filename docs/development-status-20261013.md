# Selected backlog status, 2026-10-13

This is the current worklist. The October 10 inventory records historical PRs
and issues; its unchecked boxes do not mean every proposal is authorized work
or that the corresponding feature is missing. The active scope is the selected
high-value batch, compatible changes landed on upstream main, reported defects
and their acceptance. Implementation, source verification and attended acceptance
are separate states.

## Current delivery

- Public fork main and installed/verified implementation baseline:
  `2a41af748ec173ee45fc76ae99be83f3cf4f848f`.
- Upstream main read back today: `e00e377ee6c617206245455a04146a5dd5c24265`, unchanged
  since the 32-commit compatibility review.
- All six required Windows checks passed for that baseline: 1,806 library tests, 13 existing
  ignored tests, 10 default / 12 all-feature executable tests and two updater tests.
- [Source CI 38338443966](https://github.com/eternalwaitt/zap-faster/actions/runs/38338443966):
  eight jobs passed, including Windows/macOS/Linux tests, both Linux Nix builds,
  Windows ARM64 compilation, quality and dependency checks. macOS ARM64 Nix is
  still running. The original Linux Nix keyboard-focus failure is fixed without
  weakening its assertion.
- [Flatpak 38338474925](https://github.com/eternalwaitt/zap-faster/actions/runs/38338474925)
  passed for the same source commit.
- Updated Windows client installed and hash/version verified. Version is still
  0.30.0; installed SHA256 is
  `721f18779952918603cd13a26c296042e2da78950cb951f7b2993b4a6b057f0a`.
  A verified cold rollback preserves the previous executable, 5,837 profile files,
  the existing linked session and same-user keyring. No release was tagged or published.

The exact inputs, commands and installation receipt are in
[call-start verification](verification/development-call-start-20261012.json).
New uncommitted UI source and synthetic test edits are now present locally under
[the active UI/UX batch](development-ux-20261013.md). They are not installed and
are not validated by the baseline receipts above. Preserve this concurrent work.
Earlier package receipts describe their own snapshots, not newly built release
artifacts for this head. Documentation acceptance updates remain local while CI
finishes, avoiding a documentation push that cancels the source run.

## Compatible landed upstream work

All 32 upstream commits were reviewed; compatible behavior was integrated or
already covered by the fork. There is no newly landed upstream-main backlog to
import at the verified upstream head. The detailed dispositions are in
[the upstream comparison](development-resume-20261012.md#landed-upstream-comparison).

| Landed area | Current disposition |
| --- | --- |
| Shared fastframe helpers | Updated together to v0.5.1; retained the newer tested whatsapp-rust pin and calling feature. |
| Window geometry, minimized close-to-tray, niri raising | Integrated compatible shared behavior, preserving fork geometry, transparency, legacy paths and asynchronous compositor handling. Actual niri acceptance remains separate. |
| Durable settings, account roster and wallpaper writes | Integrated, preserving archive/backup durability and account isolation. |
| OS wake reconnect and desktop clock preference | Integrated; paused/offline accounts retain their gates. Native sleep/wake acceptance remains pending. |
| Windows MSI and installer lifecycle | Fork-scoped packaging, identities and CI integrated. Disposable-runner lifecycle and user-installer packaging passed in run 38293268332. Current release artifacts still need final release preparation. |
| Qt-compatible icon | Integrated with upstream shadow strokes and fork filenames. |
| Phone links, motion photos, message info, chat links/CLI and previews | Existing compatible fork adaptations retained; wrapped phone keyboard focus corrected. |
| Presence, voice footer, macOS notification routing, pairing URI, GNOME attention and screen-reader animation | Existing compatible implementations retained. Relevant phone/native acceptance remains pending. |
| Country-specific phone formatting | Existing stronger parsing/validation retained. |
| Landed manual `@all` encoding | Intentionally not imported: conflicts with the repository's prohibition on implementing protocol pieces here. It is a dependency track, not an omitted compatible integration. |

## Selected features: implementation versus acceptance

| Area | Source state | Remaining evidence/work |
| --- | --- | --- |
| Existing-session startup/reconnect | Implemented and installed | Prior attended startup connected without QR and retained history. Check current reconnect/sleep/wake; do not confuse history presence with complete recovery. |
| Durable older-history/media recovery and progress | Implemented, source checks pass | Phone response observed; gap repair, restart/resume, older/expired media and real progress behavior need attended acceptance. |
| Organized downloads and filename policies | Implemented, source checks pass | Invented document saving passed. Account/chat/date organization, automatic future-only exports and collision behavior need runtime verification. |
| Storage budget and unused-cache protection | Implemented, source checks pass | Verify budget pause/resume and protection using safe fixtures/offline copies. Referenced archives remain protected; general attachment expiry is outside the chosen policy. |
| Encrypted backup/restore | Implemented | Native encrypted export and separate offline restore passed. Complete media equivalence and additional failure/platform cases remain unverified. Never resume a duplicate restored session merely to test. |
| Historical backup merge | Implemented; privacy-ID duplicate defect fixed in `5115683f` | Native cancellation preserved database/WAL hashes. Recheck corrected same-backup preview and safe confirmation in offline account 2, then return to active account 1. |
| Voice/video/camera/audio selection | Implemented | User reported successful voice/audio on the earlier candidate. Current startup wait feedback accepted. Connected-call timing, ringback, chat/history entries, outside dismissal, video/camera selection, permissions/removal and selected audio routing remain pending. |
| Settings and keyboard/accessibility | Focused fixes landed, source checks and eleven synthetic confirmation captures pass | Prior Settings keyboard navigation passed. Current native layout remains unconfirmed; the label-only audio-layout probe was incomplete. Screen readers, touch, RTL fonts and platform input need their own evidence. |
| CLI, notifications, tray, account/privacy and other selected controls | Implemented | Tray reopening passed. Approved CLI transport/fixture commands, incoming notifications/click routing, presence, privacy/lock boundaries, focus filtering and badges need attended acceptance. |
| UI copy/localization | Earlier translated snapshot verified; follow-up copy added | Refresh extraction and translations for the new Settings/Calls/startup strings, then check formatting and language layout. |
| Broader Impeccable UI/UX pass | In progress locally, with a whole-surface inventory and source edits recorded in the active UX checkpoint | New account/menu/dialog/gallery/login/picker/backup/lock/accessibility fixes require their own checks, bounded visual review, commit/push, installation and native acceptance. Do not describe the whole interface as finished. |

Native passes belong to the observed builds in
[attended acceptance](development-attended-acceptance-20261011.md), not automatically
to every subsequent binary. No agent inspected personal archive contents or
autonomously called contacts.

## Remaining work, in order

- [ ] Finish the last macOS ARM64 Nix job and record the final source CI result.
- [ ] Refresh the new UI/call copy in POT and translated catalogs. Five checked
  labels are absent from the POT and all ten PO catalogs: Recent calls; Audio
  and camera devices; Microphone, sound output and camera; the pre-connection
  wait message; the account-privacy call rejection. Earlier 688-entry receipts
  do not cover these follow-up strings.
- [ ] Complete the agreed connected voice-call acceptance: immediate preparation,
  actual startup time, tone stopping on answer, conversation/history entries and
  outside-click/Escape dismissal. Timing diagnostics contain phase/duration only.
- [ ] Exercise video and selected microphone/speaker/camera with a willing peer,
  including permission/missing-device behavior. No agent-initiated contact calls.
- [ ] Verify downloads, recovery progress and storage/retention controls with
  invented fixtures or safe offline profiles, preserving all personal data.
- [ ] Recheck corrected offline merge; retain active account/session and rollback.
- [ ] Complete native notifications, presence/tray, privacy/account protections,
  keyboard/accessibility and relevant platform acceptance. Respect absent
  device/recipient prerequisites rather than substituting source compilation.
- [ ] Finish the active broader UI/UX pass, with its frozen-source checks,
  bounded synthetic visual rounds and attended verification of meaningful changes.
- [ ] Once acceptance is satisfactory, prepare version/metainfo/lock updates,
  release notes and synthetic media, final artifacts/checksums and affected
  packaging checks. Report readiness before any tag/publication.

## Deferred capabilities and evidence-dependent reports

These remain tracked rather than silently marked complete. They are not a reason
to implement every open proposal or delay this batch until all upstream work exists.

- Protocol/API tracks at the tested pins: group-call interoperability, supported
  `@all`, outgoing view-once, native phone custom lists, snapshot omission
  convergence and community administration. Read-only community metadata and
  incoming view-once handling do not establish the missing capabilities.
- Shared native integration: macOS tray visibility, unmarked IME commits/accent
  picker and Windows Alt codes. Reassess the responsible official/shared APIs
  before changing dependencies; do not vendor or patch upstream crates here.
- Reproduction/device evidence: Windows flicker/crash, drag/drop and clipboard
  virtual video formats, Arabic/Hebrew ink clipping, recipient-name delivery,
  corporate TLS/IPv6 startup, KDE/GNOME notification/window behavior and older
  hardware performance. Source mitigations and synthetic measurements are not
  blanket resolutions of these reports.
- Fresh pairing, physical power-loss durability and unavailable native-platform
  hardware cases remain pending prerequisites. Do not relink the personal account
  or run destructive tests to obtain them. Non-WhatsApp networks remain excluded.

**Readiness:** implemented and installed development candidate with substantial
passing source and partial native evidence. Not release-ready yet. The remaining
worklist above, not the historical unchecked inventory count, governs completion.
