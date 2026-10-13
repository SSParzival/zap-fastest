# Settings and calling follow-up, 2026-10-12

The user reported that the installed `03a5b287` candidate still had overlapping
audio controls in Settings. They independently completed a voice call and
confirmed satisfactory audio quality. They also reported missing conversation
call entries, a Calls window that did not dismiss outside, and no outgoing tone.
No agent-initiated call or private archive inspection was performed.

The follow-up keeps the existing native theme and font system. Settings uses
label/description/control groups for complex rows, compact side-by-side switches,
and shared vertical microphone/output/camera controls. Account actions wrap below
the identity. Search and Back stay above the scrolling sections. Calls shows
recent history with timestamps and durations first; devices are collapsible.
Outside clicks and Escape dismiss Calls, with nested pickers handling Escape first.
The active-call window remains separate and cannot be hung up by an outside click.

Finished local calls now add an informational conversation entry, retaining known
duration and video kind. Synced metadata for that same locally recorded call does
not add another entry or overwrite the measured duration. Call entries cannot be
replied to, reacted to or forwarded as ordinary WhatsApp messages. This does not
promise complete historical or cross-device call history.

Outgoing ringback is generated locally through the selected output and its existing
device recovery path. It uses a quiet repeating cadence, follows device changes,
and stops at answer/terminal transitions. It is not mixed into outgoing call
audio. Incoming calls do not start this outgoing tone.

Focused checks passed: 10 layout tests, 12 call-worker tests and 20 audio tests,
with one existing ignored audio test. Synthetic viewport bounds cover narrow,
normal and wide Settings at normal and enlarged text. Tests also cover outside/
Escape dismissal without hangup, nested picker interactions, local conversation
records, synced replay, selected-output tone playback, rebind and shutdown.

Inputs are frozen under
`H:/Scratch/Codex/zap-batch-20261012/ui-calling-validation/frozen-inputs.json`
(345 files). Windows synthetic confirmation reviewed 11 captures covering audio
at normal and narrow widths, Files progress, account actions, notifications,
light-theme appearance, Calls in both themes, voice/video controls and a
conversation call entry. Controls and labels fit without the reported overlap;
End call remains visible before the video preview. Captures are offline demo
content in the task's scratch directory, never personal chats. Capture processes
did not take foreground focus.

The Linux demo link was canceled after sustained disk-bound linking, with no
source failure. Its container was restored to stopped state at the existing
5 GiB / 2 CPU limits. No resource increase was made. Windows native demo builds
passed; cross-platform source validation will use fresh CI.
All six required Windows gates passed: 1,803 library tests, 13 existing ignored,
10 default / 12 all-feature executable tests and two updater tests. The first
full run caught outdated Calls labels, a call mapping fixture and the tour's
now-scrolled Language control. Those fixtures were corrected without weakening
coverage; the complete checks passed afterward. The receipt is
`verification/development-ui-calling-20261012.json`.
Committed and pushed on linear main as
`3b3b0a53f16006668f50b50d47d61bd195d3c503`. Upstream main remains
`e00e377ee6c617206245455a04146a5dd5c24265`. Fresh source CI `38323347352`
passed all nine jobs; Flatpak `38323371017` passed.
The default Windows release build passed in 7m 49s and was installed after
graceful shutdown and a verified cold rollback backup at
`C:/Users/Victor/AppData/Local/ZapFasterRollback/20261012-204818`:
5,835 profile files, 5,908,123,690 bytes, previous executable preserved,
same-user keyring unchanged. Installed SHA256:
`da335552fde13027517d928e5dc6598d78dc485e2ae4ea8a23b5fb52dee40eed`.
Executable hash and version 0.30.0 were read back. No tag or publication occurred.
The user was asked to reopen the existing shortcut and confirm reconnect,
Settings fit, Calls dismissal, dialing tone and completed-call entries through
a user-initiated call with a willing person. Attended confirmation remains pending.

Attended follow-up: the user reports that a call initiated just after opening
eventually started after a long delay. They have not reported measured elapsed
time or whether the client was already connected. The startup call buttons could
enqueue a request before the worker finished startup, and preparation feedback
was only set after the worker accepted it. A focused correction now rejects
pre-connection requests with an explanation, sets preparation immediately for
connected requests, prevents duplicate clicks and clears preparation on worker
rejection. Timing markers record only phase and elapsed milliseconds, never
recipients, identifiers, message content or device names. Actual queue/device/
setup delay remains unconfirmed until measured with the corrected build.

Continue the original batch acceptance afterward, including the corrected offline
merge recheck, storage/download protections, notifications/privacy and video/
camera behavior. Preserve the active account and offline restored copy. Do not
tag or publish a release.
