#!/usr/bin/env python3
"""Privacy boundary tests for the counter-only exporter."""
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location("call_diagnostics", Path(__file__).with_name("export-call-diagnostics.py"))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class CounterExport(unittest.TestCase):
    def test_known_counts_are_typed(self):
        record = module.extract("2026-10-14T17:28:00Z INFO [CALL] relay at=heartbeat generation=2 received=0 closed=false")
        self.assertEqual(record["counters"], {"generation": 2, "received": 0, "closed": False})

    def test_private_or_unrecognized_fields_reject_the_whole_record(self):
        for fields in ["received=2 phone=15550000001", "received=2 payload=123", "received=2 peer=synthetic", "received=2 received=3", "received=2 at=unknown", "received=2 secret", "received=2 at=heartbeat at=heartbeat"]:
            self.assertIsNone(module.extract("2026-10-14T17:28:00Z INFO [CALL] relay " + fields))

    def test_unknown_events_and_message_text_are_not_exported(self):
        self.assertIsNone(module.extract("2026-10-14T17:28:00Z INFO message body"))
        self.assertIsNone(module.extract("2026-10-14T17:28:00Z INFO [CALL] payload received=1"))


if __name__ == "__main__":
    unittest.main()
