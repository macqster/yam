"""Morphology primitives for the current YAM runtime."""

from __future__ import annotations

from dataclasses import dataclass

from engine.ecosystem import Organism


@dataclass(slots=True)
class Node:
    x: int
    y: int


@dataclass(slots=True)
class Internode:
    start: Node
    end: Node


@dataclass(slots=True)
class Axis:
    nodes: list[Node]


@dataclass(slots=True)
class Organ:
    kind: str
    anchor: Node


@dataclass(slots=True)
class Morphology:
    """Structured morphology derived from engine organisms."""

    nodes: list[Node]
    axes: list[Axis]
    organs: list[Organ]


def build_morphology(organisms: list[Organism]) -> Morphology:
    """Convert organisms into a minimal structural representation."""
    nodes = [Node(x=organism.x, y=organism.y) for organism in organisms]
    axes = [Axis(nodes=[node]) for node in nodes]
    organs = [Organ(kind=organism.name, anchor=node) for organism, node in zip(organisms, nodes)]
    return Morphology(nodes=nodes, axes=axes, organs=organs)
