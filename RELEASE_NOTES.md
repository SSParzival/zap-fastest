# ZapFastest Release Notes

## Version 0.31.0

ZapFastest 0.31.0 is the definitive release uniting upstream ZapFast v0.20+ core protocol improvements with ZapFastest's distinct multi-account architecture, durable SQLite offline queue, and privacy-preserving power-user ergonomics. Every change has been thoroughly verified with 1,968 unit and integration tests passing.

---

### Highlights of ZapFastest 0.31.0

#### 1. Composer View-Once Photos
Photos staged in the composer can now be toggled as view-once directly via the view-once button or keyboard shortcut before sending. If a send fails or the chat is switched, the view-once choice is preserved inside the local draft and will not downgrade when re-opened.

#### 2. Chat List Clear Chat Context Action
The chat list now offers a "Clear chat" option in its right-click context menu. Messages in the conversation are dropped locally while simultaneously dispatching the clear action to WhatsApp so phone and desktop stay in sync without requiring navigating into chat details.

#### 3. Send Messages from Unlocked Folders
Conversations residing inside the password-protected locked folder can now freely compose and send messages once the folder is unlocked.

#### 4. Automatic System Appearance Following
New installations and newly added accounts automatically default to following the operating system's light or dark mode theme setting.

#### 5. Debian 12 Linux Release Compatibility
Release binaries and packages are compiled against Debian 12 (Bookworm) sysroots, restoring seamless installation across modern Linux distributions (Debian, Ubuntu, Fedora, Arch Linux).

---

### Upstream Protocol & Synchronization Fixes

- **Pinned Chats Above Parked Drafts:** Pinned chats remain permanently at the very top of the chat list across all views, with parked unsent drafts ordered directly below pinned chats and ahead of recent activity.
- **Isolated Composer Undo History:** Keystroke Undo/Redo in the composer is strictly scoped to the active chat and active account, ensuring text from previous chats or accounts can never be restored across context switches.
- **Translucent Custom Themes on Linux:** Restored transparent window backings for see-through themes on Linux Wayland/X11 compositors, eliminating visual ghosting and window flicker.
- **Recipient-Indexed Group Receipts:** Group delivery receipts are indexed by participant JID, speeding up receipt tracking in large groups.
- **Voice Message Resilience:** Playback skips stray middle initial pages without refusing valid OGG/Opus voice notes.
- **Privacy-ID (@lid) Merge Preservation:** Message metadata, stars, pins, and motion clips survive correctly when privacy-id chats and phone-number chats merge.
- **Hidden HD Photo/Video Copies:** WhatsApp's companion HD duplicate attachments are cleanly masked rather than appearing as confusing unsupported message tiles.
- **Display-Off Notification Suppression:** Notifications are deferred to the phone while displays are off, preventing battery drain and notification clutter while away.
- **Large-Window Forwarding Dialog:** Forwarding dialog now dynamically expands to give plenty of room on large displays while maintaining keyboard navigation.
- **Status Mention Notice Filtering:** Notices indicating a contact mentioned a group in their status are cleanly ignored.
- **Atomic Media Confirmation:** Downloaded media paths are committed only after SQLite archive persistence succeeds, preventing orphaned active transfers upon storage errors.
- **Strict International Phone Number Validation:** New contact creation verifies country codes and valid international phone digits, rejecting invalid alphanumeric entries.

---

### Distinct ZapFastest Architecture

- **True Multi-Account Isolation:** Independent session databases, credentials, SQLite encryption keys, and cache folders per linked WhatsApp account.
- **Durable SQLite Outgoing Queue:** All outgoing dispatches are safely staged in SQLite before network transmission, ensuring zero message loss across unexpected shutdowns or reconnects.
- **Custom Theme Catalog:** Native support for fastframe-theme and Omarchy dynamic JSON color palettes with real-time reload.
- **Native VoIP & Offline Transcription:** CPU-accelerated Whisper voice transcription running locally with zero network telemetry, alongside Sonora local echo cancellation.
- **Archive-Backed Gallery:** Instant search and inspection of media, links, and documents directly from the chat header.

---

### Installation & Updating

To build and run the latest release locally:

```bash
cargo build --release
./packaging/install-user.sh target/release/zap-fastest
```

Or install directly to Cargo bin:
```bash
cargo install --path .
```
