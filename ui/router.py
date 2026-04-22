"""UI router for the current YAM runtime."""

from __future__ import annotations

from runtime.messages import KeyMsg
from ui.input import InputRouter
from ui.model import UIModel


class UIRouter:
    """Owns UI-only routing and state transitions."""

    def __init__(self) -> None:
        self.input_router = InputRouter()

    def handle(self, ui: UIModel, msg: KeyMsg) -> UIModel:
        return self.input_router.route(ui, msg)
