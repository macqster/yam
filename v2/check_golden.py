"""Golden frame verification for the v2 scaffold."""

from __future__ import annotations

from pathlib import Path

from v2.app import build_demo_ecosystem, build_demo_model
from v2.runtime.system import render_frame_with_clock


def load_golden_frame() -> str:
    """Load the canonical golden frame from docs."""
    docs_path = Path(__file__).resolve().parents[1] / "docs" / "v2" / "GOLDEN_FRAME.md"
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
    current = render_frame_with_clock(build_demo_model(), build_demo_ecosystem(), clock_text="12:34")
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
