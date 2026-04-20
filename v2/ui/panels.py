"""Panel helpers for the v2 scaffold."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Panel:
    """Independent UI component."""

    panel_id: str
    focused: bool = False
    visible: bool = True
