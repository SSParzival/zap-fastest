# Backlog verification checkpoint

This records local verification of the recovered backlog bundle. It is not a release or attended WhatsApp acceptance record. The implementation inventory and remaining dependencies are in [the backlog](development-backlog-20261010.md); interrupted attempts and recovery details are in [the resume checkpoint](development-resume-20261010.md).

## Frozen source

Final call-layout source SHA256:

- `src/ui/call.rs`: `66badb2e1b383d2d33ed1944fca2b34cc1925c69c74e94c3f9569bb84c40587c`
- `src/demo.rs`: `3fd217fbdb0f9dc1271154b9138226883541615ad1d98967da85ee6933eed672`

All compilation uses pinned Rust 1.98 and normal project profiles. Checks use synthetic fixtures, mock credentials or fresh offline demo state. No personal archive rows were inspected.

The original snapshot contains 390 files. Fresh delivery comparison excludes
`AGENTS.md` (later verification-cadence instructions) and a generated Python cache
file. All 388 remaining product inputs match the snapshot after the accepted
Nix-only corrections: `flake.nix` adds the bindgen hook and uses LF (SHA256
`90916a6010453c03d1874e266f037ac377906973998b7a327e745c9a429203f0`);
`.gitattributes` enforces LF for that file (SHA256
`ba681faa80b75157698a0a013a7a6f5a4b9db8824bea5a192b0a23c58293ce83`).
Subsequent user-guide/history documentation corrections are recorded separately
in the manifest; they change no tested application, dependency, catalog or payload
input.

## Required Rust gates

The six gates are formatting, strict Clippy in default/all-feature configurations, complete all-target tests in default/all-feature configurations, and strict all-feature documentation.

| Platform | Result | Passing library tests per configuration | Ignored | Evidence |
| --- | --- | ---: | ---: | --- |
| Windows x86_64 | All six pass | 1,722 | 13 | `H:/Scratch/Codex/zap-windows-final-gate-status`, `zap-post-linux-fix-gate-*.log` |
| macOS ARM64 | All six and native Settings-menu integration pass | 1,715 | 13 | `H:/Scratch/Codex/zap-macos-post-call-final-verification.md` |
| Linux ARM64, Ubuntu 22.04 | All six and all-feature application build pass | 1,722 | 14 | `H:/Scratch/Codex/zap-arm64-call-final-verification.md` |
| Linux x86_64, Ubuntu 22.04 | All six and all-feature application build pass | 1,722 | 14 | `H:/Scratch/Codex/zap-linux-final-gate-status`, `zap-linux-final-gate-*.log` |

Default executable suites pass nine tests; all-feature executable suites pass eleven. Each update integration suite passes two. Ignored tests retain their manual, hardware or measurement requirements.

Windows ARM64 CI-equivalent `cargo check --locked --all-targets --all-features --target aarch64-pc-windows-msvc -j1` passes. This verifies compilation, not native ARM64 execution. Official SHA-verified Microsoft compiler payloads were extracted into a separate tool directory without modifying installed SDKs or registry. Evidence: `H:/Scratch/Codex/zap-windows-arm64-final-verification.md`.

## Other completed checks

- GNU extraction/reference and format checks pass for all ten catalogs, each with 688 translated active entries and zero empty/fuzzy entries.
- Pinned cargo-audit 0.22.2 passes: 864 dependencies, zero vulnerabilities. Existing informational maintenance notices remain for `atomic-polyfill` and `paste`; no ignore was added.
- Final ARM64 DEB/RPM/AppImage assembly, checksums, glibc 2.35 ABI, AppImage execution, ELF update metadata and SVG rendering pass.
- All six final ARM64 clean installation/removal cases pass: Ubuntu 22.04/24.04, Debian 12/13, Fedora 41/latest. Assertions cover actual package-manager installation, runtime libraries, exact desktop entries, installed Omarchy-hook execution, removal and preservation of synthetic existing settings. Evidence: `H:/Scratch/Codex/zap-arm64-final-install-validation/validation.md`.
- Final immutable ARM64 binary passes 18 independent synthetic X11 captures, including compact/short call controls and the nine CI pages. Evidence: `H:/Scratch/Codex/zap-linux-demo-20261010/final/REPORT.md`.
- Final immutable x86 binary passes all nine CI page captures, each with process exit 0, a fresh valid 1180x780 PNG and visual review. The actual interface font is DejaVu Sans; software X11 behavior is not equated to ARM fallback fonts or Wayland. Evidence: `H:/Scratch/Codex/zap-x86-linux-demo-final/REPORT.md`.
- Final x86 all-feature build/export passes with matching Linux/Windows SHA256 `10dc8c88896e68283bc0305bd2f55527fd37ee933ffc600f8d945cefb273f82e`, version 0.30.0 and maximum GLIBC 2.35. Evidence: `H:/Scratch/Codex/zap-linux-x86-final-verification.md`.
- Final AMD64 DEB/RPM/AppImage assembly, checksums, glibc 2.35 ABI, AppImage execution, ELF update metadata and SVG rendering pass. All six clean installation/removal cases pass with actual exit 0: Ubuntu 22.04/24.04, Debian 12/13, Fedora 41/latest. The runtime-library, exact desktop-entry, installed Omarchy-hook, removal and settings-preservation assertions remain intact. Evidence: `H:/Scratch/Codex/zap-amd64-final-install-validation/matrix/results.json` and `zap-amd64-final-package-build.log`; final artifacts: `H:/Scratch/Codex/zap-amd64-native-final-packages`.

## Completed Nix and host recovery

Actual x86 Nix flake preflight, normal optimized build with full tests, installation and output checks pass. The accepted package is `/nix/store/xxj8dq7ln66sgydnmxgwys6li4lxld7c-zap-faster-0.30.0`. Its executable reports 0.30.0; runtime wrapper, desktop entries, icon and Omarchy template/hook assertions pass. Evidence: `H:/Scratch/Codex/zap-nix-final`.

Two rejected attempts remain separately preserved: the first could not find `linux/videodev2.h`; the bindgen-hook correction resolved that error. The next build and tests passed but installation failed on CRLF shell text. The final LF correction and Git attribute are included in the accepted full retry. Disposable preview checks were not substituted for the actual package gate. The corrected native package-runner retry and all twelve clean installs also passed; rejected runner output was not accepted.

The original WSL configuration is restored byte-for-byte (6 GiB RAM, 2 GiB swap), SHA256 `3fcf23cdaf92cfbf7a5344e17212aa5ac82f0d6f2a36492148adbf9ec8a34a96`. Actual Docker server read-back is `29.7.2 6213468160`; original swap and owned-container limits are verified. Linux build limits are restored to 5/7 GiB memory/combined memory-plus-swap; Nix limits to 4/6 GiB. Owned build containers are stopped, and all 15 unrelated containers running immediately before restoration are verified running again. Inaccessible socket-only runtime directories were preserved, with no factory reset or data/cache deletion. Evidence: `H:/Scratch/Codex/zap-wsl-restoration-final`.

## Acceptance boundaries

Synthetic performance evidence is complete within its stated limits. Windows archive ingestion for 100 chats/10,000 messages improved from 22.65s to 9.53/9.11s after atomic-write optimization, with 18.4 MB fixture peak working set. Three final native ARM64 offline demo samples give a median capture-and-exit startup proxy of 0.73s and median application peak RSS of 176.35 MiB. Fresh app directories have a warm OS cache; this is the normal debug binary under software X11. The durable scheduler fixture allocates exactly five of 60 requests to each of 12 chats, service ratio 1.0, and preserves the exhausted budget after restoration. These measurements do not establish real-account sync, first-window presentation, release-build or old Intel performance. Evidence: `H:/Scratch/Codex/zap-arm64-startup-measurements-20261011/REPORT.md` and `zap-p2-measurement-reconciliation.md`.

Official upstream API/interoperability work remains for group calling, @all, native WhatsApp custom lists, outgoing view-once and shared native input/tray integration. Attended synthetic-account/device checks remain for pairing/history availability, phone-visible presence/deletion/receipts/favorites, voice/video interoperability and permissions, backup power loss, OS notifications, screen readers, compositor/input/font and GPU reports. Software X11 captures do not prove Wayland or physical-device behavior.

## Delivery continuation, 2026-10-11

Fresh inventory validation confirms 66 unique PRs and 87 unique issues, with
evidence descriptions on every item: 48 PRs and 27 issues marked covered, 18 PRs
and 60 issues still unchecked. The ten archive/download acceptance items remain
unchecked. Checkboxes are not converted to attended proof by compilation.

Official source/ref reassessment is in [the dependency evidence](upstream-capabilities-20261011.md).
The protocol and shared egui/winit heads match the tested pins. Current fastframe
source was inspected separately and still lacks status-item hide/show control.
No newly supported API unblocks the documented capabilities.

Accepted native/Nix exit receipts were read back without repeating builds. Nine
small final reports and the AMD64 installation matrix are preserved in
[the durable receipt record](verification/development-receipts-20261011.json).
[The source manifest](verification/development-inputs-20261011.json) preserves
raw and canonical-LF hashes, the original baseline, and documentation-only
post-verification changes. Original scratch reports remain available at read-back.

The integration and packaging were committed directly to local main as `d98487d`
and `17868ea`. Shared schemas, account state, dependent modules and translated
extraction were kept together in the product commit to preserve the exact tested
integration. Packaging has its own commit. No test or hook was weakened.

Documentation review corrected the old default-only audio-output description and
a stale paragraph denying completed-media audits/source-change retries, and added
development-build history/backup/calling instructions. These are documentation
changes to match existing tested behavior, so Rust/package gates remain applicable.

The [attended preview](development-attended-acceptance-20261011.md) names the
minimum two test accounts/phones, isolated profile, approved actions and required
observations. Test accounts/device environments have not been supplied or
authorized for connection. No attended case ran. Actual power-loss work also
needs a disposable machine/VM and a specific approved interruption method.

See [the delivery review](development-delivery-20261011.md) for the local commit
scope and external delivery prerequisites. No push, tag, release, installed-client
replacement, live WhatsApp send/call or outbound message occurred.

### Approved source delivery

The user subsequently approved only the three reviewed source commits. A guarded
fast-forward push publishes `d98487d`, `17868ea` and `595b31f` to fork/main.
Fresh Git remote and GitHub API read-back both confirm
`595b31f5a0a82fe30c0c73594e0f611c8e8b74fe`. The [Packaging validation](https://github.com/eternalwaitt/zap-faster/actions/runs/38141392466)
passes; version-dependent release packaging/install/publication jobs are skipped.
[CI](https://github.com/eternalwaitt/zap-faster/actions/runs/38141392117) remains
running on that exact SHA; formatting, both Clippy checks, Rustdoc and advisories
pass. [Flatpak](https://github.com/eternalwaitt/zap-faster/actions/runs/38141392071)
fails in `v4l2-sys-mit` because bindgen cannot load libclang. The source manifest
now requests the official LLVM 22 SDK extension and sets `LIBCLANG_PATH` to its
library directory. The extension's official branch/26.08 manifest matches our
SDK version. An isolated local Flatpak build validates that packaging correction;
its result must be recorded before claiming it passes. Follow-up checkpoint
changes remain local, outside the approved three-commit range. Release, client
replacement and attended operations are still unapproved/unperformed.

### Follow-up regression correction

Provider macOS default tests pass, but its all-feature suite fails the typing
repaint fixture because cursor blinking requests an immediate frame. A native
Mac reproduction proves that the fixture's pre-frame blink override is lost when
the application reapplies theme visuals. Commit `ddb72b5` keeps the composer
unfocused, checks that prerequisite, exercises light/dark system preferences and
retains the original timing bounds and typing-dot repaint cause. Production
animation behavior is unchanged.

All six required gates exit 0 on native Apple ARM64/Rust 1.98 for corrected
`src/demo.rs` SHA256 `3875c2af9ed1cc32081e473edd625a4fda291d5310f40fc86536a21f7f26f71d`.
Both configurations pass 1,715 library tests with 13 ignored; binary suites pass
9/11 and update integration passes 2 per configuration. Exact captured receipts:
`H:/Scratch/Codex/zap-delivery-20261011/macos-gates/results.txt` and adjacent logs.
The focused corrected test also exits 0. The earlier light-theme reproduction
exits 101 as expected, before the correction. No native window or live account
was opened.

The user confirms no test numbers are available. Phone acceptance remains
blocked on test accounts, with no substitution of personal accounts.


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
