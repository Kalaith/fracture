//! Strategic analysis - battlefield assessment for AI decision-making

use crate::game::GameState;
use crate::types::*;

/// Strategic analysis of the current battlefield state
pub struct StrategicAnalysis {
    pub faction_strength: [f32; 3],      // Military strength per faction
    pub sector_control: [u32; 3],        // Number of sectors controlled
    pub supply_advantage: [f32; 3],      // Supply advantage/disadvantage
    pub threat_level: [f32; 3],          // Threat from each faction
    pub expansion_opportunities: Vec<u32>, // Sector IDs worth capturing
    pub vulnerable_sectors: Vec<u32>,    // Our sectors under threat
}

impl StrategicAnalysis {
    /// Analyze the current game state
    pub fn analyze(game: &GameState, for_faction: FactionId) -> Self {
        let mut analysis = Self {
            faction_strength: [0.0; 3],
            sector_control: [0; 3],
            supply_advantage: [0.0; 3],
            threat_level: [0.0; 3],
            expansion_opportunities: Vec::new(),
            vulnerable_sectors: Vec::new(),
        };

        // Calculate faction strengths
        for squad in &game.squads {
            let faction_idx = squad.owner.index();
            let squad_strength: f32 = squad
                .units
                .iter()
                .map(|u| u.health / u.unit_type.max_health())
                .sum();
            analysis.faction_strength[faction_idx] += squad_strength;
        }

        // Count sector control
        for sector in &game.sectors {
            if let Some(faction) = sector.control {
                analysis.sector_control[faction.index()] += 1;
            }
        }

        // Calculate supply advantage
        for faction in FactionId::all() {
            let commander = game.get_commander(faction);
            let supply_ratio = commander.available_supply() as f32 / commander.supply_max as f32;
            analysis.supply_advantage[faction.index()] = supply_ratio;
        }

        // Calculate threat levels for our faction
        let our_idx = for_faction.index();
        for faction in FactionId::all() {
            if faction != for_faction {
                let threat_idx = faction.index();
                // Threat is based on their strength vs our strength
                let strength_ratio = analysis.faction_strength[threat_idx]
                    / analysis.faction_strength[our_idx].max(1.0);
                analysis.threat_level[threat_idx] = strength_ratio;
            }
        }

        // Find expansion opportunities (uncaptured or weakly held sectors)
        for (i, sector) in game.sectors.iter().enumerate() {
            if sector.control != Some(for_faction) {
                // Count enemy units in sector
                let enemy_count = game
                    .squads
                    .iter()
                    .filter(|s| s.owner != for_faction)
                    .flat_map(|s| &s.units)
                    .filter(|u| sector.contains_point(u.position))
                    .count();

                if enemy_count < 5 {
                    // Lightly defended
                    analysis.expansion_opportunities.push(i as u32);
                }
            }
        }

        // Find vulnerable sectors (our sectors with enemy presence)
        for (i, sector) in game.sectors.iter().enumerate() {
            if sector.control == Some(for_faction) {
                let enemy_count = game
                    .squads
                    .iter()
                    .filter(|s| s.owner != for_faction)
                    .flat_map(|s| &s.units)
                    .filter(|u| sector.contains_point(u.position))
                    .count();

                if enemy_count > 0 {
                    analysis.vulnerable_sectors.push(i as u32);
                }
            }
        }

        analysis
    }

    /// Get the strongest enemy faction
    pub fn strongest_enemy(&self, our_faction: FactionId) -> FactionId {
        let our_idx = our_faction.index();
        let mut strongest = FactionId::Faction1;
        let mut max_strength = 0.0;

        for faction in FactionId::all() {
            if faction != our_faction {
                let idx = faction.index();
                if self.faction_strength[idx] > max_strength {
                    max_strength = self.faction_strength[idx];
                    strongest = faction;
                }
            }
        }

        strongest
    }

    /// Get the weakest enemy faction
    pub fn weakest_enemy(&self, our_faction: FactionId) -> FactionId {
        let mut weakest = FactionId::Faction1;
        let mut min_strength = f32::MAX;

        for faction in FactionId::all() {
            if faction != our_faction {
                let idx = faction.index();
                if self.faction_strength[idx] < min_strength {
                    min_strength = self.faction_strength[idx];
                    weakest = faction;
                }
            }
        }

        weakest
    }

    /// Should we be on the defensive?
    pub fn should_defend(&self, our_faction: FactionId) -> bool {
        let our_idx = our_faction.index();

        // Defend if we're significantly weaker than enemies
        for faction in FactionId::all() {
            if faction != our_faction {
                let enemy_idx = faction.index();
                if self.faction_strength[enemy_idx] > self.faction_strength[our_idx] * 1.5 {
                    return true;
                }
            }
        }

        // Defend if we have vulnerable sectors
        !self.vulnerable_sectors.is_empty() && self.vulnerable_sectors.len() > 1
    }

    /// Should we expand?
    pub fn should_expand(&self, our_faction: FactionId) -> bool {
        let our_idx = our_faction.index();

        // Expand if we have supply advantage and expansion opportunities
        self.supply_advantage[our_idx] > 0.3 && !self.expansion_opportunities.is_empty()
    }
}
