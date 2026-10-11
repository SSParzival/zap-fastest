# Unused media-cache cleanup

The account storage settings offer an optional age for unused cache files.
**Keep all files** is the default. Selecting an age permits deletion of old,
unreferenced files from that account's internal media-cache folder. The first
eligible worker tick checks the folder after startup, then checks once a day.
An age is measured from the file's last-modified time, not the message date or
the last time a person viewed it. Future timestamps do not expire.

This deliberately protects the retained local archive:

- Every archived attachment path is protected, including locked chats, carousel
  cards, motion clips and nested content families. Matching basenames are
  protected even when their recorded path is outside the cache.
- All durable background media job states protect their matching filenames.
  This also protects files published just before a crash, whose archive path
  has not yet been saved. The completed-file recovery audit therefore does not
  immediately download a file removed by this cleanup.
- Sticker records and tracked export destinations are protected.
- Only regular files directly inside the account's media-cache folder are
  considered. Subfolders, edited/generated media, staging files, avatars,
  sticker folders, account state and external Downloads are untouched. A
  selected Downloads folder overlapping the internal media cache blocks cleanup.
- Links and Windows reparse points are skipped. A root with a redirected
  ancestor or a traversal component is refused. Candidates are checked again
  for path, type, size and modification time immediately before removal.
- Cleanup waits while privacy is withheld or background activity is paused,
  during logout cleanup, transfers, uploads, transcription or message removals.
  The worker rechecks those gates after its filesystem scan.

Errors and exceeded safety bounds retain files. A sweep takes a complete
reference snapshot before deleting anything, and refuses archives above 20,000
messages, sticker records, tracked exports, reference paths or durable media
jobs. A directory above 4,096 direct entries is also refused in full. These
bounds keep work bounded; a future paged ownership index can support larger
archives. Logs report only counts or fixed reasons, never archive paths,
message identifiers, content or contact details.

Unused files may be leftovers from interrupted operations or old downloads.
They can contain personal media even though no current archive record points
to them. Removing them is permanent, and WhatsApp or the phone may no longer
have a recoverable copy. No real user cache or archive was inspected or
modified to implement this feature.

On 2026-10-11 the user explicitly chose **preserve all archived attachments;
clean unused files only**. This is the requested retention policy for this
client, and narrower than upstream issue #204's proposed general cache expiry.
It provides configurable startup/daily age cleanup for orphaned media-cache
files. It does not expire attachments still referenced by messages or erase
retained background archive files. Referenced-attachment expiration is outside
the chosen policy, rather than an unfinished substitute for this request.

Synthetic fixtures cover archive reference families and locked paths, durable
job states, reference-limit refusal, age boundaries, changed candidates,
account-root confinement, links on Unix, default-off and privacy gating.
Tests and builds are deferred to the whole backlog batch validation. Windows
junction/reparse substitution and native-client storage controls still require
platform acceptance. This metadata-check design does not claim to defend
against another process racing directory replacement between the final check
and removal; stronger handle-relative filesystem operations need separate
platform work.
