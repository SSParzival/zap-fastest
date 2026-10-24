# Upstream Integration Review: ZapFast v0.20.0

- Upstream HEAD: `c56a631` (ZapFast v0.20.0)
- Fork Baseline: `3792246` (ZapFaster v0.30.0)
- Common Ancestor: `df01459f` (7 October 2026)
- Comparison Scope: 228 upstream-only commits since common ancestor, 117 commits since October 13 integration review.

---

## 1. Executive Summary

Upstream ZapFast has reached release **v0.20.0**. Key architectural shifts introduced upstream include:
1. **Group Calling & Media Engine**: Experimental group calling, direct WebRTC AEC3 (Acoustic Echo Cancellation) tuning, capture timestamp synchronization for audio/video lip-syncing, adaptive bitrate pacing, and FEC/repair packet insertion.
2. **Group Administration**: Native group member management from group info (invite link generation, member promotion/demotion, and permissions), group creation from "New chat", and WhatsApp invite protocol URI handling (`#591`).
3. **App-State & Synchronization**: Placing sent messages at server time (`#574`), syncing "Keep chats archived" setting (`#572`), omitting uncontacted directory contacts from chat list (`#573`), and syncing archive changes back to phone (`#589`).
4. **Fastframe Infrastructure**: Migration to Fastframe 0.13.1, pooling PulseAudio connections, and integrating native notification/window management.

This review maps each upstream capability to its required disposition in **ZapFastest**, highlighting where careful rework is required to maintain ZapFaster's reliability features: multi-account partitioning, local Whisper offline transcription, compact call UI, and durable send mechanics.

---

## 2. Upstream Commit Inventory & Disposition

| Upstream Commit | Description & PR | Architectural Impact | ZapFastest Disposition |
| :--- | :--- | :--- | :--- |
| `c56a631` | ctutils 0.4.5 for RUSTSEC-2026-0385 | Security advisory patch | **Integrate immediately** into Cargo dependencies. |
| `df7a928` / `bc33ccf` | ZapFast 0.20.0 release notes and metadata | Packaging and release docs | **Adapt**: Keep versioning independent (`v0.31.0-alpha.1`). |
| `7cafbc6` | Manage group from info: invite link, members, permissions | New group administration UI in `src/ui/dialogs.rs` and `src/backend/worker/` | **Integrate & Rework**: Route group administration commands through `AccountId` for multi-account safety. |
| `0e9d21a` | Create group from New chat | New dialog and protocol flow | **Integrate**: Ensure new group flows update active account archive. |
| `3f14b95` | Support opening whatsapp protocol group invite links (#591) | URL scheme handler in `src/chat_link.rs` and `src/single_instance.rs` | **Integrate**: Expand existing link parser while maintaining single-instance socket validation. |
| `bc1fc68` | Place sent messages at server's time (#574) | Modifies message ordering in `src/archive.rs` | **Integrate with Care**: Protect against timestamp tie-breaking regressions with `msg_order_id`. |
| `7a46de7` | Follow phone's Keep chats archived (#572) | App-state sync for archive behavior | **Integrate**: Add schema handling to `archive/` app-state sync. |
| `58f220b` | Leave unwritten contacts out of chat list (#573) | Contact filtering in `src/archive.rs` | **Integrate**: Clean query refinement; safe to adopt. |
| `52fd595` | Send archive changes made here to phone (#589) | App-state mutation queue | **Integrate**: Adopt queue while ensuring pending changes survive connection loss. |
| `2421739` | Translate every entry of the message menu (#552, #565) | Localization in `src/i18n.rs` and `.po` files | **Integrate**: Merge updated catalog strings across all 10 locales. |
| `fe041b4` | Keep previous run log as zapfast.previous.log | Diagnostics in `src/paths.rs` | **Integrate**: Keep log rotation without exposing user message contents. |
| `8b7ffba` / `997e408` | Direct AEC3 tuning and speaker echo cancellation | Low-level audio processing in `src/call_audio.rs` | **Rework & Integrate**: Merge upstream AEC3 pipeline while keeping ZapFaster's truthful audio recovery and ringer. |
| `e55adff` / `4c67322` / `b393deb` | Video lip-sync timing and group call tile layout | WebRTC video pipeline and `src/ui/call.rs` | **Rework & Integrate**: Merge group call grid while preserving ZapFaster's compact window mode and reachable device selectors. |
| `e559727` / `d121efc` / `3a88fc5` | Video bitrate hold and repair packets | WebRTC transport in `src/calls.rs` | **Integrate**: Critical for packet loss resilience on lossy connections. |
| `e3b5c25` | Paste text on Shift+Insert (#576) | Keyboard handler in `src/ui/conversation.rs` | **Integrate**: Standard terminal/UNIX paste shortcut. |
| `214c97c` | Voice message playback speed per chat (#526) | Audio player state per chat | **Integrate**: Store playback speed preference in chat settings. |
| `b768fd1` | Skip window attention request on Hyprland (#562) | Wayland / Hyprland compatibility | **Integrate**: Prevents compositor hangs. |
| `8b6bb6d` / `a448248` | Move to Fastframe 0.13.1 and egui 0.36.2 | Core framework upgrade | **Evaluate & Stage**: Requires verifying compatibility with existing custom plugins (`SelectionLeash`, `CopyAnnotator`). |

---

## 3. High-Priority Technical Risks & Mitigations

### 3.1. Audio & Call Subsystem Conflicts
* **Problem**: Upstream introduced direct AEC3 echo cancellation and group calls, while ZapFaster implemented compact window adaptations, incoming call ringing, and audio cleanup. A direct file replacement causes regressions on either side.
* **Mitigation**: 
  - Keep low-level DSP (AEC3, WebRTC loss recovery, capture timestamping) in `src/calls.rs` and `src/call_audio.rs`.
  - Preserve the user-facing call state machine in `src/ui/call.rs` including incoming ringing sound, compact dialogs, and minimum viewport constraints.

### 3.2. Database & Schema Safety (`src/archive.rs`)
* **Problem**: Upstream changes to message ordering (`#574`) and archive sync (`#589`) write new fields. In ZapFaster, `archive.db` is encrypted with SQLCipher using keyring keys and stores multi-account data, poll histories, and durable sends.
* **Mitigation**:
  - All schema mutations must increment `SCHEMA_VERSION` or run idempotently with `IF NOT EXISTS`.
  - Timestamp ordering adjustments must preserve `msg_order_id` secondary sorting to avoid UI jitter.
  - Never run migrations across accounts simultaneously without isolating transactions.

### 3.3. Multi-Account Boundary
* **Problem**: Newly introduced upstream actions (such as group creation, call invitations, and group info edits) assume a single global `Bot` client.
* **Mitigation**:
  - Wrap all incoming commands in `Command` variants that carry `AccountId`.
  - Ensure that background workers process actions within their scoped `state/accounts/<id>/` folder.

---

## 4. Phase 1 Implementation Plan

1. **Step 1: Security & Dependency Refresh**
   - Apply `ctutils` 0.4.5 update.
   - Verify toolchain compatibility with Rust 1.98+ / 1.99.

2. **Step 2: Group Management Integration**
   - Port group info permissions and invite link generation (`7cafbc6`).
   - Port group creation dialog (`0e9d21a`).
   - Port whatsapp protocol invite links (`3f14b95`).

3. **Step 3: Message Synchronization & Localisation**
   - Port server-time placement (`bc1fc68`) with test coverage.
   - Port uncontacted list exclusions (`58f220b`) and archive settings sync (`7a46de7`, `52fd595`).
   - Port Shift+Insert paste shortcut and message menu translations.

4. **Step 4: Calling & Echo Cancellation Synthesis**
   - Merge upstream AEC3 audio processing into `src/call_audio.rs`.
   - Re-validate incoming call ringing and compact UI in `src/ui/call.rs`.

5. **Step 5: Automated Verification & Documentation**
   - Execute test suite (`cargo test`).
   - Run headless demo tour validation.
