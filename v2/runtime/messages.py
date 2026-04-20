"""Runtime message definitions for the v2 scaffold."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class TickMsg:
    """Time progression message."""
    dt: float = 1.0


@dataclass(slots=True)
class ResizeMsg:
    """Terminal resize message."""

    width: int
    height: int


@dataclass(slots=True)
class KeyMsg:
    """Input message placeholder."""

    key: str
