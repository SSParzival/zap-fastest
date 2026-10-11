# Local command-line access

Enable Allow local command-line access in Settings, Privacy for each account
that should accept automation. The default is off. The app must already be
running. Account ids are the numeric account-folder ids, and chat ids must name
an existing conversation in that account. Restoring a backup disables this
computer-specific authority until explicitly enabled again.

The CLI uses a separate private fastframe-instance slot named
zap-faster-automation-v1 under runtime/automation-v1. Unix filesystem permissions
and Windows's random token authenticate the local user. The compatibility
legacy port never accepts these operations. This grants local programs running
as that user account access to the selected account; it is not remote access.
No network service, account system or telemetry is introduced.

Examples below use synthetic ids; substitute your chosen account/conversation.

```powershell
zap-faster automation read --account 2 --chat fixture@s.whatsapp.net --limit 10
zap-faster automation watch --account 2 --chat fixture@s.whatsapp.net --seconds 60
zap-faster automation send --account 2 --chat fixture@s.whatsapp.net --request-id 0123456789abcdef0123456789abcdef --text-file C:\path\message.txt
zap-faster automation status --account 2 --chat fixture@s.whatsapp.net --request-id 0123456789abcdef0123456789abcdef
```

Read returns ascending rows from a bounded recent archive page, plus next_before.
Pass its timestamp and id with --before-time and --before-id to read older rows.
Watch polls archived arrival order once per second for 1 through 3600 seconds,
including late history. Resume with --after-row and --after-id from the returned
cursor. It reports locally archived arrivals, not a server stream, and never
requests older history or marks a conversation read. A deleted cursor refuses
continuation instead of silently skipping data; restart the watch deliberately.
Each request allows 1 through 25 rows and 12 KiB of JSON; reduce the page size
if the response is too large. Attachment files, raw protobufs and encryption
keys are never exported. Phone-only and disappearing messages are withheld.
Locked chats are always refused, even when their folder is open in the app.
App lock refuses requests and scrubs pending replies, including hidden windows.
Each authorized request has a one-use permit. Account opt-out, account removal,
local or remotely received chat lock, and app lock permanently revoke pending
requests and replies. Unlocking or re-enabling access never revives an old request.
The transport rechecks authorization before returning a buffered response. Send
acceptance and revocation share a fence: a send accepted before revocation may
continue, while a request revoked before acceptance never enqueues.

Send accepts one UTF-8 text file of at most 4000 bytes and a stable 32-digit
hexadecimal request id. Keep that id before dispatch. It reserves the id in the
encrypted archive before enqueueing the normal supported WhatsApp text send.
Repeating that same id with the same account, chat and text returns the existing
operation instead of sending again; different content under that id is refused.
A crash after reservation may leave Unconfirmed without a visible message row.
Status queries preserve this uncertainty. A timeout never means that a send did
not happen. Query the same id, check WhatsApp, and never create a new id blindly.
Normal sends preserve the existing safe pre-transmission rate-limit cooldown;
transport failures and uncertain acknowledgements never automatically retry.
Read/status cannot edit or retry an uncertain send. Offline or read-only
conversations refuse a new send. Sending requires the explicit opt-in, not just
ordinary chat-link handling. The CLI does not send typing or read receipts.

Synthetic fixtures cover bounded requests, redacted debug representations,
lock/cancellation response scrubbing, account preference defaults/roundtrip,
request-id reuse and changed-content refusal, archive expiry/chat-lock filtering,
command parsing, per-account dispatch, opt-out/local/remote lock revocation,
and late buffered-reply scrubbing. An offline fixture builds the official SDK
with a synthetic session store and intercepts transport starts through the
existing outgoing test hook: the first request reserves/archives/enqueues one
SDK-generated id, repeats enqueue nothing, and simulated uncertain completion
stays Unconfirmed with no transport retry. This proves local acceptance and
idempotency, not live WhatsApp server acceptance. Real user archive reads and
live sends were not used to
verify implementation. Private transport, attended CLI behavior and full gates
remain verification requirements.
