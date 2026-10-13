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
  completed with seven successful jobs and two failed Linux Nix jobs. Both fail
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
on Windows. Formatting and all-feature all-target strict Clippy pass.
Both Linux Nix builds passed in source CI `38293266976`, including the unchanged
demo regression. All nine source jobs subsequently passed for `ac8c93f3`;
Flatpak and Packaging CI also passed. These receipts do not validate the newer
historical merge correction described below.

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
Actual MSI install/takeover/update/uninstall and native-packages generation passed
in [Packaging CI 38293268332](https://github.com/eternalwaitt/zap-faster/actions/runs/38293268332).
The lifecycle test runs only on disposable CI runners; its script refuses ordinary
local invocation. The separate user-installer job also passed.

The Windows release build passed and the executable was replaced after graceful
quit and a complete cold backup: 2,844 files, 3,590,731,648 bytes, individually
hash-verified. The protected machine-local recovery folder is
`C:/Users/Victor/AppData/Local/ZapFasterRollback/20261012-162008`.
It retains the previous executable, profile/cache copies and a hash-checking
executable rollback script. The same-user OS keyring and existing shortcut are
unchanged. Data rollback requires separate review and preservation of newer
state. This backup is not a portable encrypted export.

Victor opened the existing shortcut and confirmed the personal account connected
without QR/relink and existing history was present. This verifies attended
existing-session startup and history presence, not older-history recovery.
Message yourself is the agreed fixture chat. After Victor explicitly asked the
agent to send the three invented fixtures, Windows UI Automation verified the
agreed conversation header and empty composer, selected the exact three files,
verified three attachment tiles and an empty caption, and invoked Send once.
Victor confirmed all three arrived and opened correctly, including audible
three-second video playback. No private message contents or archive rows were
captured. There is no agreed separate call recipient yet; no calls occurred.

Native encrypted backup export subsequently passed: Settings reported success
and the closed 2,312,919,910-byte output was copied and hash-verified in the
protected permanent recovery folder as `attended-account.zfbackup`. Its generated
passphrase is recoverable through current-user DPAPI in the same folder, without
plaintext in reports or clipboard. The native save picker rejected the existing
`C:` folder as nonexistent; protected `H:` staging worked and remains retained.
That mismatch needs further evidence. Reconnect read-back, offline restore and
merge remained pending at export time. Subsequent native read-back confirmed
reconnection. Restore decrypted and installed account `2` with session/archive
present, `archive_only=true` and `cli_access=false`; it was never resumed.
Original account `1` remained connected and was selected again after testing.
Re-importing the same backup into the offline copy proposed three additions;
cancel preserved database/WAL hashes. Synthetic fixtures reproduce incorrect
duplicate additions when only the imported privacy IDs are normalized. A new
correction is committed as `5115683f`, separate from the `ac8c93f3` receipts.
Keyboard automation stops if the client loses foreground.

The user supplied a screenshot showing overlapping/clipped Settings content.
`d73ded1` stacks progress details and backup controls, wraps narrow setting rows,
and confines the scrolling column to its pane. A synthetic layout regression
passes at three widths and two text scales. All six required Windows gates passed
for the combined merge/Settings candidate: 1,798 library tests, 13 existing ignored,
10 default / 12 all-feature executable tests and two updater tests. The frozen
input receipt is `verification/development-settings-fix-20261012.json`.
The release build passed in 13m 19s and the corrected executable was installed
after a second graceful shutdown and full verified rollback backup:
`C:/Users/Victor/AppData/Local/ZapFasterRollback/20261012-182629`, 5,833 files,
5,908,006,154 bytes. Installed SHA256 is
`6df2a96d4db95c6e0c12e5a6562e68183b9e7bf2493e3c6c0e3234e3e23ac088`.
The previous executable and both accounts' profile/cache files were preserved;
keyring entries were unchanged. Version remains 0.30.0; no release was tagged.
Public source head is `03a5b287a2fbc32becc9c64ee4d092a2813e0dc4`.
Source CI `38309276530` passed all nine jobs and Flatpak `38310864553` passed.
The user was asked to reopen the existing shortcut, leave Settings foreground,
and report reconnect/layout. Installed native layout and corrected offline-merge
rechecks remain pending that coordination. The broader UI/calling follow-up is
tracked in `development-ui-calling-20261012.md`, with the latest status below.

Latest follow-up: `3b3b0a53f16006668f50b50d47d61bd195d3c503` is pushed to
fork/main. All six required Windows gates passed (1,803 library tests, 13
existing ignored). The 7m 49s default release build was installed with cold
rollback `C:/Users/Victor/AppData/Local/ZapFasterRollback/20261012-204818`,
5,835 verified profile files and unchanged same-user keyring. Installed SHA256
is `da335552fde13027517d928e5dc6598d78dc485e2ae4ea8a23b5fb52dee40eed`;
version remains 0.30.0. Source CI `38323347352` passed all nine jobs and
Flatpak `38323371017` passed. User reopen/reconnect, Settings fit, Calls dismissal, local tone
and completed-call entries remain pending. Do not scan a QR or relink.

New native report: a call initiated just after opening eventually started after
a long delay. Startup admission and immediate preparation feedback are being
corrected in `src/app.rs` and `src/backend/worker/calls.rs`; the installed build
is still `3b3b0a53`. The exact queue/device/protocol delay is unmeasured.
Focused tests passed (two admission/feedback cases and thirteen call-worker
cases), followed by all six required Windows gates passing: 1,806 library tests,
13 existing ignored, 10 default / 12 all-feature executable tests and two updater
tests. Frozen inputs and commands are in
`verification/development-call-start-20261012.json`. Corrected build/install
and measured attended acceptance remain pending.

Connected Settings read-back observed `Phone history: response received`.
This establishes a response, not complete recovery or a specific repair anchor.
Startup, fixture playback/saving, tray reopening, encrypted export/offline restore
and Settings keyboard navigation have the observations recorded above.
Organized downloads/storage/cache protection, complete history/media recovery,
voice/video/camera/audio, notifications/privacy and corrected offline merge
still need their own acceptance evidence.
Second-account/device, fresh-pairing and other-platform hardware cases remain
pending where their prerequisites are absent. No release is ready on source
tests alone. Prepare versioning, notes/media and artifacts only after acceptance
is satisfactory; report concrete readiness before publication.
