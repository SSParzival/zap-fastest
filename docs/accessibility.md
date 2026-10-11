# Keyboard and screen reader access

Chat lists, messages, starred messages and search results expose native
AccessKit list roles with each visible item's position and the loaded list's
count. More history can still arrive from the phone. With a row focused, use
Up/Down, Home/End or Page Up/Page Down (ten items). Text fields keep their
editing keys. Tab enters a toolbar once; Left/Right moves among its buttons.
F6 and Shift+F6 move between the sidebar, conversation and composer.

Alt+1 focuses the chat list, Alt+2 the newest loaded message and Alt+3 the
first unread loaded message. On macOS these use physical Control, leaving
Command+number available for switching chats. On a focused message, R replies,
E opens reactions, C copies text, F opens forwarding, Delete opens confirmation,
Space plays media, O opens an attachment, V focuses the first available poll
choice and G goes to its quoted message. Actions retain the message menu's
availability rules. Control+Left/Right moves through a message's controls;
plain arrows retain media seeking. Shift+F10 opens the focused message menu.

Control+Shift+R starts recording when the conversation permits sending.
Enter sends and Escape discards; recording buttons also work through Tab.
Emoji and mention suggestions expose listbox options and the composer's active
suggestion. Up/Down chooses an option and Enter or Tab inserts it.

Polite status announcements report new incoming messages in the open chat,
typing, connection changes and operation errors. They use generic labels and
never include message content. Hidden accounts and the locked or hidden window
remain silent. Locking clears pending announcements. The window title identifies
the open conversation only while unlocked.

Source fixtures exercise native roles, list movement, action routing and
announcement privacy. Native NVDA, VoiceOver and Orca acceptance still requires
an attended session on their respective platforms. Windows' Applications key
cannot currently reach this app: the pinned egui input API has no matching key
variant. Supporting it requires an upstream input change; Shift+F10 is available.
