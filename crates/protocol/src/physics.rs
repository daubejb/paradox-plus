use fixed::types::I32F32;

pub const MAX_SLIDES: usize = 16;

pub trait SafeFixedMath {
    fn safe_add(self, other: Self) -> Self;
    fn safe_sub(self, other: Self) -> Self;
    fn safe_mul(self, other: Self) -> Self;
    fn safe_div(self, other: Self) -> Self;
}

impl SafeFixedMath for I32F32 {
    #[inline]
    fn safe_add(self, other: Self) -> Self {
        self.checked_add(other).unwrap_or(I32F32::MAX)
    }

    #[inline]
    fn safe_sub(self, other: Self) -> Self {
        self.checked_sub(other).unwrap_or(I32F32::MIN)
    }

    #[inline]
    fn safe_mul(self, other: Self) -> Self {
        self.checked_mul(other).unwrap_or_else(|| {
            if (self < 0) == (other < 0) { I32F32::MAX } else { I32F32::MIN }
        })
    }

    #[inline]
    fn safe_div(self, other: Self) -> Self {
        if other == I32F32::ZERO {
            if self < 0 { I32F32::MIN } else { I32F32::MAX }
        } else {
            self.checked_div(other).unwrap_or_else(|| {
                if (self < 0) == (other < 0) { I32F32::MAX } else { I32F32::MIN }
            })
        }
    }
}

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
