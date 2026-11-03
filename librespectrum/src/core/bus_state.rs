use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::{BusHistory, BusReading, BusSignalObserver, Clock, CpuBus};

/// Combined device interface: the CPU bus and the system clock.
/// Devices communicate over the bus and synchronize against the clock, so both
/// are carried together.
pub struct BusState {
    cpu_bus: Rc<CpuBus>,
    pub clock: Rc<Clock>,
    /// Change-point history collected from the bus
    history: RefCell<Rc<BusHistory<96>>>,
}

impl BusState {
    /// Create a new bus state wrapping the given bus and clock
    pub fn new(bus: &Rc<CpuBus>, clock: &Rc<Clock>) -> Self {
        let history = BusHistory::<96>::new(clock);
        let observer: Rc<dyn BusSignalObserver> = history.clone();
        bus.observe(observer);

        Self {
            cpu_bus: Rc::clone(bus),
            clock: Rc::clone(clock),
            history: RefCell::new(history),
        }
    }

    /// Bus change-point history
    pub fn history(&self) -> Rc<BusHistory<96>> {
        Rc::clone(&self.history.borrow())
    }

    /// Latest snapshot of the bus
    pub fn snapshot(&self) -> BusReading {
        self.history().current()
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
        let bus: Rc<CpuBus> = Default::default();
        let clock: Rc<Clock> = Default::default();
        Self::new(&bus, &clock)
    }
}
