# Interface and notification preferences

Appearance has independent **Text size** and **Zoom** controls. Text size changes
fonts from 85 to 150 percent while Zoom changes the whole interface. Filters and
the selected local label are remembered separately for each account. A deleted
label clears its saved choice.

The label manager can enable focus mode for selected labels. Chats outside those
labels are hidden from the list and their notifications are suppressed. Its
mention option allows urgent group mentions and replies to you through. Focus
does not unlock chats. **Leave focus mode** restores the list immediately; each
account keeps its own choice.

Labels can have notification sounds. When several labels match, the first label
in the manager with a configured sound wins. A chat's own override has priority,
including silence. Mentions keep their mention sound unless the chat overrides
it. Muted chats stay muted and notification identity privacy still applies.

System settings can show unread chat numbers on the tray icon, with `99+` for
overflow. Chats marked VIP can give the unread indicator a contrasting color.
These options combine unread state across accounts while excluding locked,
archived and muted VIP chats. The app lock and screen privacy clear the tray
indicators. Desktop tray hosts still control how the icon appears.

Privacy settings can enable notification cards inside the window. They follow
the same account, mute, focus and preview settings as desktop notifications,
expire after eight seconds and show at most three cards. Open and dismiss work
with the keyboard. Enabling this does not turn off desktop notifications.

F6 and Shift+F6 move between major interface sections. Shift+F10 opens the
focused message's action menu. Primary Tab navigation remains available.
Touchscreen drags scroll the transcript, while mouse drags retain text/message
selection. On Linux, a middle click in the empty strip beside message bubbles
starts automatic scrolling; Escape or another input stops it.

Selected confirmed messages can be starred or unstarred together. Pending local
messages, deleted messages and stale selections cannot enqueue star changes.
Reactions retain their full list behind an overflow control. Fresh live reactions
can advance chat activity; history replay and reaction removal do not.
