damage_types=["Cold ","Energy ","Fire ","Infernal ","Lightning ","Melee ","Projectile ","Psychic ","Radiant ","Sonic ","Toxic "]
difficulties=["Normal","Advanced","Challenge","Ultimate"]
base_to_variants:dict[str,list[str]]={key:[d.name for d in group] for key,group in groupby(sorted((d for d in data if d.category==SotmCategory.Variant),key=lambda d:d.base),key=lambda d:d.base)}
base_to_variants.update({hero.name:[] for hero in data if hero.category==SotmCategory.Hero and base_to_variants.get(hero.name,None) is None})
