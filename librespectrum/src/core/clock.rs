use std::cell::Cell;

/// System clock. Counts cycles with half T-cycle precision.
#[derive(Default)]
pub struct Clock {

    /// Half T-cycles count since system start. Odd on rising, even on falling.
    htcycles: Cell<u64>,

}

impl Clock {

    /// Set clock in half t-cycles
    pub fn set(&self, val: u64) {
        self.htcycles.set(val);
    }

    /// Get clock in half t-cycles
    pub fn get(&self) -> u64 {
        self.htcycles.get()
    }

    /// Get offset in half t-cycles to the next Nth t-cycle rising edge
    pub fn to_rising(&self, n: u64) -> u64 {
        (n << 1) - (!self.get() & 1)
    }

    /// Get offset in half t-cycles to the next Nth t-cycle falling edge
    pub fn to_falling(&self, n: u64) -> u64 {
        (n << 1) - (self.get() & 1)
    }

}
