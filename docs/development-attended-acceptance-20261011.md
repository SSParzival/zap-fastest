# Attended acceptance preview

## Current authorization, 2026-10-12

The user now explicitly authorizes installation and attended testing on the
existing personal account. This supersedes the synthetic-account-only restrictions
below. Preserve the linked session and archive, prepare a recoverable backup and
executable rollback first, and coordinate each live test with the user. Use agreed
chats/recipients and invented content. Do not inspect archive contents, log private
data, autonomously send or call contacts, unlink, reset or relink. Cases requiring
a second account/device or fresh pairing remain pending until their prerequisite
exists. Safe-copy/offline backup, restore and merge tests must preserve the active
profile and must not connect a duplicate restored session. Destructive tests and
release publication remain unauthorized. Historical case descriptions follow;
none establishes attended acceptance by itself.

## Observed Windows acceptance, 2026-10-12

Source: `ac8c93f3d0c2b244de55f14ce8e15379f0574a4d`, Windows x86_64.
Installed executable SHA256:
`38491fe6cb33374c9660575f65910793b8dc8bdd4826a4900374ad6871e5ba49`.
Cold backup and executable rollback were prepared and verified before installation;
see [the current checkpoint](development-resume-20261012.md).

| Case | Observation | Result |
| --- | --- | --- |
| Existing-session startup | User opened the existing shortcut and confirmed connection without QR/relink and existing history present. | Passed for startup/history presence; recovery completeness is not established. |
| Invented document/image/video in Message yourself | User explicitly requested the three fixture attachments. Agent used Windows UI Automation to verify the agreed header, empty draft/caption and exactly three staged files, then invoked Send once. User confirmed all three arrived and opened correctly, including audible video playback. | Passed for this fixture send/open/playback. |
| Explicit saved document | User saved the invented document through Zap Faster and confirmed the saved copy opens correctly. | Passed for explicit document saving; automatic organization/exports remain separate. |
| Close-to-tray and reopen | User closed the window and reopened it from the tray, confirming the same connected account and history. | Passed for this close/reopen cycle. |
| Encrypted account backup | Agent entered a generated passphrase into the verified password fields using keyboard input, without clipboard/log disclosure. The secret is stored with current-user DPAPI under the protected rollback folder. Settings reported a saved backup. The closed file was copied into the permanent recovery folder and both SHA256 hashes matched: `aa72e596fa50f84fa7e2194b9a42b587667d45d8ccb7beefdc753c520cbeb9d0`, 2,312,919,910 bytes. | Export passed. |
| Reconnect after encrypted export | Source-aware UI Automation read-back observed the Settings progress label that is drawn only while connected, with no disconnected/connecting banner. | Passed for this post-backup reconnection. |
| Separate offline restore | App reported restore success. New account `2` has session/archive files, `archive_only=true` and `cli_access=false`; original account `1` remains present and connected. Restored session was never resumed. | Decrypt/install/offline gating passed; complete media/history equivalence is not established. |
| Offline merge preview/cancel | On offline account `2`, re-importing the same backup unexpectedly proposed three additions and no edits/deletions/conflicts/pending imports. Cancel was invoked; encrypted database and WAL hashes stayed identical. Original account `1` was selected again afterward. A synthetic privacy-ID fixture reproduces a duplicate-addition defect. | Cancellation passed; merge confirmation held pending correction and runtime recheck. |
| Settings keyboard navigation | User confirmed Tab/Shift+Tab reach controls with visible focus, without traps or unexpected actions. | Passed for the observed Settings navigation. |
| Phone history response | Connected Settings read-back observed `Phone history: response received`. | Response observed; full recovery and the exact explicit repair anchor are unproven. |
| Settings layout | User supplied a screenshot showing overlap and clipping. Correction `d73ded1` has synthetic bounds checks at three widths and two text scales, with all required Windows gates passing. | Installed correction and attended visual recheck pending. |
| Attended voice call | User independently called their mother and confirmed voice calling worked with satisfactory audio quality. Agent did not place the call or inspect its contents. | Basic Windows voice call/audio quality passed by user report; dialing tone and call-history/menu defects were reported. Video/camera and incoming peer notifications remain unverified. |

Organized downloads, retention/storage protection, reconnect/phone recovery,
notifications/privacy, offline backup/restore/merge and keyboard acceptance
still require their own observations. No private archive contents or screenshots
were captured by the agent. The successful voice call was user initiated.

Backup automation limitations: password ValuePattern actions did not enable saving;
foreground-checked keyboard input worked. The native save dialog rejected the
existing `C:` recovery-folder path with "Path does not exist"; saving to a new,
owner/SYSTEM-protected `H:` staging directory succeeded. The completed encrypted
file was copied and hash-verified in the permanent `C:` folder, retaining staging.
The open dialog likewise rejected the permanent `C:` copy; the hash-identical
protected `H:` copy restored successfully. This picker path mismatch remains unexplained and is not attributed to a
product defect without further evidence. Automation stopped when the client lost
foreground; it did not send keyboard input into another application.

## Historical 2026-10-11 preview

All cases below are **not run**. Local fixtures, software X11 captures and builds
remain recorded separately. A test account means a WhatsApp number used only for
invented conversations, with no personal chats, contacts or attachments.

The user confirmed on 2026-10-11 that no test numbers are available. Phone
acceptance is blocked on that prerequisite; personal accounts are not a fallback.

## Smallest first session

Use two test WhatsApp accounts on two phones and one separate application test
profile. The operator scans the QR and keeps credentials, QR images, backup
passwords and session files private. Before launching a connecting build, agree
on the exact executable hash, isolated state/cache/runtime roots, test accounts,
test peer, permitted actions and cleanup. Do not use the installed personal
client's profile or replace its executable.

The source uses normal account paths and the compatible single-instance channel;
an arbitrary launch-directory change does not isolate it. Inspect current path
resolution first. A separate OS user/profile is suitable. Do not launch against
the personal runtime and accidentally focus or command the running client.

The proposed first session permits only pairing the designated test account and
the named test operations below. It requires explicit approval before connecting,
sending or calling. Unlinking/removing a device also needs explicit approval.
Offline backup/restore does not authorize resuming a restored connection.

| Case | Operator actions | Required observations and stop condition |
| --- | --- | --- |
| Recent/Full pairing and phone availability | Prepare synthetic dated chats and ordinary attachments. Pair separate profiles once with Recent and once with Full. Keep the phone available, then briefly disconnect and reopen the client. | Correct persisted choice before pairing; existing sessions unchanged; oldest recovered date/counts correspond to the synthetic phone history; fair all-chat recovery resumes; a silent or unavailable phone remains incomplete rather than falsely complete. Record actual phone availability limits. |
| Presence | Observe the test account from the second phone while foregrounded, minimized, in tray and closed; reconnect once. | Phone-visible online state matches the chosen policy, including after reconnect; no receipt is caused by a hidden account's remembered chat. |
| Deletes, receipts and favorites | Explicitly approve a small set of uniquely named synthetic messages, deletions and favorite changes on each device. Compare both directions, including a reconnect/history replay. | Exactly one outgoing message per approved send; delivered/read/played states distinguish unknown timestamps; deleted items stay deleted after replay; the phone favorite order is retained and local changes converge. Stop on Unconfirmed, inspect status rather than resend. |
| Media/export recovery | Use ordinary synthetic image/video/document, carousel and motion content. Interrupt a transfer, reopen, retry an expired item if the phone can reupload, and cancel another. | Account-local jobs resume within budget; correct available/pending/failed states; user downloads follow chosen folders/names without moving older files; locked/deleted/disappearing/view-once content does not enter ordinary automatic exports. |
| 1:1 voice and video | Approve each outgoing call to the named test peer. Test both directions, reject, cancel, miss, upgrade and end. Deny then grant microphone/camera permission; remove a USB device during an approved call. | Both devices receive usable audio/video; signaling and media outcomes are real; cancel/lock/end stops capture; no hidden-account dialog or incorrect notification target; denied/missing devices report an error; device removal leaves the client responsive. Record platform, devices and route. |
| Backup/restore | Back up only the test profile, restore into a new offline account and compare synthetic settings/counts/media. Try wrong password, truncated backup and a destination conflict. Close the original before separately approving resume. | Complete encrypted package and consistent archive/session; refusal preserves existing accounts; restored account stays offline until confirmed; correct independent key/account paths; conflicts fail closed; no duplicate linked session. |

## New backlog acceptance cases

Every case in this section is **not run**. The phone-dependent cases remain
blocked because no test numbers are available. The offline cases use generated
fixtures and disposable profiles only; they do not inspect personal archives.
These instructions do not authorize a connection, send, call, device removal or
personal-profile launch.

| Case | Synthetic setup and operator actions | Required observations and stop condition |
| --- | --- | --- |
| Delivered Status and session-only viewer | On designated test devices, prepare invented text, image and video Status updates. Deliver them while the isolated client runs, open Status, explicitly download a media item, switch accounts, lock, revoke an update and restart. Also supply synthetic expired and replayed revoked entries through offline fixtures. | Only normally delivered, unexpired updates appear in the originating account. Viewing sends no read receipt. Status never becomes an ordinary chat, archived message, backup item or automatic user-folder export. Locked senders remain protected; revoked/expired items disappear and their downloads stop; replay cannot restore revoked items. Restart does not restore the previous session's viewer content. Missing older Status history is not presented as recovered. Phone delivery and receipt behavior remain blocked until test devices exist. |
| Windows native popup and foreground keyboard popup | In a disposable offline demo, inject invented incoming notifications for two accounts with the main window focused, unfocused, minimized and absent in tray. In each state click the popup and compare its account/chat/message target. With focus in the main window, exercise popup keyboard navigation, activation and dismissal, then repeat while locked. | The unfocused/hidden case uses the native Windows popup without taking focus merely to display it. The focused case exposes the in-window popup to keyboard and screen-reader navigation. Activation reaches the original account and message; dismissal sends nothing. Locked content and names stay hidden, and an activation waits for unlock. Record actual Windows observations separately from software captures and mocked routing tests. |
| Same-account historical backup merge | Generate two encrypted archives with the same canonical own identity and distinct current/imported messages. Include an imported queued send, pending read/archive/favorite/deletion records, a newer edited snapshot over an unedited message, two differing edited snapshots, current and imported deletion barriers, an empty-account legacy tombstone and a tombstone belonging to a previous identity. Prepare the merge preview, cancel once, then prepare again and confirm. Separately test missing/different identity, wrong password, changed current archive after preview and injected transaction failure. | Preview counts additions, updates, protected items, confirmed deletions, conflicts and imported pending records before confirmation. Current session credentials and active queues remain intact. Imported uncertain sends become Unconfirmed; imported pending work is retained encrypted without activation. Current and imported confirmed deletions and chat cutoffs prevent resurrection; the verified same-account legacy tombstone applies, while a different prior identity does not acquire authority. Revoked messages stay revoked. An imported edited snapshot updates an unedited version; ambiguous edited alternatives keep the current version and retain the imported alternative encrypted. Cancel changes nothing, stale previews refuse, and failed transactions publish neither partial history nor attachment copies. No test sends a message or resumes a restored session. |
| Undo before transport | In an offline recording-backend fixture, archive a synthetic chat and request Delete for me on one message and a multi-message selection. Check commands before the five-second deadline, click Undo, and advance a virtual deadline. Repeat without Undo, switching the visible chat/account before expiry. | No archive/delete command is sent during the grace period. Undo restores the original archive flag and cancels the scheduled command without changing read state or reversing an accepted deletion. After expiry, each remaining command goes once to its originating account/chat/message. Message removal still waits for backend acceptance. Toasts describe scheduled work and expose a keyboard-accessible Undo control; locked screens reveal no private names. Use virtual time for fixtures, not a five-second sleep. Actual phone acceptance remains a separate blocked case. |
| Future-only automatic exports | In an offline synthetic profile, enable automatic user-folder copies at a known time. Queue files dated before, at and after that cutoff, including cached files, then restart and repeat the audit. Turn automatic copies off and re-enable them; explicitly opt into older files in a separate run. Keep private background archival on Full and test manual explicit downloads separately. Include locked/deleted/disappearing/view-once entries and two files with colliding original names. | The persisted cutoff applies across restart; default automatic copies include only files at or after enable time. Missing cutoffs migrate deterministically before export permission is granted; re-enabling establishes a new cutoff. The older-files option is explicit, and Full private archival remains independent. Manual explicit downloads keep their existing behavior. Selected folder/organization and optional original filenames apply to new copies with safe collision handling. Privacy/deletion barriers still exclude ordinary automatic exports, and existing files are not silently renamed or moved. |
| Selected camera and discovery lifecycle | Use a generated device-list fixture for stable identifiers, reordered entries, removal and delayed discovery replies. During a separately authorized test call, select one of two distinguishable cameras, then exercise outgoing/incoming video, voice-to-video upgrade, refresh, device removal, cancellation, account shutdown and restart. | The chosen camera, identified by its stable platform identity where available, is used through offer, answer and upgrade rather than whichever device is listed first. A stale discovery result cannot replace a newer list, and an audio heartbeat cannot erase camera choices. Missing selections report an actionable error. Cancellation/shutdown ends discovery helpers and capture; no delayed completion crosses an account boundary. Record actual platform/device identity limitations and observed camera output. Hardware/call acceptance stays blocked without designated test devices. |

## Platform sessions

| Environment | Cases | Evidence that closes the case |
| --- | --- | --- |
| Windows desktop | Native notifications/click routing, Focus/DND, NVDA, drag/drop and clipboard of a newly made synthetic file/recording, Alt+numpad, selected Bluetooth/HDMI output, Arabic/Hebrew font sizes, high-refresh/VRR/HDR and reported flicker configuration. | Operator-confirmed behavior with exact build/platform/device settings. Clipboard diagnosis records only format names/IDs and medium type, never personal clipboard bytes. Screen reader announces controls and focus meaningfully without exposing locked content. |
| Signed macOS application bundle | Notification permission, clicked chat/account/message, Focus/DND and sounds, VoiceOver, Dock/tray behavior, OpenVanilla unmarked text, press-and-hold accents, microphone/camera permission and removal. | Actual signed bundle identity and OS permission read-back; native behavior observed on macOS. An unbundled executable or Linux macOS-layout demo cannot close these cases. |
| Native Wayland/niri and KDE/GNOME | Existing-window focus from another workspace, covered/resident window responsiveness, tray count/VIP privacy, native notification persistence, dropped files, middle-click autoscroll, frame ownership and Arabic/Hebrew ink bounds. | Actual compositor/backend/version, native hardware driver and window behavior. Xvfb/X11 images remain separate proof. |
| Older Intel/Linux hardware | Startup, settled-media and memory/latency during synthetic archive recovery. | Repeated runs on the reported class of hardware with profile and cache conditions, rather than extrapolation from the ARM64 software-X11 measurements. |

## Power-loss acceptance

Existing tests cover interrupted publication boundaries and ownership conflicts.
Actual filesystem/keyring durability requires a disposable test machine or VM
with a synthetic profile and an appropriate OS keyring. Preview and approve the
exact interruption method before use; do not power off this host or its unrelated
services. At each journal/key/cache/state/roster publication boundary, interrupt
the test system, boot and observe reconciliation. Registered restored accounts
must survive; unregistered transaction-owned files/keys must roll back; changed
credentials or ownership must remain untouched and block safely. Repeat on each
claimed platform. Process termination alone does not establish physical power-loss
durability.

## Result record

For each case save: case ID, date, executable SHA256, source commit, OS/backend,
synthetic profile identifier, exact approved actions, observed outcome, counts
and failure class, and pass/fail/blocked. Screenshots contain only invented
content. Keep passwords, keys, tokens, phone numbers and QR payloads out of reports.
No case is attended-verified until its observation is recorded.
