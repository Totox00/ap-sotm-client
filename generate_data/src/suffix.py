damage_types = ["Cold ","Energy ","Fire ","Infernal ","Lightning ","Melee ","Projectile ","Psychic ","Radiant ","Sonic ","Toxic "]
difficulties = ["Normal", "Advanced", "Challenge", "Ultimate"]
base_to_variants: dict[str, list[str]] = {
    key: [d.name for d in group] for key, group in
    groupby(sorted((d for d in data if d.category == SotmCategory.Variant), key=lambda d: d.base), key=lambda d: d.base)
}
def team_villain_count(state: CollectionState | SotmState, player: int) -> bool:
    return [state.has(d.name, player) for d in data if d.category == SotmCategory.TeamVillain].count(True) >= 3
def general_access_rule(state: CollectionState, player: int):
    return [state.has(f"Any {d.name}", player) for d in data if d.category == SotmCategory.Hero].count(True) >= 3 and True in (state.has(d.name, player) for d in data if d.category == SotmCategory.Environment) and (True in (state.has(d.name, player) for d in data if d.category == SotmCategory.Villain or d.category == SotmCategory.VillainVariant) or team_villain_count(state, player))
