"""Panel helpers for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Panel:
    """Independent UI component."""

    panel_id: str
    focused: bool = False
    visible: bool = True
