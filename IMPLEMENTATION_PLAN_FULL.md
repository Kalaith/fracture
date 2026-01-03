# Fracture Command - Complete Implementation Plan
## Based on Full Design Vision (No Networking)

This plan builds on the current Phase 2 prototype and extends it to the full single-player/local multiplayer vision from design.md. We're targeting a single-commander vertical slice with AI opponents.

---

## Current State (Phase 2 Complete)

✓ Basic game loop and rendering
✓ Unit spawning with supply costs
✓ Squad AI with 4 orders (Advance, Hold, Skirmish, Fortify)
✓ Commander doctrine system (5 doctrines)
✓ Basic combat resolution
✓ Sector control mechanics
✓ Victory conditions

---

## Phase 3: AI Commanders & Multi-Faction Support

**Goal**: Transition from 1v1 to 3-faction gameplay with AI commanders

### 3.1 Commander Type System
- Define 5 commander archetypes (Vanguard, Engineer, Control, Disruptor, Wildcard)
- Each type has unique doctrine pool and passive effects
- Commander switching/rotation system (later phase)
- Current implementation: Player = Vanguard, AI = mix of types

**Files to modify:**
- `types.rs` - Add `CommanderType` enum and `CommanderArchetype` trait
- `game/mod.rs` - Support multiple commanders per faction
- New: `game/commander_types.rs` - Commander archetype implementations

**Data Required**: `commander_types.json` (BULK - see below)

### 3.2 Three-Faction Game State
- Extend game state to support 3 factions instead of 2
- Faction alliance system (temporary truces, betrayals)
- Faction-wide supply pools
- Per-faction victory tracking

**Files to modify:**
- `types.rs` - Change `PlayerId` to `FactionId`, add `CommanderId`
- `game/mod.rs` - Support 3 factions with multiple commanders each
- `config.rs` - Faction configurations

### 3.3 AI Commander System
- AI decision-making framework
- Personality-driven behavior (aggressive, defensive, opportunistic)
- Doctrine selection AI
- Squad deployment AI
- Front selection and prioritization

**New files:**
- `ai/mod.rs` - AI coordinator
- `ai/commander_ai.rs` - AI decision making
- `ai/personality.rs` - AI personality types
- `ai/strategic.rs` - Strategic analysis (front priorities, threat assessment)

**Data Required**: `ai_personalities.json` (BULK - see below)

---

## Phase 4: Expanded Unit Types & Combat

**Goal**: Rich unit variety with meaningful tactical differences

### 4.1 Expanded Unit Roster
- 12-15 unit types across categories:
  - Infantry (light, heavy, elite)
  - Armor (tanks, mechs, mobile artillery)
  - Air/Drone (scouts, gunships, support)
  - Support (engineers, medics, supply)
  - Specialist (anti-armor, anti-air, siege)

**Files to modify:**
- `types.rs` - Expand `UnitType` enum
- `simulation/combat.rs` - Unit matchup bonuses (anti-armor vs armor, etc.)
- `rendering/world.rs` - Visual differentiation

**Data Required**: `unit_types.json` (BULK - see below)

### 4.2 Advanced Combat Mechanics
- Unit matchup system (rock-paper-scissors)
- Cover and terrain effects
- Morale system (squads can break/rout)
- Formation bonuses
- Range bands (optimal/falloff ranges)

**Files to modify:**
- `simulation/combat.rs` - Matchup calculations, morale
- New: `simulation/terrain.rs` - Terrain effects on combat
- `types.rs` - Add `Morale` state to squads

### 4.3 Commander Abilities (Active, Not Passive)
- Each commander type gets 2-3 active abilities
- Long cooldowns (60-120s)
- Examples:
  - Vanguard: "Shock Assault" - temporary damage boost
  - Engineer: "Emergency Repairs" - instant squad heal
  - Control: "Suppression Field" - slow enemy in area
- Abilities are targeted, not micro-intensive

**New files:**
- `game/abilities.rs` - Ability system and execution
- `input/mod.rs` - Ability hotkeys and targeting

**Data Required**: `abilities.json` (included in `commander_types.json`)

---

## Phase 5: Logistics & Territory Control

**Goal**: Strategic depth through supply management and map control

### 5.1 Supply Hub System
- 1 main hub per faction (destroyable = loss condition)
- Supply radius (units outside radius lose effectiveness)
- Hub upgrades (increase radius, throughput, storage)
- Forward supply points (capturable)

**New files:**
- `game/logistics.rs` - Supply hub logic and supply line calculations
- `types.rs` - `SupplyHub` and `SupplyNode` structs

### 5.2 Reinforcement Routes
- Squads spawn at hub and move to rally points
- Route can be intercepted (increase spawn time)
- Multiple routes = redundancy
- Route visualization

**Files to modify:**
- `game/logistics.rs` - Route pathfinding and interception
- `rendering/world.rs` - Route rendering

### 5.3 Repair & Attrition System
- Damaged squads auto-repair near hub (limited throughput)
- Units out of supply take attrition damage
- Repair prioritization (player can set)
- Field repair vs hub repair (slower but no retreat needed)

**Files to modify:**
- `game/logistics.rs` - Repair queue and throughput
- `simulation/mod.rs` - Attrition damage tick

---

## Phase 6: Strategic Map & Sectors

**Goal**: Meaningful territorial control with strategic choices

### 6.1 Enhanced Sector System
- 8-12 sectors per map (up from 4)
- Sector types:
  - Objective (victory points)
  - Resource (bonus supply generation)
  - Strategic (vision, faster reinforcements)
  - Chokepoint (defensive bonus)
- Sector connections (you can only attack connected sectors)
- Sector fortification (build defenses over time)

**Files to modify:**
- `types.rs` - Add `SectorType` and `SectorConnection`
- `game/mod.rs` - Sector connectivity and control logic
- `simulation/mod.rs` - Fortification building

**Data Required**: `maps.json` (BULK - see below)

### 6.2 Front System
- Fronts are groups of connected sectors where combat occurs
- Orders (Advance/Hold/etc) are issued per front, not globally
- Front strength visualization
- Front commander assignment (which AI/player controls which front)

**New files:**
- `game/fronts.rs` - Front detection and management

---

## Phase 7: Advanced Doctrines & Tech

**Goal**: Deep strategic customization

### 7.1 Expanded Doctrine Pool
- 20-30 doctrines across categories:
  - Offensive, Defensive, Economic, Special
- Commander-type-specific doctrines
- Doctrine synergies and conflicts
- Doctrine unlock progression (light tech tree)

**Files to modify:**
- `types.rs` - Expand `Doctrine` enum
- `config.rs` - Doctrine compatibility matrix

**Data Required**: `doctrines.json` (BULK - see below)

### 7.2 Doctrine Evolution
- Doctrines can be upgraded mid-match (costs resources)
- Doctrine-specific squad buffs
- Visual indicators for active doctrines

---

## Phase 8: UI/UX Polish

**Goal**: Clear information presentation and intuitive controls

### 8.1 Advanced UI Panels
- Multi-commander panel (switch between your commanders)
- Front overview (status of each front)
- Supply flow visualization
- Threat assessment display
- Match timeline/replay

**New files:**
- `rendering/ui/panels.rs` - Specialized UI panels
- `rendering/ui/hud.rs` - HUD overlays

### 8.2 Visual Clarity
- Unit health bars
- Supply line rendering
- Doctrine effect particles
- Combat feedback (damage numbers, hit effects)
- Minimap

**Files to modify:**
- `rendering/world.rs` - Enhanced visuals
- New: `rendering/effects.rs` - Particle and VFX system

### 8.3 Controls Enhancement
- Click-drag to select multiple squads
- Front selection shortcuts
- Doctrine hotkeys
- Commander switching (Tab/Q/E)

---

## Phase 9: Match Flow & Game Modes

**Goal**: Replayable, varied experiences

### 9.1 Match Stages
- Early game: expansion and positioning
- Mid game: front establishment and pressure
- Late game: decisive pushes and last stands
- Dynamic escalation (reinforcements get stronger over time)

**New files:**
- `game/match_state.rs` - Match stage tracking and event triggers

### 9.2 Victory Conditions (Multiple Paths)
- Domination: Control X sectors for Y time
- Elimination: Destroy all enemy supply hubs
- Economic: Reach X total supply generated
- Time: Most sectors controlled at time limit

**Files to modify:**
- `game/mod.rs` - Multiple victory condition checking

### 9.3 Difficulty Levels
- AI personality intensity scaling
- AI reaction speed
- AI strategic foresight
- Starting resources

---

## Phase 10: Content & Balance

**Goal**: Tuned, content-rich experience

### 10.1 Map Variety
- 4-6 maps with different layouts
- Asymmetric maps (different faction starting positions)
- Environmental hazards (radiation zones, unstable terrain)

**Data Required**: `maps.json` (BULK - see below)

### 10.2 Balance Tuning
- Unit cost/power curves
- Doctrine effectiveness
- Commander type balance
- Map balance

### 10.3 Match Summaries & Analytics
- Post-match breakdown
- Commander effectiveness scores
- Damage dealt/taken graphs
- Critical moments timeline

**New files:**
- `game/analytics.rs` - Match statistics tracking
- `rendering/ui/summary.rs` - Post-match summary screen

---

## Phase 11: Polish & Juice

**Goal**: Satisfying audiovisual feedback

### 11.1 Sound Design
- Combat sounds (gunfire, explosions)
- UI feedback sounds
- Ambient battlefield audio
- Commander voice lines (optional)

### 11.2 Visual Effects
- Muzzle flashes
- Explosions and impacts
- Doctrine activation effects
- Unit spawn effects
- Sector capture animation

### 11.3 Camera & Controls Polish
- Camera shake on large explosions
- Smooth zoom transitions
- Edge scrolling option
- Minimap navigation

---

## Bulk Data Requirements

The following data files will exceed 100 lines of JSON and need careful structure:

### 1. `assets/data/unit_types.json` (BULK)

**Purpose**: Define all unit types, stats, and behaviors
**Estimated Size**: 300-500 lines

**Structure**:
```json
{
  "units": [
    {
      "id": "infantry_light",
      "name": "Light Infantry",
      "category": "infantry",
      "supply_cost": 5,
      "max_health": 100,
      "armor": 0,
      "move_speed": 50,
      "attack": {
        "damage": 10,
        "range": 100,
        "cooldown": 1.0,
        "type": "ballistic"
      },
      "counters": ["infantry"],
      "countered_by": ["armor", "artillery"],
      "ai_behavior": {
        "aggression": 0.7,
        "retreat_health": 0.3,
        "formation": "loose"
      },
      "visual": {
        "size": 8,
        "color": "blue",
        "icon": "infantry"
      }
    }
  ]
}
```

**Rules**:
- Each unit must have unique `id`
- `category` must be one of: infantry, armor, air, support, specialist
- `attack.type` affects matchup calculations
- `counters` and `countered_by` define rock-paper-scissors
- All numeric values must be positive
- `supply_cost` should scale with power (5-30 range)

---

### 2. `assets/data/commander_types.json` (BULK)

**Purpose**: Define commander archetypes, doctrines, and abilities
**Estimated Size**: 400-600 lines

**Structure**:
```json
{
  "commander_types": [
    {
      "id": "vanguard",
      "name": "Vanguard Commander",
      "description": "Frontline pressure and aggressive tactics",
      "passive_effects": {
        "unit_damage_bonus": 0.1,
        "advance_speed_bonus": 0.15
      },
      "available_doctrines": [
        "aggressive_posture",
        "shock_windows",
        "rapid_deployment",
        "blitz_tactics",
        "overwhelming_force"
      ],
      "abilities": [
        {
          "id": "shock_assault",
          "name": "Shock Assault",
          "cooldown": 90.0,
          "duration": 10.0,
          "effect": {
            "type": "damage_boost",
            "value": 0.5,
            "radius": 200.0
          },
          "description": "Units in area deal +50% damage for 10s"
        }
      ],
      "starting_supply": 100,
      "supply_cap": 100
    }
  ]
}
```

**Rules**:
- Each commander type must have 5-8 unique available doctrines
- Abilities have cooldowns of 60s minimum
- Passive effects should be small (0.05-0.2 range)
- No commander should be strictly better than another
- Each type should have distinct strategic identity

---

### 3. `assets/data/doctrines.json` (BULK)

**Purpose**: Define all doctrines and their effects
**Estimated Size**: 400-700 lines

**Structure**:
```json
{
  "doctrines": [
    {
      "id": "aggressive_posture",
      "name": "Aggressive Posture",
      "category": "offensive",
      "description": "Units advance faster but take more damage",
      "compatible_commanders": ["vanguard", "disruptor"],
      "effects": {
        "move_speed_multiplier": 1.5,
        "damage_taken_multiplier": 1.25,
        "morale_drain_rate": 1.3
      },
      "conflicts_with": ["defensive_stance", "entrenched_assault"],
      "synergizes_with": ["shock_windows", "rapid_deployment"],
      "unlock_requirement": {
        "type": "none"
      }
    }
  ]
}
```

**Rules**:
- Each doctrine must have unique `id`
- `category` must be: offensive, defensive, economic, special
- Effects use multipliers (1.0 = baseline, >1.0 = increase, <1.0 = decrease)
- `conflicts_with` prevents simultaneous activation
- `synergizes_with` is informational only (no mechanical effect)
- All doctrines should have meaningful tradeoffs
- No doctrine should be universally optimal

---

### 4. `assets/data/ai_personalities.json` (BULK)

**Purpose**: Define AI commander behavior patterns
**Estimated Size**: 300-500 lines

**Structure**:
```json
{
  "personalities": [
    {
      "id": "aggressive",
      "name": "Aggressive",
      "description": "Constantly pressures and takes risks",
      "behavior_weights": {
        "expand_territory": 0.8,
        "defend_position": 0.2,
        "build_economy": 0.3,
        "harass_enemy": 0.9,
        "consolidate_forces": 0.1
      },
      "doctrine_preferences": {
        "offensive": 0.9,
        "defensive": 0.1,
        "economic": 0.3,
        "special": 0.5
      },
      "decision_timing": {
        "reaction_delay": 0.5,
        "planning_lookahead": 5.0,
        "adaptation_speed": 0.8
      },
      "risk_tolerance": 0.8,
      "preferred_unit_types": ["infantry_elite", "armor_heavy", "drone_gunship"],
      "spawn_frequency": 0.8
    }
  ]
}
```

**Rules**:
- All weights are 0.0 to 1.0
- Higher weights = more likely to perform that action
- `reaction_delay` in seconds (0.1-2.0 range)
- `planning_lookahead` in seconds (1.0-10.0 range)
- `risk_tolerance` affects whether AI commits to risky plays
- Must define at least 4 distinct personalities: aggressive, defensive, balanced, opportunistic

---

### 5. `assets/data/maps.json` (BULK)

**Purpose**: Define map layouts, sectors, and objectives
**Estimated Size**: 500-800 lines (multiple maps)

**Structure**:
```json
{
  "maps": [
    {
      "id": "contested_valley",
      "name": "Contested Valley",
      "description": "Three-way conflict over a central valley",
      "dimensions": {
        "width": 2000,
        "height": 1500
      },
      "factions": [
        {
          "faction_id": 1,
          "spawn_position": {"x": 200, "y": 750},
          "starting_sectors": ["sector_1", "sector_2"]
        },
        {
          "faction_id": 2,
          "spawn_position": {"x": 1800, "y": 750},
          "starting_sectors": ["sector_9", "sector_10"]
        },
        {
          "faction_id": 3,
          "spawn_position": {"x": 1000, "y": 200},
          "starting_sectors": ["sector_5", "sector_6"]
        }
      ],
      "sectors": [
        {
          "id": "sector_1",
          "position": {"x": 300, "y": 600},
          "radius": 150,
          "type": "starting",
          "connections": ["sector_2", "sector_4"],
          "strategic_value": 1,
          "terrain_type": "open",
          "fortification_slots": 2
        },
        {
          "id": "sector_central",
          "position": {"x": 1000, "y": 750},
          "radius": 200,
          "type": "victory_point",
          "connections": ["sector_3", "sector_4", "sector_7", "sector_8"],
          "strategic_value": 5,
          "terrain_type": "fortified",
          "fortification_slots": 4
        }
      ],
      "objectives": [
        {
          "type": "control_sector",
          "sector_id": "sector_central",
          "duration": 60.0,
          "victory_points": 100
        }
      ],
      "environmental_effects": [
        {
          "type": "radiation_zone",
          "position": {"x": 500, "y": 200},
          "radius": 100,
          "damage_per_second": 2.0
        }
      ]
    }
  ]
}
```

**Rules**:
- Each map must have exactly 3 faction spawn positions
- Sectors must form connected graph (no isolated sectors)
- `type` must be: starting, normal, resource, chokepoint, victory_point
- Each faction should have 2-3 starting sectors
- Central contested area should exist
- `strategic_value` determines AI prioritization (1-10)
- `connections` must be mutual (if A connects to B, B connects to A)
- At least 1 victory_point sector per map

---

## Data Loading System

**New file**: `src/config.rs` (expand existing)

Add JSON loading infrastructure:
```rust
pub struct GameData {
    pub units: HashMap<String, UnitDefinition>,
    pub commanders: HashMap<String, CommanderType>,
    pub doctrines: HashMap<String, Doctrine>,
    pub ai_personalities: HashMap<String, AIPersonality>,
    pub maps: HashMap<String, MapDefinition>,
}

impl GameData {
    pub fn load() -> Result<Self, String> {
        // Load all JSON files from assets/data/
        // Validate relationships (e.g., commanders reference valid doctrines)
        // Build lookup tables
    }
}
```

**Validation Requirements**:
- All cross-references must exist (e.g., commander's doctrines exist in doctrines.json)
- No circular dependencies
- Numeric ranges are valid (no negative health, etc.)
- Balanced totals (unit costs should span reasonable range)

---

## Implementation Priority

**High Priority (Core Gameplay)**:
1. Phase 3: AI Commanders & Multi-Faction
2. Phase 4: Expanded Units & Combat
3. Phase 5: Logistics & Territory
4. Phase 6: Strategic Map & Sectors

**Medium Priority (Depth & Polish)**:
5. Phase 7: Advanced Doctrines
6. Phase 8: UI/UX Polish
7. Phase 9: Match Flow & Game Modes

**Low Priority (Content & Juice)**:
8. Phase 10: Content & Balance
9. Phase 11: Polish & Juice

---

## Technical Constraints

- Keep files under 800 lines (split if exceeded)
- All bulk data loaded at startup (no runtime file I/O)
- JSON validation on load with clear error messages
- Deterministic simulation (no floating-point drift)
- Target 60 FPS with 3 factions × 100 supply = 300 units on screen

---

## Success Criteria

By Phase 6 completion, a player should experience:
- Commanding a Vanguard in a 3-faction battle
- Meaningful doctrine choices
- Strategic supply management
- AI opponents with distinct personalities
- Clear cause-effect between decisions and outcomes
- Satisfying autonomous squad behavior

If these land, the vertical slice is validated ✓
