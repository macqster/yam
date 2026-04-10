from __future__ import annotations

from datetime import datetime

from terminal import RESET


PANEL_ACCENT = "\x1b[38;5;180m"
PANEL_TEXT = "\x1b[38;5;252m"
PANEL_MUTED = "\x1b[38;5;109m"
PANEL_RULE = "\x1b[38;5;59m"


def _fit(text: str, width: int) -> str:
    return text[:width].ljust(width)


def build_panel_lines(config: dict) -> list[str]:
    panel_cfg = config["panel"]
    now = datetime.now().astimezone()

    title = panel_cfg.get("title", "yam")
    date_text = now.strftime("%A")
    full_date_text = now.strftime("%d %B %Y")
    time_text = now.strftime("%H:%M:%S")
    tz_text = now.tzname() or "local"

    width = config["layout"]["info_width"]
    rule = PANEL_RULE + ("─" * width) + RESET
    lines = [
        PANEL_ACCENT + _fit(title.upper(), width) + RESET,
        rule,
        PANEL_TEXT + _fit(time_text, width) + RESET,
        PANEL_MUTED + _fit(date_text, width) + RESET,
        PANEL_MUTED + _fit(full_date_text, width) + RESET,
        "",
        PANEL_RULE + _fit(tz_text, width) + RESET,
    ]
    return lines
