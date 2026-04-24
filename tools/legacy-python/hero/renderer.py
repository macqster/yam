"""Python hero renderer helper for the v2 verification path."""

from __future__ import annotations

from dataclasses import dataclass
import subprocess


@dataclass(slots=True)
class ChafaRenderer:
    """Shell out to chafa so Python mirrors the Go hero pipeline."""

    def render_frame(self, gif_path: str, width: int, height: int) -> str:
        result = subprocess.run(
            [
                "chafa",
                "--format=symbols",
                "--symbols=braille",
                "--colors=none",
                "--animate=off",
                "--bg=#101010",
                "--fg-only",
                "--size",
                f"{width}x{height}",
                gif_path,
            ],
            check=True,
            capture_output=True,
            text=True,
        )
        return result.stdout.rstrip("\n")
