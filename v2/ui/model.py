"""UI state model for the v2 scaffold."""

from __future__ import annotations

from dataclasses import dataclass, field


@dataclass(slots=True)
class Panel:
    """Independent UI component placeholder."""

    panel_id: str
    focused: bool = False
    visible: bool = True


@dataclass(slots=True)
class UIModel:
    """UI-only state separated from the engine."""

    mode: str = "live"
    focused_panel: str = "viewport"
    panels: list[Panel] = field(default_factory=lambda: [Panel(panel_id="viewport", focused=True)])
    status: str = ""
