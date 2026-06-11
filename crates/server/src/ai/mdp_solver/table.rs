use fixed::types::I32F32;

pub struct MdpSolverTable {
    pub values: Box<[I32F32]>,
}

impl MdpSolverTable {
    pub fn new() -> Self {
        // Pre-allocate capacity and resize to prevent stack copying
        let mut vec = Vec::with_capacity(131_072);
        vec.resize(131_072, I32F32::ZERO);
        Self {
            values: vec.into_boxed_slice(),
        }
    }

    /// Reset all values to zero.
    pub fn clear(&mut self) {
        self.values.fill(I32F32::ZERO);
    }
}
