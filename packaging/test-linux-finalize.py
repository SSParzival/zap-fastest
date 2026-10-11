#!/usr/bin/env python3
"""Synthetic ELF fixtures for offset-preserving AppImage metadata."""
import importlib.util
from pathlib import Path
import struct
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("finalize", Path(__file__).with_name("linux-finalize.py"))
finalize = importlib.util.module_from_spec(spec)
spec.loader.exec_module(finalize)


def fixture():
    data = bytearray(2048)
    data[:6] = b"\x7fELF\x02\x01"
    data[8:11] = b"AI\x02"
    struct.pack_into("<Q", data, 40, 64)
    struct.pack_into("<HHH", data, 58, 64, 3, 1)
    names = b"\0.shstrtab\0.upd_info\0"
    struct.pack_into("<I", data, 128, 1)
    struct.pack_into("<QQ", data, 152, 256, len(names))
    data[256:256 + len(names)] = names
    struct.pack_into("<I", data, 192, 11)
    struct.pack_into("<QQ", data, 216, 512, 1024)
    data[1800:] = b"S" * 248  # Appended filesystem bytes must survive.
    return data


class FinalizeTests(unittest.TestCase):
    def test_architecture_and_offsets_survive_repeated_finalization(self):
        for architecture, name in [("amd64", "x86_64"), ("arm64", "aarch64")]:
            with tempfile.TemporaryDirectory() as root:
                path = Path(root) / "fixture.AppImage"
                before = fixture()
                path.write_bytes(before)
                finalize.finalize(path, architecture)
                first = path.read_bytes()
                finalize.finalize(path, architecture)
                self.assertEqual(path.read_bytes(), first)
                self.assertEqual(first[:512], before[:512])
                self.assertEqual(first[1536:], before[1536:])
                self.assertIn(f"*-{name}.AppImage.zsync".encode(), first)

    def test_invalid_or_conflicting_sections_fail_before_mutation(self):
        for corruption in ["bounds", "name", "conflict", "header"]:
            data = fixture()
            if corruption == "bounds":
                struct.pack_into("<Q", data, 216, 2040)
            elif corruption == "name":
                struct.pack_into("<I", data, 192, 999)
            elif corruption == "conflict":
                data[512:517] = b"other"
            else:
                data[5] = 2
            with tempfile.TemporaryDirectory() as root:
                path = Path(root) / "fixture.AppImage"
                path.write_bytes(data)
                with self.assertRaises(ValueError):
                    finalize.finalize(path, "amd64")
                self.assertEqual(path.read_bytes(), data)


if __name__ == "__main__":
    unittest.main()
