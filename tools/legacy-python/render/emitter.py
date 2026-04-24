"""Emitter for the current YAM runtime."""

from __future__ import annotations

from .framebuffer import Framebuffer


class Emitter:
    """Converts framebuffer state into terminal output."""

    def emit(self, framebuffer: Framebuffer) -> str:
        return "\n".join(framebuffer.rows())
