# RTS Conversion Plan

## Objective

Convert the current Fracture Command prototype from an indirect sector-control war simulation into a playable RTS foundation.

The first deliverable is intentionally narrow:

- One playable map.
- Two playable races: Aetherborn Concord and Terran Expeditionary Directorate.
- Local single-player skirmish against a basic AI opponent.
- Workers, matter, aether, buildings, production, tech gates, supply, unit control, combat, and victory by destroying the enemy main base.

The Inheritors remain a designed faction, story force, and future implementation target. They are not part of the first playable deliverable except as lore hooks or non-interactive map dressing.

## Current State

The current runtime still reflects the older commander/squad prototype:

- `src/game/mod.rs` owns `GameState`, commanders, squads, sectors, markers, and victory timers.
- `src/types.rs` defines `FactionId`, `UnitType`, `Squad`, `Commander`, `Sector`, and related sector-control concepts.
- `src/simulation/` handles squad AI, combat, and sector capture.
- `src/input/` currently supports camera, spawning units by hotkey, doctrine cycling, squad selection, squad orders, and right-click sector markers.
- `src/rendering/` renders sectors, squads, units, commander UI, supply bars, and victory status.
- `tests/simulation.rs` tests the current sector-control prototype: sector flip, player victory, and AI victory.

The RTS design direction exists in docs and data:

- `docs/races/` defines Aetherborn, humans, and Inheritors.
- `docs/systems/resource_management.md` defines matter and aether.
- `docs/story/war_origin.md` defines the war context.
- `docs/characters/` defines campaign-facing characters.
- `assets/data/rts_races/` contains design JSON for units, buildings, and tech.

The conversion should reuse working pieces where useful, but the current sector/squad/doctrine loop is not the target game loop.

## First Deliverable Definition

The first deliverable is a playable RTS slice called **Crash Basin Skirmish**.

### Player Experience

The player launches the game and enters a single map with:

- A Terran or Aetherborn starting base.
- An enemy base of the other race.
- Starting workers.
- Nearby matter source.
- Nearby aether access point.
- One contested central aether lane.
- At least one expansion location per player.
- Fog of war can be omitted for the first playable slice, but scouting units should still exist.

The player can:

- Select units and buildings.
- Box-select units.
- Right-click move.
- Attack-move or issue attack orders.
- Train workers.
- Gather matter.
- Build race-specific aether infrastructure.
- Build production and supply structures.
- Train combat units.
- Research at least one tech upgrade per race.
- Fight the enemy.
- Win by destroying the enemy main base.
- Lose when their own main base is destroyed.

### First Playable Races

Use these two races first:

1. Aetherborn Concord
2. Terran Expeditionary Directorate

Reasoning:

- They are the core story conflict.
- Their aether economies are sharply different.
- Their visual and mechanical identities are easy to read.
- Inheritors require adaptation, remains, and corruption systems that should be built after the RTS foundation works.

## First Map: Crash Basin Skirmish

### Map Intent

Crash Basin Skirmish should teach the full RTS loop without requiring multiple maps, campaign scripting, or complex terrain.

The map should express the story:

- Humans have a crashed foothold and industrial extraction route.
- Aetherborn defend a damaged ley basin and ritual network.
- The middle contains a contested ley line scar where both economies want different things from the same place.

### Layout

Use a compact mirrored-but-themed 1v1 layout:

- West start: Aetherborn grove base.
- East start: Terran crash base.
- Center: contested ley line crossing.
- North expansion: matter-rich salvage/ruin field.
- South expansion: living grove/mineral-root field.
- Two side paths for harassment.
- One direct central attack lane.
- A few blockers or cliffs for readable choke points.

### Required Map Objects

The map data should contain:

- Main base spawn positions.
- Worker spawn positions.
- Matter source nodes.
- Ley line segments.
- Ley intersections.
- Buildable areas.
- Path blockers.
- Expansion markers.
- Camera bounds.
- Starting race ids.

### Suggested Data File

Create a new RTS map data file when implementation begins:

- `assets/data/rts_maps/crash_basin_skirmish.json`

The current `assets/data/maps.json` is built around old 3-faction sector maps and should not be used as the primary source for the RTS slice.

## Race Scope For First Deliverable

The existing race JSON files are broad design data. For the first playable slice, implement a reduced subset.

### Aetherborn Concord MVP

Core identity:

- Magical units and creatures.
- Ritual aether network.
- Sustainable territory.
- Strong defensive magic if ley connections are protected.

Required units:

- `sprite_gatherer`: worker and builder.
- `elven_warden`: basic ranged combat unit.
- `grove_sentinel`: defensive frontline.
- `wizard_adept`: first tech unit and spell/control identity.
- `unicorn_lancer`: mobile harassment/support unit if time allows.

Required buildings:

- `heartwood_nexus`: main base, trains workers.
- `moonwell`: supply and light local recovery.
- `grove_circle`: basic production.
- `ley_shrine`: new RTS building for claiming ley intersections.
- `ritual_node`: new RTS building for connecting ley flow.
- `arcane_spire`: tech building for Wizard Adepts and first upgrade.

Required tech:

- `living_bark`: defensive durability upgrade.
- `silencing_glyphs` or simpler `ley_amplification`: first caster upgrade.

Required economy behavior:

- Sprite Gatherers gather matter from matter nodes.
- Ley Shrines only function if connected to a Heartwood Nexus through Ritual Nodes.
- Connected Ley Shrines generate stored aether and ley flow capacity.
- Cutting a Ritual Node stops or reduces downstream aether income.

### Terran Expeditionary Directorate MVP

Core identity:

- Militarized expeditionary sci-fi.
- Matter and salvage efficiency.
- Aether extraction through batteries and logistics.
- Strong production if infrastructure is protected.

Required units:

- `field_engineer`: worker, builder, repair.
- `ranger_trooper`: basic ranged combat unit.
- `pulse_bike`: scout and raider.
- `aegis_walker`: armored frontline tech unit.
- `rail_artillery`: siege unit if time allows.

Required buildings:

- `command_ark`: main base, trains workers.
- `supply_pylon`: supply.
- `fabricator_bay`: basic production.
- `power_relay`: infrastructure/power gate.
- `aether_extractor_rig`: new RTS building for extracting from ley access points.
- `battery_depot`: new RTS building for receiving and storing aether cells.
- `hardlight_lab`: tech building for Aegis Walker and first upgrade.

Required tech:

- `stabilized_barrels`: basic ranged upgrade.
- `aegis_projectors`: first advanced shield/frontline upgrade.

Required economy behavior:

- Field Engineers gather matter from matter nodes.
- Aether Extractor Rigs generate Aether Cells.
- Aether Cells move automatically to Battery Depots or Command Ark through a visible route.
- If the route is blocked or depot is destroyed, aether income stalls.
- Battery objects should be visible and raidable, but not manually controlled.

## Architecture Plan

The existing modules can be kept as names, but their contents should shift to RTS responsibilities.

### Proposed Module Layout

```text
src/
├── main.rs
├── lib.rs
├── config.rs
├── data.rs
├── types.rs
├── game/
│   ├── mod.rs
│   ├── commands.rs
│   ├── selection.rs
│   └── victory.rs
├── simulation/
│   ├── mod.rs
│   ├── economy.rs
│   ├── production.rs
│   ├── construction.rs
│   ├── movement.rs
│   ├── combat.rs
│   ├── tech.rs
│   └── ai.rs
├── rendering/
│   ├── mod.rs
│   ├── world.rs
│   ├── ui.rs
│   ├── resources.rs
│   └── selection.rs
├── input/
│   └── mod.rs
├── ai/
│   ├── mod.rs
│   ├── skirmish_ai.rs
│   └── build_order.rs
└── network/
    └── mod.rs
```

Do not build networking in the first deliverable. Preserve the module for later.

### Types To Add

Add RTS-native types before rewriting behavior:

- `RaceId`: `Aetherborn`, `Terran`, `Inheritor`.
- `ResourceStockpile`: `matter`, `aether`, possibly `ley_flow_capacity`.
- `EntityId`: stable id for units, buildings, resources, projectiles, and logistics objects.
- `EntityKind`: `Unit`, `Building`, `ResourceNode`, `LeyNode`, `Projectile`, `Logistics`.
- `UnitInstance`.
- `BuildingInstance`.
- `ResourceNode`.
- `LeyNode`.
- `ProductionQueue`.
- `ConstructionJob`.
- `ResearchState`.
- `PlayerState`.
- `RtsMap`.
- `Command`.
- `SelectionState`.

Existing `Unit`, `Squad`, `Commander`, and `Sector` should be considered legacy during migration. Avoid expanding them further.

### Data Loading

The runtime should eventually load:

- Race unit definitions from `assets/data/rts_races/*/units.json`.
- Race building definitions from `assets/data/rts_races/*/buildings.json`.
- Race tech definitions from `assets/data/rts_races/*/tech.json`.
- RTS map definitions from `assets/data/rts_maps/*.json`.

For the first implementation pass, it is acceptable to define a small Rust-side `MvpCatalog` that mirrors the JSON subset. The second pass should replace that with data loading and validation.

## Migration Strategy

### Principle

Do not attempt to evolve squads/sectors into RTS entities. That will preserve the wrong model. Build RTS entities beside the old model, switch the main loop to them, then delete or quarantine old behavior.

### Step 1: Establish RTS Domain Types

Create RTS-native structs in `src/types.rs` or split them into `src/game/state.rs` if the file becomes too large.

Minimum:

- players
- entities
- resources
- units
- buildings
- resource nodes
- ley nodes
- commands
- production queues

Acceptance:

- `cargo test` passes.
- New headless tests can instantiate an RTS state without Macroquad rendering.

### Step 2: Build Headless RTS Simulation

Add systems in `src/simulation/`:

- `economy.rs`: worker gathering, matter income, aether income hooks.
- `construction.rs`: workers place and complete buildings.
- `production.rs`: buildings train units.
- `tech.rs`: buildings research upgrades.
- `movement.rs`: unit movement and basic path target following.
- `combat.rs`: direct unit combat using RTS entities.
- `victory.rs`: main-base destruction win condition.

Acceptance:

- Tests prove workers gather matter.
- Tests prove buildings train units.
- Tests prove a player loses when their main base dies.

### Step 3: Implement First Map Data

Create `assets/data/rts_maps/crash_basin_skirmish.json`.

Acceptance:

- Test loads map data.
- Test asserts it has two starts, matter sources, ley nodes, and expansion points.
- Game can create an RTS state from the map.

### Step 4: Implement Basic Unit Control

Replace old spawn/order controls with RTS controls:

- Left-click select.
- Drag box select.
- Shift-click add selection if easy.
- Right-click move or interact.
- Attack-move hotkey.
- Building selection panel.
- Production buttons.
- Build menu for workers.

Acceptance:

- Player can select workers.
- Player can send workers to gather.
- Player can select a production building and train a unit.
- Player can command army units to move and attack.

### Step 5: Implement Race Economy Differences

Implement Aetherborn and Terran aether systems.

Aetherborn:

- Ley Shrine claims ley node.
- Ritual Node connects shrine to nexus.
- Connected shrine provides aether and flow capacity.
- Broken chain stops or reduces output.

Terran:

- Aether Extractor Rig claims ley node.
- Aether Cell logistics object travels to Battery Depot or Command Ark.
- Destroyed depot or unsafe route interrupts income.
- Battery route is visible.

Acceptance:

- Test proves Aetherborn connected shrine generates aether.
- Test proves disconnected shrine does not generate aether.
- Test proves Terran extractor generates aether only when depot route is available.
- Test proves destroying or disabling depot stops Terran aether delivery.

### Step 6: Implement First Combat Loop

Implement enough combat to make army choices matter:

- Unit weapons.
- Range.
- Cooldown.
- Health.
- Armor or damage reduction.
- Target acquisition.
- Attack commands.
- Building damage.
- Main-base destruction.

Acceptance:

- Ranger Troopers can kill Elven Wardens under controlled conditions.
- Grove Sentinels beat basic infantry in defensive roles.
- Aegis Walkers require tech and beat unteched light units.
- Wizard Adepts provide a clear control/spell advantage after tech.

### Step 7: Implement Basic Skirmish AI

The first AI does not need to be clever. It needs to make the map playable.

AI requirements:

- Builds workers until a target count.
- Builds supply when needed.
- Builds first production building.
- Gathers matter.
- Builds aether infrastructure.
- Trains basic combat units.
- Sends periodic attacks.
- Rebuilds workers if economy is damaged.

Race-specific AI can be simple scripts:

- Terran AI: build supply, fabricator, extractor, depot, troopers, walkers.
- Aetherborn AI: build moonwell, grove circle, ley shrine, ritual node, wardens, sentinels.

Acceptance:

- AI can reach combat unit production without cheating.
- AI sends attacks.
- AI can lose naturally.
- AI can win if ignored.

### Step 8: Render A Playable UI

The UI should support the RTS loop, not the old commander doctrine panel.

Required UI:

- Top resource bar: matter, aether, supply, selected race.
- Selection panel: selected unit/building info.
- Command card: move, attack, stop, gather, build, train, research.
- Production progress.
- Minimap, even if simplified.
- Build placement preview.
- Game over screen.

Acceptance:

- User can play without reading debug console output.
- Resource changes are visible.
- Build/production errors are visible.
- Selection state is clear.

## Milestone Schedule

### Milestone 0: Planning And Data Audit

Deliverables:

- This conversion plan.
- Links from README.
- Confirmed first deliverable scope.

Completion evidence:

- `docs/plans/rts_conversion_plan.md` exists.
- README links to it.

### Milestone 1: RTS State Skeleton

Goal:

Headless RTS state exists beside current systems.

Deliverables:

- RTS entity model.
- Player resources.
- Basic map-state construction.
- No rendering dependency.

Tests:

- `rts_state_initializes_two_players`
- `player_starts_with_main_base_and_workers`
- `resources_start_at_expected_values`

### Milestone 2: Matter Economy And Construction

Goal:

Workers can gather matter and build structures.

Deliverables:

- Resource nodes.
- Worker gather commands.
- Worker build commands.
- Build completion timers.
- Supply from buildings.

Tests:

- `worker_gathers_matter_from_node`
- `worker_constructs_supply_building`
- `cannot_train_when_supply_blocked`

### Milestone 3: Production And Tech

Goal:

Buildings produce units and research first upgrades.

Deliverables:

- Production queues.
- Unit training.
- Research queues.
- Race-specific building requirements.

Tests:

- `fabricator_trains_ranger_after_cost_paid`
- `grove_circle_trains_warden_after_cost_paid`
- `tech_requires_correct_building`
- `researched_upgrade_modifies_unit_stats`

### Milestone 4: Aether Economy

Goal:

Aetherborn and Terran economies feel different.

Deliverables:

- Aetherborn ley network.
- Terran battery logistics.
- Visual state for ley interaction.

Tests:

- `aetherborn_ley_shrine_requires_connection`
- `aetherborn_broken_ritual_chain_stops_income`
- `terran_extractor_requires_delivery_route`
- `terran_battery_depot_receives_aether_cells`

### Milestone 5: Combat And Victory

Goal:

The game can be won through RTS combat.

Deliverables:

- Direct unit combat.
- Building damage.
- Main-base destruction victory.
- Basic unit counters.

Tests:

- `units_attack_enemy_in_range`
- `main_base_destroyed_ends_game`
- `tech_unit_beats_basic_unit_when_unanswered`

### Milestone 6: One Playable Map

Goal:

Crash Basin Skirmish is playable end-to-end.

Deliverables:

- RTS map JSON.
- Map loader.
- Spawn bases and workers.
- Matter and ley nodes.
- Camera bounds.
- Rendered resource sites.

Tests:

- `crash_basin_loads_two_start_positions`
- `crash_basin_has_matter_and_ley_nodes`
- `game_state_spawns_from_crash_basin`

### Milestone 7: Input And UI

Goal:

A human can play without debug shortcuts.

Deliverables:

- Selection.
- Box selection.
- Right-click move/interact.
- Attack command.
- Build menu.
- Production panel.
- Resource UI.
- Game over UI.

Manual verification:

- Start match.
- Select workers.
- Gather matter.
- Build supply.
- Build production.
- Build aether infrastructure.
- Train army.
- Attack enemy base.
- Win or lose.

### Milestone 8: Basic AI

Goal:

Single-player skirmish works against a simple AI.

Deliverables:

- Race-specific scripted build order.
- Economy management.
- Production management.
- Attack waves.

Tests:

- `ai_builds_workers_and_production`
- `ai_trains_combat_units`
- `ai_sends_attack_wave`

Manual verification:

- AI does not stall in the first five minutes.
- AI can damage the player if ignored.
- Player can beat AI through normal RTS play.

## Test Strategy

Keep simulation test coverage headless. Rendering and input can have thinner manual checks until the RTS core is stable.

### Required Headless Test Groups

Economy:

- Matter gathering.
- Aether generation.
- Supply caps.
- Resource spending.

Construction:

- Placement validity.
- Worker construction.
- Build completion.
- Building requirements.

Production:

- Queue progress.
- Cost payment.
- Supply blocking.
- Tech gates.

Combat:

- Range and cooldown.
- Damage and death.
- Building destruction.
- Victory state.

Map:

- Map loads.
- Required node counts.
- Starting states.

AI:

- Build order progresses.
- Attack wave exists.
- AI can win if uncontested.

## Runtime Conversion Order

Recommended file-level sequence:

1. Add RTS types behind existing public module surface.
2. Add RTS tests that do not interact with old squad tests.
3. Add `RtsGameState` beside `GameState`.
4. Make `main.rs` instantiate `RtsGameState` only after the headless tests pass.
5. Replace rendering with RTS rendering incrementally.
6. Replace input with RTS commands.
7. Remove old squad/sector UI.
8. Retire old sector-control tests once equivalent RTS tests exist.

This avoids breaking all runtime behavior before a replacement is testable.

## What To Keep From Current Prototype

Keep:

- Macroquad window and loop structure.
- Camera handling, with adjustments.
- Basic rendering organization.
- `lib.rs` testable module surface.
- Some combat math ideas, if moved to RTS entities.
- Existing JSON-loading approach as a pattern.

Replace:

- Commander doctrines for first RTS deliverable.
- Sector-control victory.
- Squad orders as primary control.
- Hotkey spawning.
- Three-faction runtime assumption.
- Cosmetic strategic markers.
- Supply-as-only-economy.

Defer:

- Networking.
- Full campaign.
- Inheritor playable faction.
- Fog of war.
- Advanced pathfinding.
- Multiplayer.
- Save/load.

## Risks

### Risk: Too Much Economy Complexity

Mitigation:

- Implement matter first as a simple worker resource.
- Implement only one aether mechanic per race.
- Avoid Inheritor remains/adaptation until two-race RTS works.

### Risk: Human Battery Logistics Becomes Annoying

Mitigation:

- Make logistics automatic.
- Show routes clearly.
- Let players defend route decisions, not micromanage haulers.

### Risk: Aetherborn Ley Network Becomes Passive Turtling

Mitigation:

- Put ley nodes outside main base.
- Reward connected outer nodes.
- Let enemies cut network links.
- Make offensive spells stronger near connected ley paths.

### Risk: Old Code Fights New Architecture

Mitigation:

- Create RTS state beside old state first.
- Move `main.rs` only after headless RTS tests pass.
- Delete old features only when replacement features exist.

## First Deliverable Completion Checklist

The first deliverable is complete only when all of these are true:

- Game launches into Crash Basin Skirmish.
- Player can play as Aetherborn or Terran.
- Opponent can be the other race.
- Both races start with main base and workers.
- Matter can be gathered.
- Aether can be collected through each race's distinct mechanic.
- Supply can block and unblock production.
- Buildings can be constructed.
- Units can be produced.
- At least one tech upgrade per race can be researched.
- Units can be selected, moved, and ordered to attack.
- Combat can destroy units and buildings.
- Destroying the enemy main base wins.
- Losing your main base loses.
- Basic AI can build economy, produce units, and attack.
- Headless tests cover economy, production, aether, combat, victory, and map load.
- A human can complete a match without debug-only hotkeys.

## Definition Of Done For This Plan

This plan is complete when it is linked from the README and can guide implementation without relying on conversation history.
