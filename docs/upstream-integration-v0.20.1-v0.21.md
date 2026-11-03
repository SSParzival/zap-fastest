# Upstream integration: 0.20.1 and 0.21

The comparison covers `c56a631..upstream/main`, fetched at
[`92f5eca`](https://github.com/crmne/zapfast/commit/92f5eca), including upstream's
0.21.1 follow-ups. The implementation scope is the mission's A–O list below.
This is an adapted integration, not a merge of every later upstream feature or
release, and ZapFastest keeps its own version and packaging.

| Requirement | Upstream source | Integration |
| --- | --- | --- |
| A: Account notification mute | [0cf42bc](https://github.com/crmne/zapfast/commit/0cf42bc) | Each switcher row has an accessible bell. It changes that account's existing `notifications` preference, without switching accounts. Muted counts stay visible in grey, but leave process badges and the avatar dot. The local field is named `notifications`, not `notifications_desktop`. |
| B: Accessible message list | [a4db616](https://github.com/crmne/zapfast/commit/a4db616) | Spoken labels and `list_nav` are adapted to the local calling model, transfer states, transcript virtualization, selection and focus order. One ListBox owns focus; drawn options carry position/count and the active descendant. Remembered cursor keys include both account and chat. Arrows, Home, End and page keys move within the list; pointer interaction keeps its existing selection and drag behavior. |
| C: Download organization | [9f46f13](https://github.com/crmne/zapfast/commit/9f46f13) | Organized copies use the account phone number and a readable chat folder. Atomic archive reservations disambiguate names with numeric suffixes, survive restarts and renames, and follow privacy-ID canonicalization. The local verified-copy and durable-export machinery remains in use. New configurations default to account/chat organization; saved flat/monthly choices remain supported. Unlike upstream's rename behavior, renames keep the assigned folder as requested. |
| D: Unheard voice notes | [919e606](https://github.com/crmne/zapfast/commit/919e606) | Incoming unheard notes show an accent dot. Local playback and phone played-self receipts persist heard state. Early receipts and history updates also retain it; playback runs skip notes heard on the phone. |
| E: Private played receipts | [5e8ae88](https://github.com/crmne/zapfast/commit/5e8ae88) | The protocol library sends played-self only after its privacy snapshot confirms public receipts are disabled. Unknown privacy withholds the send. ZapFastest's explicit Send read receipts preference still gates network receipts; local heard state is independent. |
| F: Missing/corrupt stickers | [6a1562d](https://github.com/crmne/zapfast/commit/6a1562d) | The view reports unreadable media through an Action. The app drops image/animation decode failures and retries once per account/message/session using the automatic 64 MiB path. A fresh file that still fails offers a manual retry, avoiding a download loop. This extends upstream's corrupt-file behavior, which only offered a manual retry. |
| G: Encrypted historical edits | [6eff2f4](https://github.com/crmne/zapfast/commit/6eff2f4) | whatsapp-rust opens sealed edit snapshots using the original sender's phone/privacy identities and secrets from the chunk, archive or entry. The original target ID, envelope timestamp/order, local media paths and newer edits survive filing/replay. Diagnostics contain counts only. |
| H: Old starred messages | [5f30e4b](https://github.com/crmne/zapfast/commit/5f30e4b) | Each worker requests a one-time RegularHigh snapshot. An unfinished replay retries after reconnect; timestamped local star/unstar state takes precedence over older replayed actions. Recovery metadata lives in that account's encrypted archive. |
| I: Video containers | [34a6187](https://github.com/crmne/zapfast/commit/34a6187) | Inline video requires MP4 MIME and a successfully parsed MP4 container, excluding QuickTime's major brand even when mislabeled. Other videos use the existing document path with their filename/MIME. No transcoding is introduced. |
| J: Mute submenu | [0d57696](https://github.com/crmne/zapfast/commit/0d57696) | Mute durations share one submenu; a muted chat has one Unmute action. Local sound, VIP and voice-speed controls remain. |
| K: French numbers | [ebb34c7](https://github.com/crmne/zapfast/commit/ebb34c7) | Metropolitan, service and overseas formats use the upstream pair grouping. Existing Brazilian and general formatting remain. |
| L: Ctrl+End | [f5681c8](https://github.com/crmne/zapfast/commit/f5681c8) | A nonempty focused text field retains its editor shortcut. Empty composers and non-editor focus can still jump to the newest message. |
| M: Per-chat history timeouts | [e297ede](https://github.com/crmne/zapfast/commit/e297ede) | The fork uses `background_history`, not upstream's `prefetch`. Its per-chat ownership/backoff already isolates unanswered chats. Regression coverage verifies another chat runs at the normal pace and late replies retain their original owner. |
| N: Unsent cumulative receipts | [5f2ae4f](https://github.com/crmne/zapfast/commit/5f2ae4f) | The existing `status >= Sent` SQL guard is retained. Expanded tests cover queued, unconfirmed, pending and failed sends, other chats and monotonic delivery/read/played advancement. |
| O: Dependency policy | [07fdd86](https://github.com/crmne/zapfast/commit/07fdd86) | Tokio is pinned exactly to the requested 1.53.3. The starting lockfile contained 1.54.1, so this is an explicit requested pin rather than a claim that 1.53.3 is newer. The existing rodio commit is unchanged and resolves through the approved crmne repository. The upstream source checker is restored and runs in CI. |

Original upstream contributors include Carmine Paolino, eternalwaitt, ABDULAZIZ
ALSHMASI, Rodolfo Michetti and Alexandre Augrain; the linked commits retain the
original implementation and attribution history.

The initial working tree is preserved in `6b52818`, including Linux PRIMARY
selection/middle paste, NoFlush/B-frame timing and WebP fixes, emoji-picker
dismissal, reactive contact names and letterbox dismissal. The durable outgoing
queue, 2 GiB explicit versus 64 MiB automatic transfers, archive gallery, account
storage/keyring separation and compact calling/device switching remain in place.
No personal archive was inspected, no protocol was reimplemented, and translation
templates/catalogs were not regenerated.

## Verification

The baseline `cargo check --bin zap-fastest` passed before integration. All Cargo
commands use `CARGO_TARGET_DIR=/tmp/cargo-target`; because `/tmp` is memory-backed,
that path points to a dedicated disk-backed build cache. Tests use synthetic
fixtures and mock/in-memory archives. The tested source is `366dde2` on Linux
x86_64 with Rust/Cargo 1.99.1. The
[checkpoint receipt](verification/upstream-integration-v0.20.1-v0.21.json) records
commands, exit codes, test summaries and synthetic capture hashes.

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| Dependency source policy | Passed |
| `cargo check --locked --all-targets` | Passed |
| Clippy, default and all features, `-D warnings` | Both passed |
| `cargo test --locked --lib` | 2,001 passed, 14 ignored |
| Tests, all targets, default features | 2,001 library + 12 CLI + 2 integration tests passed; 14 ignored |
| Tests, all targets, all features | 2,001 library + 14 CLI + 2 integration tests passed; 14 ignored |
| Rustdoc, all features, `RUSTDOCFLAGS=-D warnings` | Passed |
| Native synthetic captures | Muted account switcher and voice notes inspected at 1100×780 |

The ignored tests are existing fixture/manual probes. Initial test failures
identified old Tab-order assumptions, nonexistent “downloaded” image fixtures,
and an upstream invalid-number fallback expectation. The fixtures now use real
synthetic image files, navigation assertions include the message list, and the
local invalid-number fallback is preserved. The remaining wrapped-phone-link
ordering issue was fixed and its original bidirectional interaction test passes.
No lint or test was disabled to obtain these results.

Captures and full logs remain outside the repository in the local integration
cache. No release or live-account action was needed for these checks. Windows
and macOS were neither compiled nor run here; native device acceptance and live
WhatsApp behavior still require attended testing.
