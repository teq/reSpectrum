use std::{ops::Deref, rc::Rc};

use super::{Clock, CpuBus};

/// Combined device interface: the CPU bus and the system clock.
/// Devices communicate over the bus and synchronize against the clock, so both
/// are carried together.
pub struct BusState {
    cpu_bus: Rc<CpuBus>,
    pub clock: Rc<Clock>,
}

impl BusState {
    /// Create a new bus state wrapping the given bus and clock
    pub fn new(bus: &Rc<CpuBus>, clock: &Rc<Clock>) -> Self {
        Self {
            cpu_bus: Rc::clone(bus),
            clock: Rc::clone(clock),
        }
    }
}

impl Deref for BusState {
    type Target = CpuBus;

    fn deref(&self) -> &Self::Target {
        &self.cpu_bus
    }
}

impl Default for BusState {
    fn default() -> Self {
        Self {
            cpu_bus: Default::default(),
            clock: Default::default(),
        }
    }
}
