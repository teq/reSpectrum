use std::{cell::RefCell, rc::Rc};

use super::{BusSignal, Clock, Identifier, RingBuff};

/// Type-erased value driven on a bus line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusValue {
    Word(u16),
    Byte(u8),
    Bit(bool),
}

impl BusValue {
    pub fn as_word(&self) -> u16 {
        match self { BusValue::Word(v) => *v, _ => panic!("expected word bus value") }
    }
    pub fn as_byte(&self) -> u8 {
        match self { BusValue::Byte(v) => *v, _ => panic!("expected byte bus value") }
    }
    pub fn as_bit(&self) -> bool {
        match self { BusValue::Bit(v) => *v, _ => panic!("expected bit bus value") }
    }
}

/// Convert a typed line value into the type-erased bus value.
pub trait IntoBusValue {
    fn into_bus_value(self) -> BusValue;
}

impl IntoBusValue for u16 {
    fn into_bus_value(self) -> BusValue { BusValue::Word(self) }
}
impl IntoBusValue for u8 {
    fn into_bus_value(self) -> BusValue { BusValue::Byte(self) }
}
impl IntoBusValue for bool {
    fn into_bus_value(self) -> BusValue { BusValue::Bit(self) }
}

/// A change event emitted when a bus line is driven or released.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BusChange {
    /// The bus line that changed
    pub signal: BusSignal,
    /// New owner and value, or `None` when the line was released (high impedance)
    pub state: Option<(Identifier, BusValue)>,
}

/// Listener notified on every bus line change.
pub trait BusSignalObserver {
    fn on_change(&self, change: BusChange);
}

/// A point-in-time snapshot of the whole bus.
#[derive(Debug, Default, Clone, Copy)]
pub struct BusReading {
    pub htcycles: u64,
    pub addr: Option<(Identifier, u16)>,
    pub data: Option<(Identifier, u8)>,
    pub mreq: Option<(Identifier, bool)>,
    pub iorq: Option<(Identifier, bool)>,
    pub rd: Option<(Identifier, bool)>,
    pub wr: Option<(Identifier, bool)>,
    pub rfsh: Option<(Identifier, bool)>,
    pub m1: Option<(Identifier, bool)>,
    pub busak: Option<(Identifier, bool)>,
    pub halt: Option<(Identifier, bool)>,
    pub wait: Option<(Identifier, bool)>,
    pub int: Option<(Identifier, bool)>,
    pub nmi: Option<(Identifier, bool)>,
    pub reset: Option<(Identifier, bool)>,
    pub busrq: Option<(Identifier, bool)>,
}

/// Collects bus changes into a history of [`BusReading`] snapshots.
///
/// Each change updates the tracked `current` snapshot and appends it to the
/// ring buffer, so only actual change points are recorded.
pub struct BusHistory<const N: usize> {
    clock: Rc<Clock>,
    current: RefCell<BusReading>,
    readings: RefCell<RingBuff<BusReading, N>>,
}

impl<const N: usize> BusHistory<N> {
    pub fn new(clock: &Rc<Clock>) -> Rc<Self> {
        Rc::new(Self {
            clock: Rc::clone(clock),
            current: RefCell::new(BusReading::default()),
            readings: RefCell::new(RingBuff::new()),
        })
    }

    /// Current (latest) snapshot of the bus.
    pub fn current(&self) -> BusReading {
        *self.current.borrow()
    }

    /// All recorded snapshots, oldest first, including the current (latest) snapshot.
    pub fn readings(&self) -> Vec<BusReading> {
        let mut all: Vec<BusReading> = self.readings.borrow().iter_to_head().collect();
        all.push(*self.current.borrow());
        all
    }
}

impl<const N: usize> BusSignalObserver for BusHistory<N> {
    fn on_change(&self, change: BusChange) {
        let htcycles = self.clock.get();
        let mut current = self.current.borrow_mut();

        if current.htcycles != htcycles {
            // Timestamp advanced: commit the previous snapshot and start a new one
            self.readings.borrow_mut().push(*current);
            current.htcycles = htcycles;
        }

        match change.signal {
            BusSignal::ADDR => current.addr = change.state.map(|(id, v)| (id, v.as_word())),
            BusSignal::DATA => current.data = change.state.map(|(id, v)| (id, v.as_byte())),
            BusSignal::MREQ => current.mreq = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::IORQ => current.iorq = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::RD => current.rd = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::WR => current.wr = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::RFSH => current.rfsh = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::M1 => current.m1 = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::BUSAK => current.busak = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::HALT => current.halt = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::WAIT => current.wait = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::INT => current.int = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::NMI => current.nmi = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::RESET => current.reset = change.state.map(|(id, v)| (id, v.as_bit())),
            BusSignal::BUSRQ => current.busrq = change.state.map(|(id, v)| (id, v.as_bit())),
        }
    }
}
