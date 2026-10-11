# Local delivery review, 2026-10-11

The recovered integration is committed on main and its three reviewed commits
are published to the fork after explicit approval. It is not installed, released
or a completed full-backlog delivery. No source change was needed to
adopt a newly available upstream API because none was found for the documented
blockers. Remaining work is explicit below.

## Reviewable commits

Public fork main was initially read back as `c5896974215a84fafa2c8ee27a9ee2bdd97b83c0`.
After approval, the normal fast-forward push publishes the three commits below.
Both `git ls-remote` and GitHub's commits/main API read back
`595b31f5a0a82fe30c0c73594e0f611c8e8b74fe`. History is linear, with no branch,
merge, force push or tag. Follow-up receipt/checkpoint changes remain local to
respect the exact approved three-commit range.

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
| Delivered | Three approved source commits published at `595b31f`; packaging validation passed. Original CI finished with the known macOS fixture failure; Windows tests passed but its job was cancelled during cache cleanup. The corrected macOS fixture passes all six native gates, and the corrected Flatpak source build/export/bundle exits 0 locally. Both corrections remain local pending follow-up push approval. Installed client and release assets are unchanged. Current version remains 0.30.0. |
| Blocked | Supported upstream APIs/interoperability and specific native reproduction; test accounts/devices and explicitly approved test operations; release/client delivery approval. |

## External delivery preview

The approved external action was a normal fast-forward push of the exact three
reviewed commits to **eternalwaitt/zap-faster main**, through remote `fork`.
The pre-push guard confirmed unchanged local/public heads, no tracked worktree
changes and no merges. Existing CI, Packaging and Flatpak validation workflows
were triggered. No version was supplied to Packaging, so its release assembly,
installation matrix and AppImage publication jobs are skipped; its user-installer,
ELF fixture and Qt icon validation passed. This is validation, not release delivery.
Release builds still require a `v*` tag, and no tag was created.

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
- Attended runtime: the user confirms no test numbers are available. Supply two test accounts/phones and identify available native platforms, then approve the exact [test preview](development-attended-acceptance-20261011.md). Personal accounts remain untouched.
- Power-loss: provide a disposable test environment and approve its interruption method; this host's unrelated services remain untouched.
- Source delivery: the exact three-commit range is approved and published. Follow-up checkpoint changes stay local. Release/client deployment remain separately reviewable actions.

The full goal stays open. Builds are not used to close device, upstream or
publication requirements.

## Restart recovery and follow-up preview

The restart preserved all source edits, local commit `ddb72b5` and the completed
Flatpak receipt. The saved build exits 0, compiles camera bindings, composes
AppStream metadata and exports OSTree commit
`0b31dfe8d29b8cc754120b63a3691fe49df4751983a0651fc37db14e28250aaf`.
Its generated bundle SHA256 is
`65baa480abe3d9d681b72f701fb3ec9134d6c2fab983bf260a888208869c1430`.
This is local build/package proof, not installation or camera acceptance.

Fresh input comparison confirms only the reviewed demo fixture and Flatpak
manifest differ from the 388 recorded functional inputs; 386 remain unchanged.
All six native Mac gates pass for the exact corrected demo hash. The immutable
baseline is preserved; [follow-up receipts](verification/development-followup-20261011.json)
record the new hashes, native gate outcomes and final original-provider status.
Linux tests, quality/advisories, Windows ARM64 compilation and all three Nix jobs
passed on the public snapshot. Windows test suites passed before cache cleanup
was cancelled; the complete Windows job is not claimed successful.

The reviewable next external action is one normal fast-forward push of the local
follow-up range after `595b31f` to `fork/main`: the macOS fixture correction,
the Flatpak LLVM build-tool correction and this recovery/receipt checkpoint.
Approval previously covered exactly the original three commits, so this new
range needs explicit approval. Recheck public/local heads and exclude merges
before pushing, then verify the new CI and Flatpak terminal outcomes. No tag,
release, installed-client change or live-account action is part of this preview.
