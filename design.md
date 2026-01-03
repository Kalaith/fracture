# Fracture Command - Game Design Document

## Working Title
Fracture Command

## Genre
Strategic Co-op War Simulation  
*(Indirect RTS • Multi-Faction Conflict • Emergent Narrative)*

## High Concept
Fracture Command is a large-scale cooperative strategy game where players take the role of commanders, not micromanagers. Instead of controlling individual units, players shape the battlefield through doctrines, logistics, and high-level orders while autonomous forces carry out the war.

The conflict is not a simple two-sided battle. Multiple fractured coalitions clash simultaneously, forming unstable alliances, betrayals, and shifting fronts. Victory comes not from perfect micro, but from foresight, coordination, and knowing when to let the war burn.

## Core Hook
- 3–5 Commanders per faction
- Up to three factions active on a battlefield
- Asymmetric co-op across unstable alliances (e.g. 5v5v5)
- No unit micromanagement

Players collaborate within a faction while reacting to two others doing the same. Every decision ripples across the conflict.

## Gameplay Pillars

### Commander-Driven Warfare
Each player controls a commander with a distinct strategic role:
- Vanguard (frontline pressure)
- Engineer / Logistics (repairs, supply flow)
- Control (area denial, fortifications)
- Disruptor (sabotage, interference)
- Wildcard (experimental tech, rule-benders)

Commanders influence the war through auras, doctrines, and policies, not click-heavy abilities.

### Autonomous Units, Strategic Control
- Units operate in squads, not individually
- Orders are abstract: advance, hold, fortify, skirmish
- Units respond to morale, supply, and battlefield conditions automatically
- The player's role is to set intent, not babysit execution.

### Unit Supply as Commitment
Each faction has a hard cap of 200 unit supply:
- Supply represents mass, upkeep, and attention
- Most units are low-attention, fire-and-forget forces
- High-impact units cost more supply but require little micro
- Even at maximum scale, total on-screen forces remain readable and performant.

### Logistics Are the Real Battlefield
Control comes from:
- Supply lines
- Reinforcement timing
- Power and repair allocation
- Territory influence

Starve an enemy front, reinforce another, or let rivals exhaust each other before committing.

## Conflict Structure
- Not all commanders are active on the battlefield at once
- Some operate off-map, influencing economy, intel, or sabotage
- Commanders rotate in and out as fronts collapse or escalate
- Wars evolve in stages, escalating naturally rather than exploding instantly.

## Narrative Emergence
There is no scripted story. The story is what happens:
- Temporary alliances
- Strategic betrayals
- Rogue commanders
- Factions that win by delaying, not conquering

Every match writes its own post-mortem.

## Player Fantasy
You are not a general shouting orders. You are a commander setting forces in motion, watching plans collide, and choosing when to intervene.

Victory feels earned. Defeat feels inevitable. Both feel meaningful.

## Next Steps Recommendations
If you want next steps, I'd recommend:
- A single-commander vertical slice
- A 3-faction AI-only simulation to test readability
- Or a coop demo with logistics-only interaction

This pitch has legs. The trick now is letting the systems do the talking 🛰️

A single-commander vertical slice. Perfect. A single-commander vertical slice is exactly the right knife to cut this open 🔪 Small surface area, full flavor.

## Single-Commander Vertical Slice

### Goal of the Slice
Answer one question decisively: Is commanding intent more fun than micromanaging units?

If the answer is yes, everything else scales.

### Player Role
The player controls one Commander in a live battlefield with:
- 1 allied AI commander
- 2 enemy AI commanders  
*(3 factions total, minimal chaos)*

The player never selects individual units.

**Chosen Commander Type:** Vanguard

**Why Vanguard?**
- Direct battlefield impact
- Easy to read cause and effect
- Clear feedback when decisions succeed or fail

### Core Systems Included (Only What's Necessary)

#### 1. Unit Supply (100 Supply Cap)
Lower than final vision for clarity.

Example breakdown:
- Infantry Squad: 5 supply
- Heavy Squad: 10 supply
- Artillery Platform: 20 supply
- Support Drone Swarm: 10 supply

Player composes forces, then largely leaves them alone.

#### 2. Squad AI + Orders
Squads operate autonomously under one of four orders:
- Advance – push objectives
- Hold – defend position
- Skirmish – harass and disengage
- Fortify – entrench, build defenses

Orders are issued per front, not per unit.

#### 3. Commander Doctrines
Instead of abilities, the commander selects two active doctrines:

Examples:
- Aggressive Posture: faster advance, higher casualties
- Entrenched Assault: slower push, better survivability
- Shock Windows: brief damage spikes after reinforcements arrive

These are toggles, not cooldown spam.

#### 4. Logistics Light
Just enough to feel strategic:
- One supply hub
- One reinforcement route
- One repair throughput limit

If the player overcommits, fronts starve. If they pace, they grind forward.

### Battlefield Layout
- Single medium-sized map
- 3–4 strategic sectors
- One central contested objective
- No fog-of-war tricks. Clarity over cleverness.

### Enemy AI (Deliberately Simple)
Each enemy commander has:
- A fixed personality (e.g. reckless vs defensive)
- A visible doctrine bias
- Predictable reinforcement behavior

The goal is not to surprise the player, but to teach them to read the war.

### Win / Loss Conditions
**Victory:**
- Control the central objective for X time
- OR force both enemy commanders to retreat

**Loss:**
- Supply hub destroyed
- Reinforcements drop below recovery threshold

Clear, fast feedback.

### What This Slice Proves
By the end of a 20–30 minute match, you should know:
- Do players feel in control without micro?
- Do doctrines meaningfully change outcomes?
- Is watching autonomous squads satisfying?
- Does logistics pressure create interesting decisions?

If even two of those land, the project is viable.

### What Is Explicitly Out of Scope
- No tech tree
- No diplomacy UI
- No commander swapping
- No multiplayer networking
- No story systems

This slice is a pressure test, not a promise.

### Success Metric
A player says: "I lost because I committed to the wrong front… and I can see exactly why."

That sentence is the green light 🚦