"""Input routing for the current YAM runtime."""

from __future__ import annotations

from dataclasses import replace

from runtime.messages import KeyMsg
from ui.model import UIModel


class InputRouter:
    """Routes messages to UI actions without touching engine state."""

    def route(self, ui: UIModel, msg: KeyMsg) -> UIModel:
        if msg.key == "tab":
            return replace(ui, focused_panel="viewport", status="focus viewport")
        if msg.key == "d":
            next_mode = "debug" if ui.mode != "debug" else "live"
            return replace(ui, mode=next_mode, status=f"mode {next_mode}")
        if msg.key == "h":
            return replace(ui, status="help: q quit, tab focus, d debug")
        return ui
