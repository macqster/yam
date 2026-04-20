"""Terminal session helpers for the v2 scaffold."""

from __future__ import annotations

from contextlib import contextmanager
from typing import Iterator


@contextmanager
def live_session() -> Iterator[None]:
    """Enter an alternate screen and hide the cursor for a live scene."""
    print("\x1b[?1049h\x1b[?25l", end="", flush=True)
    try:
        yield
    finally:
        print("\x1b[?25h\x1b[?1049l", end="", flush=True)
