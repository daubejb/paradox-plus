pub const MAX_SLIDES: usize = 16;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideError {
    CycleDetected,
    LimitExceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlideTracker {
    visited_cells: [Option<usize>; MAX_SLIDES],
    slide_count: usize,
}

impl SlideTracker {
    pub fn new() -> Self {
        Self {
            visited_cells: [None; MAX_SLIDES],
            slide_count: 0,
        }
    }

    pub fn slide_count(&self) -> usize {
        self.slide_count
    }

    /// Safely records cell visits using bounded array access to prevent panic overflows.
    pub fn record_and_check_cycle(&mut self, cell_index: usize) -> Result<(), SlideError> {
        if self.slide_count >= MAX_SLIDES {
            return Err(SlideError::LimitExceeded);
        }
        for i in 0..self.slide_count {
            if let Some(visited) = self.visited_cells[i] {
                if visited == cell_index {
                    return Err(SlideError::CycleDetected);
                }
            }
        }
        self.visited_cells[self.slide_count] = Some(cell_index);
        self.slide_count += 1;
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MovementDirection {
    Forward,
    Reverse,
}
