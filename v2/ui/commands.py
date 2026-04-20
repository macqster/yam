"""Command system for the v2 scaffold."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Command:
    """Text-driven user command."""

    name: str
    args: list[str] | None = None
