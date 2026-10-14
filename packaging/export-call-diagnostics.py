#!/usr/bin/env python3
"""Export only known call counters; never copy raw log lines or arbitrary fields."""

import argparse
import datetime as dt
import json
import re
from pathlib import Path


SCHEMAS = {
    "media stats": "generation rtp_received unexpected_pt srtp_failed sframe_failed unclassified pipe_dropped decoded delivered foreign_decoded concealed without_decoder without_encoder mlow_sid mlow_dropped sink_dropped trimmed codec_switches video_sink_dropped peer_keyframes forwarding_rejected",
    "audio path": "generation mic_opens speaker_opens speaker_stalls speaker_restarts live cleanup_capture_blocks cleanup_render_blocks cleanup_failed mic_frames_sent",
    "Opus": "generation received encoded decoded submitted rejected dropped failed source_closed sink_closed",
    "video": "generation captured queue_dropped encoded_bytes encode_us capture_clock peer_received peer_decoded peer_decode_failed peer_waiting peer_decode_us peer_images_taken",
    "relay": "generation connection received handed rtp rtcp stun_like other dropped submitted send_failed closed",
}
SCHEMAS = {kind: set(fields.split()) for kind, fields in SCHEMAS.items()}
MARKERS = {"heartbeat", "video-before", "video-after", "ended", "hangup", "mute", "accepted", "active"}
LINE = re.compile(r"(?P<time>\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z).*?\[CALL\] (?P<kind>media stats|audio path|Opus|video|relay) (?P<fields>(?:[a-z_]+=(?:[a-z-]+|\d+)\s*)+)$")


def extract(line):
    match = LINE.search(line)
    if not match:
        return None
    kind = match["kind"]
    fields = {}
    marker = None
    for item in match["fields"].split():
        key, value = item.split("=", 1)
        if key == "at" and marker is None and value in MARKERS:
            marker = value
        elif key in SCHEMAS[kind] and key not in fields and (value in {"true", "false"} or value.isascii() and value.isdecimal()):
            fields[key] = {"true": True, "false": False}.get(value, value)
            if isinstance(fields[key], str):
                fields[key] = int(value)
        else:
            return None
    if not fields:
        return None
    return {"time": match["time"], "kind": kind, "marker": marker, "counters": fields}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("log", type=Path)
    parser.add_argument("--since", required=True, help="UTC timestamp, e.g. 2026-10-14T17:28:00Z")
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    since = dt.datetime.fromisoformat(args.since.replace("Z", "+00:00"))
    if since.tzinfo is None:
        parser.error("--since must include a timezone")
    records = []
    with args.log.open(encoding="utf-8", errors="replace") as stream:
        for line in stream:
            record = extract(line)
            if record and dt.datetime.fromisoformat(record["time"].replace("Z", "+00:00")) >= since:
                records.append(record)
    args.output.write_text(json.dumps(records, indent=2) + "\n", encoding="utf-8")
    print(f"Exported {len(records)} counter records.")


if __name__ == "__main__":
    main()
