use std::{cell::{Cell, RefCell}, rc::Rc};

use super::{BusChange, BusSignal, BusSignalObserver, Identifiable, Identifier, IntoBusValue};

/// Signal bus line
pub struct BusLine<T> {

    /// Line signal
    signal: BusSignal,

    /// Line owner and state
    state: Cell<Option<(Identifier, T)>>,

    /// Registered change observers
    observers: Rc<RefCell<Vec<Rc<dyn BusSignalObserver>>>>,

}

impl<T: Copy + IntoBusValue + PartialEq> BusLine<T> {

    /// Create new bus line
    pub fn new(signal: BusSignal) -> Self {
        Self {
            signal,
            state: Cell::new(None),
            observers: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Subscribe to changes on this line
    pub fn observe(&self, observer: Rc<dyn BusSignalObserver>) {
        self.observers.borrow_mut().push(observer);
    }

    /// Get line state (owner and value)
    pub fn state(&self) -> Option<(Identifier, T)> {
        self.state.get()
    }

    /// Get line owner (if any)
    pub fn owner(&self) -> Option<Identifier> {
        self.state.get().and_then(|(owner, ..)| Some(owner))
    }

    /// Probe signal line
    pub fn probe(&self) -> Option<T> {
        self.state.get().and_then(|(.., value)| Some(value))
    }

    /// Expect signal on the line
    pub fn expect(&self) -> T {
        self.probe().unwrap()
    }

    /// Drive signal line
    pub fn drive<U: Identifiable>(&self, device: &U, value: T) {
        match self.state.get() {
            None => self.change(Some((device.id(), value))),
            Some((owner, current_value)) if owner == device.id() => {
                if current_value != value {
                    self.change(Some((owner, value)));
                }
            }
            Some((owner, ..)) => panic!("Device {} tries to drive the line {} owned by {}", device.id(), self.signal, owner)
        }
    }

    /// Release signal line
    pub fn release<U: Identifiable>(&self, device: &U) {
        match self.state.get() {
            Some((owner, ..)) if owner == device.id() => self.change(None),
            _ => ()
        }
    }

    /// Change the state and notify all observers
    fn change(&self, new_state: Option<(Identifier, T)>) {
        self.state.set(new_state);
        let change_event = BusChange {
            signal: self.signal,
            state: self.state.get().map(|(id, value)| (id, value.into_bus_value())),
        };
        for observer in self.observers.borrow().iter() {
            observer.on_change(change_event);
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::BusValue;

    fn mkline() -> BusLine::<bool> { BusLine::<bool>::new(BusSignal::DATA) }

    struct TestDevice {
        id: Identifier
    }

    impl Identifiable for TestDevice {
        fn id(&self) -> Identifier { self.id }
    }

    static DEV1: TestDevice = TestDevice { id: 1 };
    static DEV2: TestDevice = TestDevice { id: 2 };

    #[test]
    fn line_drive_and_probe() {
        let line = mkline();
        assert_eq!(line.probe(), None);
        line.drive(&DEV1, false);
        assert_eq!(line.probe(), Some(false));
        line.drive(&DEV1, true);
        assert_eq!(line.probe(), Some(true));
        line.release(&DEV1);
        assert_eq!(line.probe(), None);
        line.drive(&DEV2, true);
        assert_eq!(line.probe(), Some(true));
    }

    #[test]
    #[should_panic]
    fn only_one_device_can_drive_the_line() {
        let line = mkline();
        line.drive(&DEV1, false);
        line.drive(&DEV2, true);
    }

    #[test]
    fn notifies_observers_of_change() {
        let line = mkline();
        let changes: Rc<RefCell<Vec<BusChange>>> = Rc::new(RefCell::new(Vec::new()));

        struct Spy(Rc<RefCell<Vec<BusChange>>>);
        impl BusSignalObserver for Spy {
            fn on_change(&self, change: BusChange) {
                self.0.borrow_mut().push(change);
            }
        }

        line.observe(Rc::new(Spy(Rc::clone(&changes))));
        line.drive(&DEV1, true);

        let recorded = changes.borrow();
        assert_eq!(recorded.len(), 1);
        assert_eq!(recorded[0].signal, BusSignal::DATA);
        assert_eq!(recorded[0].state, Some((DEV1.id, BusValue::Bit(true))));
    }

    #[test]
    fn no_duplicate_notification_or_panic_on_redundant_drive() {
        let line = mkline();
        let changes: Rc<RefCell<Vec<BusChange>>> = Rc::new(RefCell::new(Vec::new()));

        struct Spy(Rc<RefCell<Vec<BusChange>>>);
        impl BusSignalObserver for Spy {
            fn on_change(&self, change: BusChange) {
                self.0.borrow_mut().push(change);
            }
        }

        line.observe(Rc::new(Spy(Rc::clone(&changes))));
        line.drive(&DEV1, true);
        line.drive(&DEV1, true); // redundant: same owner, same value

        let recorded = changes.borrow();
        assert_eq!(recorded.len(), 1, "redundant drive must not notify again");
    }

    #[test]
    fn no_notification_when_releasing_unowned_line() {
        let line = mkline();
        let changes: Rc<RefCell<Vec<BusChange>>> = Rc::new(RefCell::new(Vec::new()));

        struct Spy(Rc<RefCell<Vec<BusChange>>>);
        impl BusSignalObserver for Spy {
            fn on_change(&self, change: BusChange) {
                self.0.borrow_mut().push(change);
            }
        }

        line.observe(Rc::new(Spy(Rc::clone(&changes))));
        line.release(&DEV1); // DEV1 doesn't own the line yet

        assert!(changes.borrow().is_empty(), "release of unowned line must not notify");
    }

}
