# Local delivery review, 2026-10-11

The recovered integration is committed locally on main. It is not published,
installed or a completed full-backlog delivery. No source change was needed to
adopt a newly available upstream API because none was found for the documented
blockers. Remaining work is explicit below.

## Reviewable commits

Public fork main was read back as `c5896974215a84fafa2c8ee27a9ee2bdd97b83c0`.
Local work is linear on top of that commit, with no branch or merge commit.

| Commit | Scope | Verification |
| --- | --- | --- |
| `d98487d` | Recovered native product integration: durable encrypted history capture/replay, fair history/media scheduling, organized exports and full account backup publication, official 1:1 calling/device handling, agreed interface/privacy improvements, catalogs and user docs. | Exact frozen product input comparison, all accepted native Rust gates, synthetic regressions/layouts and matching receipts. Shared Action/Command/Event schemas, account state, modules, lockfile and extraction must compile together; no speculative feature-by-feature rewriting of the verified bundle. |
| `17868ea` | Linux ABI/package/update metadata and install validation, native macOS call permissions, actual Nix bindgen/LF corrections and translation-tool setup. | Matching accepted workflow/packaging checks, twelve clean distro installs, ABI/ELF/Qt checks and actual Nix release build/full tests/install/output receipts. |
| Documentation checkpoint commit | Scope inventory, current upstream reassessment, preserved source/receipt manifests, attended acceptance preview, user-guide corrections and resume checkpoint. | Inventory cardinality/uniqueness, document links/JSON, staged diff and unchanged functional inputs. |

All paths were classified before staging and staged through explicit path lists.
The generated `packaging/__pycache__/` file remains untracked. No unrelated file
was reset or deleted. Git's LF normalization was checked against working inputs;
the physical tested files remain preserved.

Review emphasized account/hidden-event ownership, app/chat locks, deletion/export
barriers, revoked automation permits and stable request IDs, capture cleanup,
backup quiescence and credential-owned rollback, durable replay/write failures,
and bounded native device/media work. Existing synthetic regressions cover those
boundaries. This review does not establish phone interoperability or hardware
acceptance. Documentation corrections address two stale descriptions discovered
in review without altering the implementation.

## Status of the full goal

| Dimension | Current result |
| --- | --- |
| Inventory assessed | 66 PRs and 87 issues retained, 48/27 marked covered and 18/60 unchecked; ten additional archive/download acceptance entries remain unchecked. |
| Implemented | Integration is in local source, including durable archive/history/media recovery, backup/restore, 1:1 calls and the agreed UI/privacy work. Partial features retain their detailed limitations. |
| Locally verified | Accepted required native Rust gates, Windows ARM64 compilation, twelve native package install/removal cases, 27 synthetic captures and actual x86 Nix gate. Fresh source/receipt read-back, with no unnecessary rerun. |
| Attended verified | No new case. Phone availability/sync, hardware calls/permissions, native notifications/input/accessibility/compositors/fonts/GPUs and power loss remain unverified. |
| Delivered | Local commits only. Public source, installed client and release assets are unchanged. Current version is still 0.30.0; local test artifacts are development builds, not a new 0.30.0 release. |
| Blocked | Supported upstream APIs/interoperability and specific native reproduction; test accounts/devices and explicitly approved test operations; external source/release/client delivery approval. |

## External delivery preview

The first external action to approve is a normal fast-forward push of the reviewed
local commits to **eternalwaitt/zap-faster main**, through remote `fork`. It runs
the existing main CI. The checked workflow set has release builds on `v*` tags;
the proposed main push does not create a release tag. Re-read public main and
preserve a linear commit range immediately before pushing. If public main moved,
reassess the exact range instead of forcing or overwriting it.

A stable release remains blocked by the recorded acceptance and dependency scope.
If a development prerelease is later requested despite those limits, agree on its
explicit limitations and version first. Version bump, lockfile build, Flatpak
metainfo, final gates for changed inputs, notes, contributor/report credits and
synthetic media are then prepared for that exact version. The previous two stable
Spotifast release bodies (v0.12.0 and v0.11.2) were read for the required notes style.
No final notes or nonexistent asset links were published.

After release approval, push the release commit to main before tagging, verify its
public reachability, await every platform asset/checksum/signature/attestation,
upload the approved synthetic screenshots and verify all public URLs, then update
download/version docs and native recipes. Existing tested artifacts still report
0.30.0, so they must not be relabeled as a new version. Installed-client replacement
requires a separate exact artifact/hash preview and approval, followed by actual
installed version/read-back. Neither CI nor a saved package proves installation.

## What unblocks remaining work

- Protocol/shared APIs: the exact [dependency requirements](upstream-capabilities-20261011.md) must be supported officially. No upstream outreach is authorized.
- Attended runtime: identify two test accounts/phones and available native platforms, then approve the exact [test preview](development-attended-acceptance-20261011.md). Only those accounts/actions are in scope.
- Power-loss: provide a disposable test environment and approve its interruption method; this host's unrelated services remain untouched.
- External source delivery: approve the concrete local commit range for the fork's main. Release/client deployment remain separately reviewable actions.

The full goal stays open. Builds are not used to close device, upstream or
publication requirements.
