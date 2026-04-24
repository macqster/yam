"""Runtime state model for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class RuntimeModel:
    """Application-wide state container for the v2 runtime."""

    width: int
    height: int
    tick: int = 0
    seed: int = 0
