# Fracture Command - Data Structure Summary

This document explains the bulk data files and their validation rules.

---

## Overview

All game balance and configuration data is stored in JSON files under `assets/data/`. These files are loaded once at startup and validated for consistency. This data-driven approach allows for:

- **Easy balancing** without recompiling code
- **Modding support** (future feature)
- **Clear separation** between code and content
- **Rapid iteration** on game balance

---

## Bulk Data Files

### 1. `unit_types.json` (10 units, ~200 lines)

**What it defines**: All unit types with stats, behaviors, and visual properties

**Key Fields**:
- `id`: Unique identifier (used in code and other JSON files)
- `category`: infantry, armor, air, support, specialist
- `supply_cost`: How much supply this unit consumes (5-20 range)
- `attack`: Damage, range, cooldown, and damage type
- `counters`/`countered_by`: Rock-paper-scissors matchups
- `ai_behavior`: How AI uses this unit type

**Validation Rules**:
- All IDs must be unique
- Supply costs should scale with power (5-30 range)
- Categories must be valid enum values
- Counters must reference existing unit categories
- All numeric values must be positive

**Example Expansion**: Add 5-10 more unit types for full roster (mechs, siege units, specialized support)

---

### 2. `commander_types.json` (5 commanders, ~200 lines)

**What it defines**: Commander archetypes with unique doctrines and abilities

**Key Fields**:
- `id`: vanguard, engineer, control, disruptor, wildcard
- `passive_effects`: Always-on bonuses (small values: 0.05-0.2)
- `available_doctrines`: List of doctrine IDs this commander can use
- `abilities`: Active abilities with cooldowns and effects
- `supply_cap`: Starting supply cap (usually 100)

**Validation Rules**:
- Each commander must have 5-8 available doctrines
- Doctrine IDs must exist in `doctrines.json`
- Passive effects should be small multipliers
- Abilities must have cooldowns ≥60 seconds
- Each commander should have distinct strategic identity

**Balance Notes**:
- No commander should be strictly better than another
- Commanders should excel in different scenarios
- Passive effects + doctrines = distinct playstyle

---

### 3. `doctrines.json` (28 doctrines, ~700 lines)

**What it defines**: All doctrines and their strategic effects

**Key Fields**:
- `id`: Unique identifier
- `category`: offensive, defensive, economic, special
- `compatible_commanders`: Which commanders can use this doctrine
- `effects`: Stat multipliers and special modifiers
- `conflicts_with`: Doctrines that can't be active simultaneously
- `synergizes_with`: Informational (for player guidance)
- `unlock_requirement`: Conditions to unlock (level, achievements, etc.)

**Validation Rules**:
- All IDs unique
- Commander IDs must exist in `commander_types.json`
- Effects use multipliers (1.0 = baseline)
- Conflicts must be mutual (if A conflicts with B, B conflicts with A)
- All doctrines should have meaningful tradeoffs
- No universal "best choice" doctrine

**Effect Types**:
- Multipliers: `move_speed_multiplier`, `damage_taken_multiplier`
- Absolute values: `fortify_armor_bonus`, `denial_dps`
- Thresholds: `retreat_health_threshold`, `outnumber_threshold`

**Balance Philosophy**:
- Every doctrine should have a cost or tradeoff
- Synergies encourage strategic combinations
- Conflicts prevent overpowered combinations

---

### 4. `ai_personalities.json` (8 personalities, ~400 lines)

**What it defines**: AI commander behavior patterns

**Key Fields**:
- `id`: aggressive, defensive, balanced, opportunistic, etc.
- `behavior_weights`: Priority values for different actions (0.0-1.0)
- `doctrine_preferences`: Category preferences for doctrine selection
- `decision_timing`: Reaction speed and planning horizon
- `combat_behavior`: Risk tolerance and engagement preferences
- `preferred_unit_types`: Unit IDs this AI favors
- `ability_usage`: How/when AI uses abilities

**Validation Rules**:
- All weights must be 0.0-1.0
- Unit type IDs must exist in `unit_types.json`
- Personalities should be distinct and recognizable
- At least 4 distinct personalities required

**Behavior Weight Examples**:
- `expand_territory: 0.9` = Very aggressive expansion
- `defend_position: 0.9` = Defensive focus
- `harass_enemy: 1.0` = Constant pressure
- `build_economy: 0.3` = Ignores economy

**Personality Types**:
- **Aggressive**: Early pressure, high risk
- **Defensive**: Turtle strategy, late-game focused
- **Balanced**: Adapts to situation
- **Opportunistic**: Waits for mistakes
- **Reckless**: All-in aggression
- **Turtle**: Extreme defense
- **Harasser**: Hit-and-run specialist
- **Economist**: Economy maximization

---

### 5. `maps.json` (3 maps, ~800 lines)

**What it defines**: Map layouts, sectors, objectives, and environmental effects

**Key Fields**:
- `id`: Unique map identifier
- `dimensions`: World width and height
- `factions`: Starting positions and sectors for each faction
- `sectors`: All sectors with connections, types, and properties
- `objectives`: Victory conditions
- `environmental_effects`: Terrain bonuses/hazards
- `recommended_ai_setup`: Suggested AI commanders and personalities

**Validation Rules**:
- Must have exactly 3 factions
- Sectors must form connected graph (no isolated sectors)
- Connections must be mutual
- Each faction needs 2-3 starting sectors
- At least 1 victory_point sector per map
- Sector types must be valid enum values

**Sector Types**:
- `starting`: Faction starting zones (fortified)
- `normal`: Standard capturable sectors
- `resource`: Provides supply generation bonus
- `chokepoint`: Defensive advantage, key strategic position
- `victory_point`: Required for victory objectives
- `strategic`: Special abilities (vision, spawn speed, etc.)

**Connection Rules**:
- If sector A connects to B, sector B must connect to A
- No isolated sectors (all must be reachable)
- Central sectors should be contested (no faction starts with them)

**Map Design Philosophy**:
- **Contested Valley**: Balanced 3-way, central objective
- **Asymmetric Highlands**: Player defensive advantage, resource focus
- **Chokepoint Gauntlet**: Linear progression, breakthrough challenge

---

## Cross-File References

The data files reference each other. All references are validated on load:

```
commander_types.json → doctrines.json (available_doctrines)
ai_personalities.json → unit_types.json (preferred_unit_types)
maps.json → all files (recommended_ai_setup references commanders and personalities)
```

**Validation**: On game startup, all cross-references are checked:
1. Commander doctrines exist in doctrine list
2. AI preferred units exist in unit list
3. Map AI setups reference valid commanders and personalities
4. Sector connections are mutual

**Error Handling**: If validation fails, game shows clear error message:
```
ERROR: Commander 'vanguard' references unknown doctrine 'super_doctrine'
  in file: assets/data/commander_types.json
  line: 12
```

---

## Data Loading System

### Implementation (in `src/config.rs`)

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
        // 1. Load JSON files
        let units = load_json("assets/data/unit_types.json")?;
        let commanders = load_json("assets/data/commander_types.json")?;
        let doctrines = load_json("assets/data/doctrines.json")?;
        let ai = load_json("assets/data/ai_personalities.json")?;
        let maps = load_json("assets/data/maps.json")?;

        // 2. Validate cross-references
        validate_references(&commanders, &doctrines)?;
        validate_references(&ai, &units)?;

        // 3. Build lookup tables
        Ok(Self {
            units: build_lookup(units),
            commanders: build_lookup(commanders),
            // ... etc
        })
    }
}
```

### Usage in Game

```rust
// At startup
let game_data = GameData::load().expect("Failed to load game data");

// Accessing data
let unit_def = game_data.units.get("infantry_light").unwrap();
let damage = unit_def.attack.damage;

// Spawning units based on data
let unit = spawn_unit_from_definition(unit_def, position, owner);
```

---

## Expanding the Data

### Adding a New Unit Type

1. Add entry to `unit_types.json`
2. Ensure unique ID
3. Set appropriate counters/countered_by
4. Balance supply cost vs power
5. Test in game

### Adding a New Doctrine

1. Add entry to `doctrines.json`
2. Choose compatible commanders
3. Define effects (use multipliers)
4. Set conflicts with overpowered combinations
5. Add synergies for strategic depth
6. Update commander's `available_doctrines` lists

### Adding a New Map

1. Add entry to `maps.json`
2. Design sector layout (8-12 sectors recommended)
3. Ensure all sectors are connected
4. Place strategic sectors (resources, chokepoints, victory points)
5. Define objectives
6. Test with different AI setups

---

## Balance Guidelines

### Unit Balance
- Supply cost should correlate with power
- Range typically: 5 (infantry) to 20 (artillery)
- Each unit should have clear role
- Counters create rock-paper-scissors
- Avoid strictly better units (each should excel in scenarios)

### Doctrine Balance
- Every doctrine should have a cost/tradeoff
- Offensive doctrines: increase damage but increase risk
- Defensive doctrines: increase survivability but decrease offense
- Economic doctrines: improve efficiency but limit flexibility
- Special doctrines: unique mechanics with drawbacks

### Map Balance
- Central contested zones create conflict
- Each faction should have path to victory
- Chokepoints force strategic decisions
- Resource sectors encourage expansion
- Victory point sectors force engagement

---

## Future Expansion Ideas

### Unit Types to Add
- Siege mechs (very slow, very high damage)
- Medic squads (heal nearby units)
- Recon vehicles (vision and speed)
- Mobile fortifications (movable cover)
- Kamikaze drones (one-time massive damage)

### Doctrine Categories to Add
- Terrain-specific doctrines
- Time-of-battle doctrines (early/mid/late game)
- Alliance doctrines (boost when allied)
- Experimental doctrines (high risk/reward)

### Map Features to Add
- Capturable neutral structures
- Destructible terrain
- Weather effects
- Day/night cycles
- Orbital bombardment zones

---

## Summary

The bulk data system provides:
- ✓ Clear separation of code and content
- ✓ Easy balance iteration
- ✓ Moddability foundation
- ✓ Type-safe data structures
- ✓ Comprehensive validation
- ✓ Strategic depth

All files are ready to expand as development progresses!
