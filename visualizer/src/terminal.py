from __future__ import annotations

import shutil
import sys
from contextlib import contextmanager
from dataclasses import dataclass


CSI = "\x1b["
RESET = "\x1b[0m"


@dataclass(frozen=True)
class TerminalSize:
    columns: int
    rows: int


def get_size() -> TerminalSize:
    size = shutil.get_terminal_size(fallback=(120, 36))
    return TerminalSize(columns=size.columns, rows=size.lines)


def clear_screen() -> None:
    sys.stdout.write(f"{CSI}2J{CSI}H")


def move_home() -> None:
    sys.stdout.write(f"{CSI}H")


def hide_cursor() -> None:
    sys.stdout.write(f"{CSI}?25l")


def show_cursor() -> None:
    sys.stdout.write(f"{CSI}?25h")


def enter_alt_screen() -> None:
    sys.stdout.write(f"{CSI}?1049h")


def exit_alt_screen() -> None:
    sys.stdout.write(f"{CSI}?1049l")


def flush() -> None:
    sys.stdout.flush()


@contextmanager
def terminal_session():
    enter_alt_screen()
    hide_cursor()
    clear_screen()
    flush()
    try:
        yield
    finally:
        sys.stdout.write(RESET)
        show_cursor()
        exit_alt_screen()
        flush()
