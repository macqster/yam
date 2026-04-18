from __future__ import annotations

import math
import random
from dataclasses import dataclass

from terminal import RESET
from vines_ornament import GREEN, LIGHT_GREEN, OLIVE, DARK_BROWN


@dataclass
class FallingLeaf:
    x: float
    y: float
    vx: float
    vy: float
    age: int
    lifespan: int
    drift_seed: int
    color: str


GLYPHS = ["·", "'"]


def spawn_falling_leaf(state, layout, rng: random.Random):
    del layout
    if rng.random() > 0.08:
        return

    candidates = []
    for (x, y) in getattr(state, "leaf_stamps", {}):
        if y <= 4:
            candidates.append((x, y))

    if not candidates:
        return

    sx, sy = rng.choice(candidates)

    leaf = FallingLeaf(
        x=float(sx),
        y=float(sy),
        vx=rng.uniform(-0.15, 0.15),
        vy=rng.uniform(0.35, 0.7),
        age=0,
        lifespan=rng.randint(18, 42),
        drift_seed=rng.randint(0, 10000),
        color=rng.choice([OLIVE, OLIVE, GREEN, DARK_BROWN]),
    )

    if not hasattr(state, "falling_leaves"):
        state.falling_leaves = []

    state.falling_leaves.append(leaf)


def update_falling_leaves(state, layout, rng: random.Random):
    if not hasattr(state, "falling_leaves"):
        state.falling_leaves = []

    spawn_falling_leaf(state, layout, rng)

    survivors = []
    for leaf in state.falling_leaves:
        leaf.age += 1

        sway = math.sin((leaf.age + leaf.drift_seed) * 0.25) * 0.16
        flutter = math.sin((leaf.age + leaf.drift_seed) * 0.15) * 0.05
        wind_bias = 0.018
        leaf.x += leaf.vx + sway + wind_bias
        leaf.y += leaf.vy + flutter

        if leaf.age < leaf.lifespan:
            survivors.append(leaf)

    state.falling_leaves = survivors


def render_falling_leaves(state, layout) -> dict[tuple[int, int], str]:
    del layout
    rendered = {}

    for leaf in getattr(state, "falling_leaves", []):
        px = int(round(leaf.x))
        py = int(round(leaf.y))

        glyph = GLYPHS[(leaf.age // 6) % len(GLYPHS)]
        if (px + 1, py) in rendered:
            continue
        rendered[(px, py)] = f"{leaf.color}{glyph}{RESET}"

    return rendered