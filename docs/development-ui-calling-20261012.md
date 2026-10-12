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
Commits/push, fresh CI, updated Windows install and
attended runtime confirmation remain pending. The installed personal client is
still the prior candidate, with its verified rollback backup intact.

Continue the original batch acceptance afterward, including the corrected offline
merge recheck, storage/download protections, notifications/privacy and video/
camera behavior. Preserve the active account and offline restored copy. Do not
tag or publish a release.
