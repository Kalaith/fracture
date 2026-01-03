//! AI system for autonomous commanders
//!
//! This module implements AI decision-making for computer-controlled commanders.
//! AI commanders use personality profiles to make strategic decisions about:
//! - Unit deployment and composition
//! - Doctrine selection
//! - Squad orders and front priorities
//! - Resource allocation
//! - Ability usage
//!
//! The AI is designed to be challenging but readable - players should understand
//! what the AI is doing and why.

mod commander_ai;
mod personality;
mod strategic;

pub use commander_ai::CommanderAI;
pub use personality::AIPersonality;

use crate::data::GameData;
use crate::game::GameState;
use crate::types::*;

/// AI coordinator - manages all AI commanders
pub struct AICoordinator {
    pub ai_commanders: Vec<CommanderAI>,
}

impl AICoordinator {
    pub fn new() -> Self {
        Self {
            ai_commanders: Vec::new(),
        }
    }

    /// Initialize AI commanders from game data
    pub fn initialize_from_data(
        &mut self,
        game_data: &GameData,
        commanders: &[Commander],
        ai_personality_ids: &[String],
    ) {
        self.ai_commanders.clear();

        let mut ai_index = 0;
        for commander in commanders.iter() {
            if commander.is_ai() {
                let personality_id = ai_personality_ids
                    .get(ai_index)
                    .and_then(|id| game_data.ai_personalities.get(id))
                    .expect("AI personality not found");

                let personality = AIPersonality::from_definition(personality_id);
                let ai_commander = CommanderAI::new(commander.faction_id, personality);

                self.ai_commanders.push(ai_commander);
                ai_index += 1;
            }
        }
    }

    /// Update all AI commanders
    pub fn update(&mut self, game: &mut GameState, dt: f32) {
        for ai in &mut self.ai_commanders {
            ai.update(game, dt);
        }
    }
}
