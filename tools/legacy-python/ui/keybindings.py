"""Keybinding definitions for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Keybinding:
    """Declarative key to action mapping."""

    key: str
    action: str
