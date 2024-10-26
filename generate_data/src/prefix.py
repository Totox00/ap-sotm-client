# This file is generated as part of the compilation of the client
from enum import IntEnum
from itertools import groupby
from typing import NamedTuple, Callable, Set, Optional, Iterable
from BaseClasses import CollectionState
class SotmState:
    items: Set[str]
    def __init__(self):
        self.items = set()
    def has(self, item: str, _player: int, _count: int = 1) -> bool:
        if item.startswith("Any "):
            base = item[4:]
            return base in self.items or any(variant in self.items for variant in base_to_variants[base])
        return item in self.items
    def has_any(self, items: Iterable[str], _player: int) -> bool:
        return any(item in self.items for item in items)
class SotmCategory(IntEnum):
    Scion = 0
    Hero = 1
    Villain = 2
    TeamVillain = 3
    Environment = 4
    Variant = 5
    VillainVariant = 6
    Filler = 7
    Trap = 8
    Event = 9
class FillerType(IntEnum):
    Hero = 0
    Villain = 1
    Other = 2
class FillerData(NamedTuple):
    name: str
    type: FillerType
    name_pos: Optional[str] = None
    name_neg: Optional[str] = None