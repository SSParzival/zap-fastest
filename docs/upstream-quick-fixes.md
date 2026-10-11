# Upstream usability fixes

Adapted from crmne/zapfast PRs #268, #470, #471, #473, #474, and #475.

- Narrow chat-list headers leave room for every button, including this fork's
  Starred messages button, hiding the title when there is insufficient space.
- Pairing QR codes also open WhatsApp's Linked devices screen from a phone's
  native camera. No QR payload is logged.
- Voice-message time and delivery ticks share the duration line when they fit.
  Download errors that need more space retain a separate footer.
- The window owns each account's online presence. Reconnects and display-name
  sync no longer implicitly announce online. Existing serialized retries and
  connection-generation guards remain in place.
- On niri, showing an existing window asks the compositor to focus it on its
  existing workspace. Requests run off the interface thread with socket
  timeouts and bounded replies. Failure retains the existing Wayland reopen
  fallback. Other platforms keep their existing window behavior.
- Downloaded conversation stickers open a native viewer with phone-synced
  favorite controls. File headers establish animation independently of protocol
  flags. A paused thumbnail cannot reset a playing viewer. Account switches,
  locked/deleted source chats, and source-message deletion dismiss the viewer.

Tests use synthetic files and headless UI. Root verification runs the project
checks; physical phone cameras and niri focus still require attended runtime
verification on those platforms.

## Optional Messages appearance (#332)

Adapted the contributor's Messages appearance into the fork's existing native
components, retaining the account boundaries, private name/photo covers,
selection/transcript behavior, pinned-message banner and media albums. A
single reverse pass resolves run tails to avoid repeated history scans.
Settings retain compatibility with releases that know only the system theme.
Custom JSON palettes cannot opt into the special bubble geometry.

Synthetic tests cover settings compatibility, bubble geometry and tail/run
boundaries, photo framing, and text/link contrast. Demo flags `messages` and
`messages-light` expose both appearances with synthetic content. Native macOS
materials, Windows Mica and Hyprland translucency require attended verification;
unsupported platforms use opaque backgrounds. This does not affect WhatsApp
protocol behavior or send any content outside the client.

## Group rename and notifications (#313, #346)

The group rename field now guarantees positive inner text width even in a
narrow dialog. This removes a source-evidenced invalid-size path; it does not
prove the historical Windows 0.18.1 crash was caused by that path. Group subject
validation, permission checks and acknowledgement handling remain in place.

Issue #346 originally requests the macOS Dock unread badge. The fork now has
a native badge implementation; the later report of banners failing while
sounds play remains an attended macOS verification item. Platform compilation
and synthetic checks do not establish notification permission or delivery.
