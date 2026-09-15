"""Python hero renderer helper for the v2 verification path."""

from __future__ import annotations

from dataclasses import dataclass
import subprocess


@dataclass(slots=True)
class ChafaRenderer:
    """Shell out to the current YAM Chafa preset for legacy fallback output."""

    def render_frame(self, gif_path: str, width: int, height: int) -> str:
        result = subprocess.run(
            [
                "chafa",
                "--format=symbols",
                "--symbols=braille",
                "--colors=full",
                "--color-space=rgb",
                "--color-extractor=median",
                "--dither=diffusion",
                "--dither-grain=1x1",
                "--fg-only",
                "--bg=#00e000",
                "--size",
                f"{width}x{height}",
                gif_path,
            ],
            check=True,
            capture_output=True,
            text=True,
        )
        return result.stdout.rstrip("\n")
