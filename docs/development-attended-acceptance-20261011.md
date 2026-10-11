# Attended acceptance preview

All cases below are **not run**. Local fixtures, software X11 captures and builds
remain recorded separately. A test account means a WhatsApp number used only for
invented conversations, with no personal chats, contacts or attachments.

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
