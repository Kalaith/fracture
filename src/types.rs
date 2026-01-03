    /// Set initial control from faction index (1-3)
    pub fn set_initial_control(&mut self, faction_index: u32) {
        self.control = FactionId::from_index((faction_index - 1) as usize);
        self.control_progress = match self.control {
            Some(FactionId::Faction1) => -1.0,
            Some(FactionId::Faction2) => 0.0,
            Some(FactionId::Faction3) => 1.0,
            None => 0.0,
        };
    }
}

/// Strategic marker type for player control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerType {
    Attack,  // Units prioritize attacking this sector
    Defend,  // Units prioritize defending this sector
}

/// Strategic marker placed by player
#[derive(Debug, Clone)]
pub struct StrategicMarker {
    pub sector_id: u32,
    pub marker_type: MarkerType,
    pub faction: FactionId,
}

impl StrategicMarker {
    pub fn new(sector_id: u32, marker_type: MarkerType, faction: FactionId) -> Self {
        Self {
            sector_id,
            marker_type,
            faction,
        }
    }
}
