use std::rc::Rc;

use super::{BusLine, BusSignalObserver};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum BusSignal {
    ADDR, DATA, MREQ, IORQ, RD, WR, RFSH,
    M1, BUSAK, HALT, WAIT, INT, NMI, RESET, BUSRQ
}

impl std::fmt::Display for BusSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Z80 CPU bus
pub struct CpuBus {
    /// Address bus (tri-state outputs)
    pub addr: BusLine<u16>,
    /// Data bus (tri-state in/outputs)
    pub data: BusLine<u8>,
    /// MREQ output (tri-state)
    pub mreq: BusLine<bool>,
    /// IORQ output (tri-state)
    pub iorq: BusLine<bool>,
    /// RD output (tri-state)
    pub rd: BusLine<bool>,
    /// WR output (tri-state)
    pub wr: BusLine<bool>,
    /// RFSH output
    pub rfsh: BusLine<bool>,
    /// M1 output
    pub m1: BusLine<bool>,
    /// BUSAK output
    pub busak: BusLine<bool>,
    /// HALT output
    pub halt: BusLine<bool>,
    /// WAIT input
    pub wait: BusLine<bool>,
    /// INT input
    pub int: BusLine<bool>,
    /// NMI input
    pub nmi: BusLine<bool>,
    /// RESET input
    pub reset: BusLine<bool>,
    /// BUSRQ input
    pub busrq: BusLine<bool>,
}

impl CpuBus {
    pub fn log(&self, _message: String) {
        // Logging is currently disabled
    }

    /// Subscribe an observer to every bus line
    pub fn observe(&self, observer: Rc<dyn BusSignalObserver>) {
        self.addr.observe(Rc::clone(&observer));
        self.data.observe(Rc::clone(&observer));
        self.mreq.observe(Rc::clone(&observer));
        self.iorq.observe(Rc::clone(&observer));
        self.rd.observe(Rc::clone(&observer));
        self.wr.observe(Rc::clone(&observer));
        self.rfsh.observe(Rc::clone(&observer));
        self.m1.observe(Rc::clone(&observer));
        self.busak.observe(Rc::clone(&observer));
        self.halt.observe(Rc::clone(&observer));
        self.wait.observe(Rc::clone(&observer));
        self.int.observe(Rc::clone(&observer));
        self.nmi.observe(Rc::clone(&observer));
        self.reset.observe(Rc::clone(&observer));
        self.busrq.observe(Rc::clone(&observer));
    }
}

impl Default for CpuBus {
    fn default() -> Self {
        Self {
            addr: BusLine::new(BusSignal::ADDR),
            data: BusLine::new(BusSignal::DATA),
            mreq: BusLine::new(BusSignal::MREQ),
            iorq: BusLine::new(BusSignal::IORQ),
            rd: BusLine::new(BusSignal::RD),
            wr: BusLine::new(BusSignal::WR),
            rfsh: BusLine::new(BusSignal::RFSH),
            m1: BusLine::new(BusSignal::M1),
            busak: BusLine::new(BusSignal::BUSAK),
            halt: BusLine::new(BusSignal::HALT),
            wait: BusLine::new(BusSignal::WAIT),
            int: BusLine::new(BusSignal::INT),
            nmi: BusLine::new(BusSignal::NMI),
            reset: BusLine::new(BusSignal::RESET),
            busrq: BusLine::new(BusSignal::BUSRQ),
        }
    }
}
