# AGENTS.md

This file provides guidance to Codex (Codex.ai/code) when working with code in this repository.

This project uses the shared RustGames agent instructions in [`../AGENTS.md`](../AGENTS.md). Codex should read and apply that file when working here.

## Project Overview

**Fracture Command** is a strategic co-op war simulation game where players command autonomous forces through high-level doctrines and logistics rather than micromanagement. The current implementation target is a 1v1 networked prototype.

### Core Concept
- Players control Commanders, not individual units
- Units operate autonomously under strategic orders (Advance, Hold, Skirmish, Fortify)
- Success comes from doctrine selection, logistics management, and strategic positioning
- No unit micromanagement - the goal is to test if "commanding intent is more fun than micromanaging units"

## Tech Stack

- **Graphics Engine**: Macroquad (2D rendering, input, game loop)
- **Networking**: Laminar (UDP-based, low-latency)
- **Async Runtime**: Tokio (network I/O)
- **Serialization**: Serde + Bincode
- **Language**: Rust

## Architecture

### Module Structure
```
src/
├── main.rs          # Entry point, initialization
├── game/            # Game state and coordination
│   ├── mod.rs       # Main game state, public API
│   └── state.rs     # State management (if needed)
├── simulation/      # Unit AI, combat, doctrines
│   ├── mod.rs       # Simulation coordinator
│   ├── ai.rs        # Autonomous unit AI
│   └── combat.rs    # Combat resolution
├── rendering/       # Drawing and visual effects
│   ├── mod.rs       # Rendering coordinator
│   ├── world.rs     # World rendering
│   └── ui.rs        # UI panels and HUD
├── input/           # Input handling
│   └── mod.rs       # Input processing
├── network/         # Networking (Phase 3)
│   └── mod.rs       # Network manager
├── types.rs         # Shared data structures
└── config.rs        # Game constants
```

**Organization Philosophy:**
- Use subdirectories for domain grouping (game/, simulation/, rendering/)
- Max 1-2 levels of nesting - avoid deep hierarchies
- `mod.rs` serves as the public interface for each subdirectory
- Split modules when they exceed 500 lines or have multiple concerns

### Networking Model
- **Deterministic Simulation**: Both clients run identical simulations
- **Input Synchronization**: Players exchange input commands each frame
- **Peer-to-Peer**: No central server
- **Target Latency**: <50ms average, <5% packet loss tolerance

### Key Systems

**Supply System**
- 100 supply cap per player (simplified from 200 in full vision)
- Units have supply costs (Infantry: 5, Heavy: 10, Artillery: 20, Drones: 10)
- Supply represents mass, upkeep, and attention

**Commander Doctrines**
- Commanders select 2 active doctrines (toggles, not cooldowns)
- Examples: Aggressive Posture, Entrenched Assault, Shock Windows
- Doctrines change strategic behavior, not tactical micro

**Autonomous Squad AI**
- Units operate under one of four orders per front:
  - **Advance**: Push objectives
  - **Hold**: Defend position
  - **Skirmish**: Harass and disengage
  - **Fortify**: Entrench, build defenses
- Orders are issued per front, not per unit
- Squads respond to morale, supply, and battlefield conditions automatically

**Logistics**
- Supply hubs and reinforcement routes
- Repair throughput limits
- Overcommitting causes fronts to starve

## Development Commands

*Note: Project is in early stages - no build/test commands yet. Update this section as the project develops.*

### When Project Has Code

Expected commands (update when implemented):
```bash
# Build the project
cargo build

# Run in development mode
cargo run

# Run tests
cargo test

# Run with release optimizations
cargo run --release
```

## Key Design Constraints

### What This Prototype Tests
1. Is commanding intent more fun than micromanaging units?
2. Do doctrines meaningfully change outcomes?
3. Is watching autonomous squads satisfying?
4. Does logistics pressure create interesting decisions?

### Scope Boundaries
**In Scope for 1v1 Prototype:**
- Single Vanguard commander per player
- Autonomous squad AI with 4 order types
- Commander doctrine system (2 active per commander)
- Basic supply and logistics
- Peer-to-peer networking
- Single medium-sized map with 3-4 sectors

**Explicitly Out of Scope:**
- Tech trees
- Diplomacy UI
- Commander swapping
- Multi-faction battles (3+ factions)
- Story systems
- Multiple commander types initially

### Success Metric
A player should be able to say: "I lost because I committed to the wrong front... and I can see exactly why."

## Technical Challenges

### Async Integration
- Macroquad runs on single-threaded async runtime
- Use Tokio for networking in separate thread
- Communicate via channels between network and game threads
- Never block the game loop

### Deterministic Simulation
- Use fixed-point math for positions/calculations
- Seed RNGs identically on both clients
- Send only inputs, not full state updates
- Both clients must produce identical results from same inputs

### Networking Best Practices
- Send inputs immediately on change
- Buffer and reorder packets to handle jitter
- Implement basic client-side prediction
- Handle reconnection and pause on disconnect

## Game Balance Targets

- Match duration: 10-15 minutes
- Network latency: <50ms average
- Unit readability: All forces visible and trackable at max supply (100 per player)
- Feedback clarity: Cause and effect must be obvious

## Philosophy

### Design Principles
- **Clarity over cleverness**: Players should understand what's happening
- **Indirect control**: Shape forces through doctrine and logistics, not micro
- **Meaningful loss**: Defeats should feel inevitable but understandable
- **Emergent narrative**: Story comes from what happens, not scripts

### Code Principles
- Modular architecture
- Comprehensive error handling
- Profile early for performance
- Keep scope minimal - focus on core gameplay validation

## Coding Standards

All code must align with the project's Rust coding standards. Key highlights:
- **Readability over cleverness**: Prefer clear, straightforward code over clever optimizations
- **Module responsibilities are strict**: Each module has a single, well-defined purpose (see Section 2.1 of CODE_STANDARDS.md)
- **Use subdirectories for organization**: Group related modules in subdirectories (game/, rendering/, etc.) but avoid deep nesting (max 1-2 levels). See Section 2.3 of CODE_STANDARDS.md
- **Target 200-400 lines per file, max 800**: Keep files focused and manageable
- **Functions target 20-50 lines, max 100**: Break down complex logic into smaller, focused functions
- **UI code is "dumb"**: UI components read state and emit actions, but contain no business logic

## Macroquad Toolkit

This project uses the shared `macroquad-toolkit` crate located at `../macroquad-toolkit/` in the workspace. It provides common utilities for Macroquad development:

### Key Features to Use
- **Input utilities**: `is_hovered()`, `was_clicked()`, `InputState::capture()`
- **UI rendering**: `button()`, `panel()`, `progress_bar()` with consistent dark theme
- **Asset management**: `AssetManager` for texture loading and caching
- **Camera2D**: Pan and zoom support for 2D games
- **Event bus**: Generic event system for decoupled game logic
- **Color palettes**: Consistent dark theme colors (`dark::BACKGROUND`, `dark::PANEL`, etc.)
- **Sprite system**: Builder pattern for texture rendering with transformations

### Usage Pattern
```rust
use macroquad_toolkit::prelude::*;

// In Cargo.toml
macroquad-toolkit = { path = "../macroquad-toolkit" }

// Basic usage
let mut assets = AssetManager::new();
assets.load_texture("unit", "assets/unit.png").await.ok();

// UI with toolkit
if button(10.0, 10.0, 100.0, 40.0, "Deploy") {
    // Handle action
}
```

Prefer toolkit components over custom implementations to maintain consistency across games.
