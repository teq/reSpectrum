use super::BusLine;

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
}

impl Default for CpuBus {
    fn default() -> Self {
        Self {
            addr: BusLine::new("ADDR"),
            data: BusLine::new("DATA"),
            mreq: BusLine::new("MREQ"),
            iorq: BusLine::new("IORQ"),
            rd: BusLine::new("RD"),
            wr: BusLine::new("WR"),
            rfsh: BusLine::new("RFSH"),
            m1: BusLine::new("M1"),
            busak: BusLine::new("BUSAK"),
            halt: BusLine::new("HALT"),
            wait: BusLine::new("WAIT"),
            int: BusLine::new("INT"),
            nmi: BusLine::new("NMI"),
            reset: BusLine::new("RESET"),
            busrq: BusLine::new("BUSRQ"),
        }
    }
}
