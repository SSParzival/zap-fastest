# Organizing downloaded attachments

Settings, Files lets you choose a download folder and organize future files:

- **One folder** preserves the existing layout.
- **Account and chat** separates each linked account and conversation.
- **Account, chat and month** adds the month of the message.
- **Readable download names** adds the message date and time in UTC, chat name,
  file type, original document name where available, and a short identity suffix.

These options apply to downloads saved in a chosen folder. The default private
media folder keeps its internal names. Changing a setting does not move or rename
any existing files. Save as continues to use the filename and location you choose.

Organized downloads are verified in the account's private media folder before a
copy is saved in the chosen folder. The archive keeps referencing its private
file, so moving the exported copy does not break message playback. The copy
reserves its destination without overwriting an existing file; duplicate names
gain a numbered suffix. If saving
the organized copy fails, the download reports the failure and keeps the verified
private file. Chat names are sanitized and bounded; hashed identities separate
chats with the same name without exposing their WhatsApp addresses in paths.

Background archive recovery always retains attachments with the account archive.
When you enable archival downloads, explicitly choose an export folder, and
enable organization or readable names, future completed attachments also queue
an organized copy. Motion clips receive their own video filename. Enabling these
preferences does not export old files or infer the system Downloads folder.
Locked, deleted, view-once and disappearing content cannot export automatically.
Copies resume after restarting; failures back off while the managed file remains.
Readable exports reveal chat names and message dates in filenames, so choose a
private destination if that information is sensitive.
