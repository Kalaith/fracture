# Resource Management

## Design Goal

Fracture Command uses two core resources:

- `matter`: physical mass used to build bodies, structures, repairs, and conventional war material.
- `aether`: living planetary power used for magic, advanced technology, shields, rituals, mutations, and elite systems.

The key rule:

**Matter is what war is made from. Aether is why the war becomes impossible to stop.**

Matter should be readable and common. Aether should be faction-defining and visibly change the map.

## Resource Roles

| Resource | Role | Primary Use |
| --- | --- | --- |
| Matter | Body | Workers, basic units, buildings, repairs, defenses, expansion infrastructure |
| Aether | Power | Spells, shields, elite units, high tech, mutations, global abilities, advanced upgrades |

The game should not make both resources equally strange. If both resources require unusual faction-specific systems, the economy becomes hard to read. Aether carries the weirdness. Matter grounds the player in familiar RTS decisions.

## Aether: Living Flow

Aether is not a crystal pile or a renamed gas node.

Aether is flow.

Ley lines are the planet's living power grid. They connect ruins, forests, crash zones, sacred basins, and ancient structures. Every faction wants access to the same ley network, but each faction uses a different verb.

| Race | Verb | Relationship To Aether | Gameplay Feel |
| --- | --- | --- | --- |
| Aetherborn Concord | Conduct | Harmonize ley flow through rituals | Network, positioning, ritual timing |
| Terran Expeditionary Directorate | Extract | Drill, bottle, ship, and burn it | Logistics, convoys, batteries, infrastructure |
| The Inheritors | Consume | Digest ley flow and mutate from it | Corruption, scarcity, snowball, map damage |

### Shared Ley States

Ley lines should visually remember who touched them.

| Ley State | Created By | Gameplay Effect |
| --- | --- | --- |
| Wild Ley | Neutral map state | Claimable, unstable, normal output |
| Harmonized Ley | Aetherborn rituals | Better spells, wards, movement, and ley-flow capacity |
| Drained Ley | Human extraction | Lower natural output, higher industrial output, battery hazards |
| Corrupted Ley | Inheritor absorption | Regeneration, mutation bonuses, hostile pressure against other factions |
| Fractured Ley | Overuse or damage | Dangerous storms, unstable income, hard to exploit cleanly |

Permanent ley damage should be rare. Most damaged ley should be restorable, stabilizable, or corruptible by opposing faction actions.

## Aetherborn Aether: Ritual Network

The Aetherborn should not mine aether. They complete ritual circuits.

### Core Mechanic

Aetherborn build `Ley Shrines` on ley intersections and connect them back to a `Heartwood Nexus` through chains of `Ritual Nodes`.

Ley Shrines generate limited value alone. Their strength comes from uninterrupted connection.

The Aetherborn economy depends on:

- Number of ley points claimed.
- Strength of connected ritual networks.
- Whether the network is uninterrupted.
- Whether Sprite Gatherers or ritual units are maintaining the circuit.
- Whether nearby terrain remains healthy.

### Ley Flow Capacity

Aetherborn should have two aether concepts:

- Stored Aether: spent on spells, research, and production.
- Ley Flow Capacity: active current reserved by wards, elite units, rituals, and map effects.

Examples:

- Spells consume stored aether.
- Defensive wards reserve flow capacity.
- Elite magical creatures require active flow capacity.
- Global rituals require uninterrupted ley paths.
- Spirit movement can accelerate along connected ley lines.

If an enemy cuts a Ritual Node, the Aetherborn do not merely lose income. Wards may dim, teleport paths may break, elite units may lose bonuses, and spell timing may collapse.

### Player Fantasy

The battlefield is my spell circle.

### Strategic Strength

Aetherborn are terrifying when their ley network is intact. They gain strong local defense, powerful ritual windows, and efficient magical control.

### Strategic Weakness

Their network is brittle when cut.

Enemy raids should target:

- Ritual Nodes.
- Ley Shrines.
- Sprite Gatherers maintaining rituals.
- Corrupted junctions.
- Terrain between connected shrines.

## Human Aether: Battery Logistics

Humans treat aether as dangerous fuel.

They do not understand it spiritually. They industrialize it. They rip it from ley lines, condense it, containerize it, move it, and burn it inside shields, hardlight, reactors, weapons, and walkers.

### Core Mechanic

Humans build `Aether Extractor Rigs` near ley lines. These generate unstable `Aether Cells`.

Aether Cells do not teleport safely into the player's bank. They move through automatic logistics:

- Battery drones.
- Convoy trucks.
- Hover haulers.
- Cargo walkers.
- Pipeline relays in later tech.

The player does not manually click every convoy. The player chooses where to extract, where to build depots, which routes to defend, and whether batteries are reserved for economy or battlefield use.

### Battery Uses

Batteries can support both economy and combat:

- Power forward bases.
- Overcharge rail artillery.
- Trigger emergency shield bursts.
- Rapid-build deployable structures.
- Temporarily power hardlight bridges or walls.
- Activate Titan Frame systems.

### Battery Risks

Batteries create visible targets and moral ugliness:

- Batteries can explode if destroyed.
- Leaks can damage terrain.
- Convoys can attract Inheritor organisms.
- Battery yards can become raid objectives.
- Aetherborn campaign factions treat extractors as desecration.
- Overuse can create scarred ley zones that reduce future natural output.

### Player Fantasy

Secure the fuel line. Power the war machine.

### Strategic Strength

Humans get reliable industrial output once infrastructure is protected. Their economy scales cleanly and supports powerful timing pushes.

### Strategic Weakness

Their aether economy creates physical logistics targets:

- Extractor rigs.
- Battery convoys.
- Refinery depots.
- Relay towers.
- Stored battery yards.

## Inheritor Aether: Ley Assimilation

The Inheritors do not gather aether. They feed on ley lines.

They do not build a stable economy. They create wounds.

### Core Mechanic

Inheritors grow `Assimilation Sinks` onto ley lines. These produce strong aether output at first, corrupt surrounding terrain, and reduce or transform the ley source if left unchecked.

Fresh ley sites are valuable because the Inheritors are strongest when they have something new to consume.

### Assimilated Essence

Inheritor aether should be high-tempo but destructive.

Assimilation Sinks can be sustained by feeding them:

- Biomass.
- Wreckage.
- Battlefield remains.
- Captured traits.
- Corrupted ley growth.

Output starts high, then decays unless the sink is fed or the Inheritor player expands to fresh ley.

### Map Transformation

An Inheritor ley site should become visibly corrupted:

- Local regeneration for Inheritor units.
- Mutation acceleration.
- Hostile terrain pressure against other factions.
- Reduced clean output for future Aetherborn or human use.
- Higher risk of ley fracture if overfed.

### Player Fantasy

Everything becomes part of us.

### Strategic Strength

Inheritors can turn conflict into fuel. They spike hard after harvesting battlefields and fresh ley sources.

### Strategic Weakness

Absorption is greedy and reveals intent.

If Inheritors consume too much too fast, they may:

- Exhaust nearby ley sources.
- Reveal themselves through corruption spread.
- Become dependent on expansion.
- Struggle to hold clean territory.
- Trigger neutral planetary defenses.
- Create restoration targets for Aetherborn.
- Create predictable attack paths.

## Matter: Physical Mass

Matter is the shared physical resource all factions understand. It represents anything that can be broken down, reshaped, grown, printed, forged, fed, or built into war.

Matter should be easier to understand than aether:

1. Put workers near a matter source.
2. Get matter.
3. Spend matter on the body of the army and base.

Faction identity still matters, but matter should not become as mechanically complex as aether.

| Race | What Matter Means | What They Do With It |
| --- | --- | --- |
| Aetherborn Concord | Living mass, mineral roots, sacred timber, stone, beast-bone, fertile growth | Grow buildings, arm warriors, awaken guardians |
| Terran Expeditionary Directorate | Salvage, ore, alloys, wreckage, polymers, machine parts | Fabricate units, repair vehicles, build infrastructure |
| The Inheritors | Biomass, corpses, wreckage, genetic material, broken machines | Grow bodies, mutate shells, rebuild broods |

## Matter Source Types

All factions can gather most matter sources, but each race prefers different sources.

| Source | Description | Best User |
| --- | --- | --- |
| Mineral Roots | Deep planetary material and neutral deposits | Everyone |
| Living Groves | Dense life, roots, timber, and natural mass | Aetherborn |
| Salvage Fields | Crash debris, machine wrecks, broken hulls | Humans |
| Ancient Ruins | Old stone, relic metals, dormant constructs | Aetherborn and humans |
| Corpse Fields | Post-battle remains | Inheritors |
| Infested Slurry | Corrupted biomass and wreckage blend | Inheritors |

This keeps matter common while making the best expansion spots faction-dependent.

## Aetherborn Matter: Cultivation

The Aetherborn cultivate matter instead of strip-mining it.

They gather from:

- Living groves.
- Mineral roots.
- Sacred timber.
- Stone circles.
- Beast remains.
- Ancient guardian shells.

### Regenerative Matter

Matter sources inside Grove Influence recover slowly if the surrounding land remains healthy.

This makes Aetherborn territory sustainable but vulnerable to scorched earth, industrial damage, and corruption.

Good mechanics:

- Sprite Gatherers collect from groves and mineral roots.
- Grove Influence improves matter regeneration.
- Destroyed Aetherborn buildings leave living remnants that can be regrown at a discount.
- Heavy war damages the land and lowers future matter output.

## Human Matter: Salvage And Fabrication

Humans are best at turning dead machines and broken terrain into useful war material.

They gather from:

- Crash debris.
- Wrecked vehicles.
- Ore deposits.
- Scrap fields.
- Broken drones.
- Decommissioned structures.

### Salvage Efficiency

Humans get bonus matter from wreckage and destroyed machines.

Good mechanics:

- Field Engineers mine generic matter normally.
- Salvage Drones reclaim wrecks faster than normal workers.
- Human buildings can be packed down or scrapped for partial refunds.
- Destroyed mechanical units leave high-value salvage.
- Human battlefields become valuable if they survive long enough to clean them up.

Limits:

- Salvage must be collected physically.
- Salvage fields deplete.
- Salvage drones and convoys can be raided.
- Biological salvage is poor unless upgraded.
- Inheritors can deny salvage by absorbing remains first.

## Inheritor Matter: Remains Economy

The Inheritors treat matter as food.

They gather from:

- Biomass.
- Corpses.
- Wreckage.
- Mutated growth.
- Captured units.
- Battle remains.

### Remains Economy

Dead units leave `Remains`. Inheritors harvest Remains better than anyone else.

Good mechanics:

- Scavenger Larvae gather from normal matter sources.
- Dead units leave Remains.
- Inheritors can harvest Remains for matter.
- Some Remains also provide adaptation progress.
- Infested terrain slowly grows low-value biomass.
- Inheritor structures can consume nearby matter sources faster but damage them.

Balance limits:

- Remains decay over time.
- Harvesters are vulnerable.
- Large remains require a short channel.
- Fire, cleansing, or human demolition can deny remains.
- Inheritors must choose between harvesting for matter or consuming for adaptation.

## Matter Denial

Matter conflict should produce tactical denial without making the resource too strange.

| Action | Denies |
| --- | --- |
| Humans demolish wreckage | Inheritor remains and adaptation |
| Aetherborn cleanse corrupted fields | Inheritor biomass |
| Inheritors consume wrecks | Human salvage |
| Humans strip a grove | Aetherborn regeneration |
| Aetherborn regrow battlefields | Human salvage and Inheritor remains |
| Inheritors corrupt mineral roots | Clean matter access for everyone |

## Implementation Guidance

### Keep Aether Visible

Aether collection should visibly change the map.

Players should be able to tell at a glance:

- Aetherborn have woven this region into a ritual lattice.
- Humans have scarred this region into an industrial battery corridor.
- Inheritors have infected this region into a hungry wound.

### Keep Matter Readable

Matter should remain the common RTS grounding resource. It can have faction flavor and denial mechanics, but it should not require the same amount of mental overhead as aether.

### Avoid Manual Logistics Chores

Human battery logistics should be automatic and raidable, not click-intensive. The player should make strategic route and depot choices, not babysit every hauler.

### Avoid Permanent Map Ruin As Default

Inheritor corruption and human extraction should damage ley lines, but most sites should have counterplay:

- Aetherborn cleanse.
- Humans stabilize.
- Inheritors re-corrupt.
- Fractured sites recover slowly or become dangerous neutral zones.

### Resource Spending Rule

Use matter for the body:

- Workers.
- Basic units.
- Most buildings.
- Repairs.
- Defensive structures.
- Expansion infrastructure.
- Unit replacement.

Use aether for the soul:

- Advanced units.
- Magic.
- Shields.
- Artillery overcharge.
- Mutations.
- Elite upgrades.
- Global abilities.
- Special economy boosts.
