# ZapFastest

**Native WhatsApp for power users. Built on the best of ZapFast and ZapFaster.**

ZapFastest is the next-generation fork uniting [ZapFast](https://github.com/crmne/zapfast) (v0.20+) and [ZapFaster](https://github.com/eternalwaitt/zap-faster) (v0.30.0). The mission is simple: keep in continuous lockstep with the core performance and protocol updates of official ZapFast upstream (such as group calls, AEC3 acoustic echo cancellation, and server-time synchronization) while retaining the power-user ergonomics, multi-account isolation, and workflow reliability pioneered by ZapFaster.

ZapFastest is a compact native WhatsApp companion client for Windows, Linux, and macOS, built with Rust, [egui](https://github.com/emilk/egui), [whatsapp-rust](https://github.com/oxidezap/whatsapp-rust), and [fastframe](https://github.com/crmne/fastframe). It links directly to your phone as an authorized companion device. No browser engine, telemetry, hosted cloud servers, or third-party accounts are ever used.

All credit is reserved for the upstream ZapFast and ZapFaster contributors who made this foundation possible. This fork strictly preserves the MIT license, copyright notices, and contributor attribution.

## Build from source

To build from source, use the toolchain defined in `rust-toolchain.toml`:

```sh
git clone https://github.com/SSParzival/zap-fastest.git
cd zap-fastest
cargo run --locked --release
```

Linux builds need the development packages for ALSA, OpenGL, Wayland, and xkbcommon, plus CMake, Clang, and Perl. Windows builds need Visual Studio C++ Build Tools, CMake, LLVM/libclang, and Perl. macOS builds need Xcode Command Line Tools and CMake. See [the build and packaging guide](PACKAGING.md).

## Core Capabilities & Heritage

ZapFastest combines the advances of upstream ZapFast and the ZapFaster fork:

- **Upstream v0.20+ Synchronization**: Integrated group calling, WebRTC audio echo cancellation (AEC3), adaptive video bitrates, server timestamp synchronization, and group member administration.
- **Power User Ergonomics**: Media albums, attachment drafts tied to their chat, crop/rotate before sending, and an archive-backed gallery and media viewer.
- **Resilient Delivery**: Durable waiting sends, conservative recovery after reconnects, and cancellable attachment transfers with progress indicators.
- **Privacy & Transcription**: On-demand local Whisper voice transcripts (100% offline), private group replies, screen privacy guards, and multi-recipient forwarding.
- **Multi-Account Companion**: Isolated local storage, SQLCipher keys, and session databases per linked WhatsApp account.
- Screen privacy, read-receipt privacy, attachment storage usage and notification throttling.
- Group descriptions and disappearing-message timers, keyboard shortcuts and mouse-side navigation.

See [ZapFast's pull requests](https://github.com/crmne/zapfast/pulls) for work proposed to the original project. Source links and author credit for changes we integrate remain in commit history and release notes.

## Existing ZapFast users

Quit ZapFast before starting ZapFastest. The fork intentionally retains ZapFast's existing settings, session, encrypted archive, OS keyring identity and single-instance lock. It uses the existing `zapfast` data directories; installing both does not create independent accounts. Back up your data before switching, and use one application at a time. The fork does not read your archive during builds or development.

ZapFastest checks its own GitHub releases for updates and verifies them with its own Ed25519 signing key. It does not consume upstream ZapFast updates. Update signatures are separate from Windows Authenticode and Apple notarization; download pages state any platform signing limitations.

The [settings and files guide](docs/_reference/settings-and-files.md) describes local storage, encryption and network behavior. GIF search contacts Giphy only when used and configured; preview links and location map links may contact the relevant website when opened. Whisper downloads its 1.5 GB multilingual model from Hugging Face when first requested, verifies its pinned SHA-256 digest, and transcribes on your CPU. Transcripts are stored in the encrypted account archive; model files are in the account cache. No feature uploads message content to a transcription or analytics service.

## Contribute

Help build the WhatsApp client you want to use every day. Bug reports, feature requests, code, testing, translations and documentation are welcome through our [issues](https://github.com/SSParzival/zap-fastest/issues) and [pull requests](https://github.com/SSParzival/zap-fastest/pulls).

**AI-assisted coding is welcome.** A developer must review the code, understand it, test it, and sign off on the submission. Maintainer review and required checks still apply before merging. See [CONTRIBUTING.md](CONTRIBUTING.md) for how to get started and what a good contribution includes.

We welcome compatible changes from ZapFast and can contribute fixes back. When bringing work from another project, link the source and credit its authors.

```sh
cargo run --locked --features demo -- --demo
```

The demo uses synthetic conversations offline. [DEMO.md](DEMO.md) documents its screens and layout checks.

## Disclaimer and license

This is an unofficial client, unaffiliated with WhatsApp or Meta. Using unofficial clients may violate WhatsApp's terms and risk account suspension.

MIT; see [LICENSE](LICENSE) and [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md). Original ZapFast copyright and attribution are retained. Fonts, icons, wallpaper and notification sounds keep their respective licenses.
