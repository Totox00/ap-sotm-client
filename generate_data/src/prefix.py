# This file is generated as part of the compilation of the client
from enum import IntEnum
from itertools import groupby
from typing import NamedTuple, Callable, Set, Optional, Iterable
from BaseClasses import CollectionState
from collections import Counter
class SotmState:
    prog_items:dict[int,Counter[str]]
    def __init__(self):self.prog_items={0:Counter()}
    def has(self,item:str,_player:int,_count:int=1)->bool:
        return item in self.prog_items[0]
    def has_any(self,items:Iterable[str],_player:int)->bool:return any(self.prog_items[0].get(item, False) for item in items)
    def has_all(self,items:Iterable[str],_player:int)->bool:return all(self.prog_items[0].get(item, False) for item in items)
class SotmCategory(IntEnum):Scion=0;Hero=1;Villain=2;TeamVillain=3;Environment=4;Variant=5;VillainVariant=6;Filler=7;Trap=8;Contender=9;Gladiator=10
class FillerType(IntEnum):Hero=0;Villain=1;Other=2
class FillerData(NamedTuple):name:str;type:FillerType;name_pos:Optional[str]=None;name_neg:Optional[str]=None