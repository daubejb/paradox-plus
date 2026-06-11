use protocol::physics::MovementDirection;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MdpState {
    pub cell_index: u16,
    pub direction: MovementDirection,
    pub origin_cell: u16,
    pub triggered_wagers: heapless::Vec<u16, 4>,
}

impl MdpState {
    pub fn new(
        cell_index: u16,
        direction: MovementDirection,
        origin_cell: u16,
        triggered_wagers: heapless::Vec<u16, 4>,
    ) -> Self {
        Self {
            cell_index,
            direction,
            origin_cell,
            triggered_wagers,
        }
    }

    /// Records a triggered wager cell index to damper infinite loops.
    pub fn record_triggered_wager(&mut self, cell: u16) {
        if !self.triggered_wagers.contains(&cell) {
            let _ = self.triggered_wagers.push(cell);
        }
    }

    /// Maps the state to a unique flat index under 131,072.
    /// Uses cell_index and direction only.
    pub fn to_index(&self) -> Option<usize> {
        let dir_val = match self.direction {
            MovementDirection::Forward => 0,
            MovementDirection::Reverse => 1,
        };
        let cell = self.cell_index as usize;
        if cell >= 65536 {
            return None;
        }
        Some((cell << 1) | dir_val)
    }

    /// Reconstructs a base state from a flat index under 131,072.
    /// Sets origin_cell to cell_index and triggered_wagers to empty.
    pub fn from_index(index: usize) -> Option<Self> {
        if index >= 131072 {
            return None;
        }
        let cell_index = (index >> 1) as u16;
        let direction = if (index & 1) == 0 {
            MovementDirection::Forward
        } else {
            MovementDirection::Reverse
        };
        Some(Self {
            cell_index,
            direction,
            origin_cell: cell_index,
            triggered_wagers: heapless::Vec::new(),
        })
    }
}
