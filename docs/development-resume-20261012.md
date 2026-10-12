# Selected development batch, 2026-10-12

This checkpoint continues the selected batch, not every upstream proposal.
The user's 2026-10-12 instructions supersede historical installation and
synthetic-account-only restrictions in earlier checkpoints. Installation and
attended personal-account tests are authorized, with a recoverable backup and
rollback first. Preserve the session, archive and OS keyring. Live sends/calls
require an agreed chat/recipient and invented content. No destructive tests,
unlink/reset/relink, release tag or publication is authorized.

## Fresh starting evidence

- Local and public fork main: `eb09b23ae29a7576251a8c91790736cc612ee0fd`.
- CI [38180933357](https://github.com/eternalwaitt/zap-faster/actions/runs/38180933357)
  completed with eight successful jobs and two failed Linux Nix jobs. Both fail
  `demo::tests::phone_number_link_opens_its_actions_from_the_keyboard` at the
  focus assertion. Flatpak [38180933389](https://github.com/eternalwaitt/zap-faster/actions/runs/38180933389)
  passed for the same commit. These receipts do not validate later edits.
- Current upstream main after fetch:
  `e00e377ee6c617206245455a04146a5dd5c24265`.
- Only unrelated initial dirty work: `packaging/__pycache__/`, preserved and
  excluded from every commit.

## Keyboard correction

Phone-number hit regions were grouped like a toolbar in `tab_controls`, which
kept just one region. With a wrapped number, backwards navigation from chat
search selected the first region instead of the last expected by the existing
demo regression. Retain every phone region in the Tab cycle, while keeping
arrow movement among those regions. The existing demo assertion is unchanged;
a focused regression verifies first/last and intermediate navigation for two
regions independently of fonts/platform wrapping.

Commit `25e6567` contains this fix. Six focus tests and all 278 demo tests pass
on Windows. Formatting and all-feature all-target strict Clippy pass. Fresh
Linux Nix evidence remains required for the correction.

## Landed upstream comparison

All 32 commits between the common ancestor and fetched upstream main were
reviewed against the existing fork source and selected-batch evidence. No open
proposal was imported just because it exists.

| Upstream commits | Disposition |
| --- | --- |
| `169b11ab`, `2fc8fe6c`, `a7d3f58e`, `b357a0b7`, `d78d1138`, `d818cdfc` | Patch-equivalent changes already in main. |
| `6aecbce0`, `06e41d55`, `6b911c42`, `6e6030e0`, `3178608b`, `e91295ac`, `48f4faf5` | Existing adapted phone links, motion photos, message info, chat links/CLI, preview names and navigation retained. The phone-link focus correction is above. |
| `9f6fe149`, `8b7e1175`, `c3235b6c`, `72669077`, `1643dd18`, `bd98ad56` | Existing screen-reader animation stepping, GNOME attention suppression, pairing URI, account-bound macOS notifications, voice footer and acknowledged manual presence retained. Desktop integration now shares the upstream helper. Native acceptance remains separate. |
| `11a7e975` | Existing full-digit phonenumber parsing and Brazilian formatting retained; covers country-specific grouping without replacing the fork's stronger validation. |
| `95767b01` | Not imported: encodes generated `non_jid_mentions` flags and eligibility here, conflicting with AGENTS.md's prohibition on reimplementing protocol pieces. A landed client commit does not supply the missing official protocol facade. `@all` remains ordinary text. |
| `d256350d`, `eef365c4` | Shared fastframe dependencies updated together to v0.5.1 (`491f7194d555578bf75e2ce9706b2b2b4d42cf31`). Other locked dependencies remain unchanged. |
| `a3b07609`, `2422b5c8` | Shared logical-pixel window geometry, geometry/close handling in the logic pass, and minimized close-to-tray fix adapted. Theme transparency, legacy persistence path and async niri helper retained. |
| `86e0e2cb` | Existing bounded async socket-based niri raising retained, avoiding synchronous compositor work in the interface. The shared dependency carries upstream's implementation too. |
| `22861cb2`, `6434d354`, `6eede808` | Durable settings/account roster/wallpaper writes, desktop portal clock preference, and OS wake events integrated. Wake reconnects every connected account; paused/offline accounts remain gated by worker status. Existing archive/backup durability remains intact. |
| `f04801d4` | Qt-compatible filter-free icon with the upstream shadow strokes integrated, retaining the fork's filenames. |
| `26846fb5`, `e00e377e` | MSI recipe, release artifact/attestation wiring and disposable-runner install/update/uninstall CI adapted. Unique fork MSI GUIDs, fork Inno identity, paths, marker, notification identity and signing key retained. No upstream install/shortcut is taken over. |

Dependency comparison also confirms both clients use the same egui/winit
revisions. Upstream's protocol pin `f7468ae` is an ancestor of our `94c53e8`
(GitHub compare: 130 commits ahead, zero behind). Retain our newer tested pin and
the `voip` feature instead of downgrading the fork's calling/recovery support.

## Validation and remaining acceptance

Windows compilation and 55 focused window tests pass after the shared update.
The default Windows suite passes 1,795 library tests (13 existing ignored), ten executable tests and two updater tests. Both strict Clippy modes pass.
All six Windows gates pass, including the all-feature suite with the same 1,795 library tests, 12 executable tests and two updater tests, and strict rustdoc.
Fresh CI and native acceptance must still be recorded before claiming completion.
Exact inputs and gate receipts are in [the follow-up receipt](verification/development-followup-20261012.json).
Workflow lint, PowerShell syntax, MSI XML parsing and Git whitespace checks pass.
Actual MSI install/update/uninstall must run only on disposable CI runners, not
on the personal Windows profile. Its script refuses ordinary local invocation.

Installation has not happened in this checkpoint. The running personal client
and all account files remain unchanged. Prepare a cold, hash-verified copy after
graceful quit, retain the current executable and same-user keyring, then replace
only the executable. Keep a separate executable rollback and preserve any newer
state before a later data rollback. File backups tied to the existing OS keyring
are machine-local recovery, not portable encrypted exports.

Attended startup/reconnect, phone history/media recovery, storage/cache policy,
voice/video/camera/audio, notifications/tray/privacy, offline backup/restore/
merge and keyboard changes remain unverified until observed with the user.
Second-account/device, fresh-pairing and other-platform hardware cases remain
pending where their prerequisites are absent. No release is ready on source
tests alone. Prepare versioning, notes/media and artifacts only after acceptance
is satisfactory; report concrete readiness before publication.
