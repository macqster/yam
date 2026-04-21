"""Golden frame verification for the current YAM clock baseline."""

from __future__ import annotations

import subprocess
from pathlib import Path


def load_golden_frame() -> str:
    """Load the canonical golden frame from docs."""
    docs_path = Path(__file__).resolve().parent / "docs" / "v2" / "GOLDEN_FRAME.md"
    content = docs_path.read_text(encoding="utf-8")
    lines: list[str] = []
    in_frame = False
    for line in content.splitlines():
        if line == "## Frame":
            in_frame = True
            continue
        if in_frame and line == "```text":
            continue
        if in_frame and line == "":
            continue
        if in_frame and line == "```":
            break
        if in_frame:
            lines.append(line)
    return "\n".join(lines)


def main() -> int:
    """Exit with a non-zero status when the frame drifts."""
    repo_root = Path(__file__).resolve().parent
    current = subprocess.run(
        [
            "go",
            "run",
            "./cmd/yamv2",
            "--once",
            "--width",
            "40",
            "--height",
            "20",
            "--clock",
            "12:34",
            "--day",
            "wtorek, 21 kwietnia",
        ],
        cwd=repo_root / "v2",
        check=True,
        capture_output=True,
        text=True,
    ).stdout.rstrip("\n")
    expected = load_golden_frame()
    if current != expected:
        print("golden frame mismatch")
        print("--- expected ---")
        print(expected)
        print("--- current ---")
        print(current)
        return 1
    print("golden frame ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
