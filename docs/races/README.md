# RTS Race Design

This section defines the three RTS factions for the Fracture Command reboot concept. These documents describe fantasy, economy, tech identity, and intended gameplay before the data is wired into the runtime.

The story frame for the war lives in [War Origin](../story/war_origin.md).

The economy and resource model lives in [Resource Management](../systems/resource_management.md).

Named cast and campaign-facing character docs live in [Character Profiles](../characters/README.md).

The matching JSON scaffolds live under `assets/data/rts_races/`:

- `aetherborn/units.json`, `aetherborn/buildings.json`, `aetherborn/tech.json`
- `expeditionary_humans/units.json`, `expeditionary_humans/buildings.json`, `expeditionary_humans/tech.json`
- `inheritors/units.json`, `inheritors/buildings.json`, `inheritors/tech.json`

## Shared RTS Assumptions

- Players start with a main base and gatherers.
- Economy comes from two readable resources: `matter` and `aether`.
- Supply comes from faction-specific support buildings.
- Tech comes from explicit structures and researched upgrades.
- Every faction should have scouting, harassment, anti-rush defense, tech timings, siege pressure, and a late-game plan.

## Inspiration Boundaries

These factions use familiar genre touchstones, but must stay original in names, silhouettes, lore, symbols, and specific mechanics.

- Aetherborn Concord: tabletop high-fantasy warband energy. Magical people, wizards, enchanted beasts, unicorns, spirits, living forests, summoned guardians, and mythic battlefield magic.
- Terran Expeditionary Directorate: militarized expeditionary sci-fi. Drop troops, powered armor, armored walkers, drones, fleet doctrine, propaganda, survivalist conquest, and a hard-edged command culture.
- The Inheritors: assimilation collective horror. Former individuals absorbed into a shared will, cyber-organic adaptation, copied enemy traits, identity loss, and the unsettling question of whether preservation without consent is survival or erasure.

## Factions

- [Aetherborn Concord](aetherborn.md): native pure-magic civilization using elves, wizards, unicorns, enchanted beasts, living groves, and ley power.
- [Terran Expeditionary Directorate](expeditionary_humans.md): crash-landed advanced humans using expeditionary infantry, powered armor, drones, hardlight, and rail weapons.
- [The Inheritors](inheritors.md): adaptive assimilation collective that copies, mutates, and inherits attributes from defeated or absorbed enemies.

## Data Shape

The new files are intentionally separate from the current prototype JSON. They are design data, not live runtime data yet.

Common concepts:

- `race_id`: stable race identifier.
- `cost`: `{ "matter": number, "aether": number }`.
- `requires`: building or tech ids required before this item can be produced.
- `unlocks`: ids made available by a building or tech.
- `effects`: structured upgrade effects that can later be mapped to simulation rules.
- `strategic_intent`: short balancing note explaining why the item exists.
