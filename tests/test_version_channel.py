import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))

import version_channel


class VersionChannelTests(unittest.TestCase):
    def test_first_dev_bumps_patch(self):
        self.assertEqual(version_channel.next_dev("0.1.0", 7), "0.1.1-dev.7")

    def test_dev_keeps_candidate_before_first_stable_tag(self):
        self.assertEqual(version_channel.next_dev("0.1.1-dev.7", 8), "0.1.1-dev.8")

    def test_dev_uses_latest_stable_as_next_patch_base(self):
        self.assertEqual(
            version_channel.next_dev("0.1.1-dev.8", 9, "v0.1.1"),
            "0.1.2-dev.9",
        )

    def test_stable_strips_prerelease(self):
        self.assertEqual(version_channel.stable_version("2.4.6-dev.12"), "2.4.6")

    def test_workspace_version_round_trip(self):
        with tempfile.TemporaryDirectory() as temp:
            cargo = Path(temp) / "Cargo.toml"
            cargo.write_text(
                '[workspace]\n\n[workspace.package]\nversion = "0.1.0"\nedition = "2021"\n',
                encoding="utf-8",
            )
            self.assertEqual(version_channel.read_workspace_version(cargo), "0.1.0")
            version_channel.write_workspace_version(cargo, "0.1.1-dev.3")
            self.assertEqual(version_channel.read_workspace_version(cargo), "0.1.1-dev.3")


if __name__ == "__main__":
    unittest.main()
