use memory::{MemoryMappedIO, Memory};
use std::cell::Cell;

static TRUE: bool = true;
pub trait Source {
    fn load(&self) -> StandardControllerState;
}

pub struct StandardController<'a> {
    source: &'a Source,

    state_loaded: bool,
    state_index: Cell<u8>,
    state: StandardControllerState,
}

impl <'a> StandardController<'a> {
    pub fn new(source: &'a Source) -> StandardController<'a> {
        StandardController {
            source: source,
            state_loaded: false,
            state_index: Cell::new(0),
            state: StandardControllerState {
                a_button: false,
                b_button: false,
                start: false,
                select: false,
                up: false,
                down: false,
                left: false,
                right: false,
            }
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct StandardControllerState {
    pub a_button: bool,
    pub b_button: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

use std::ops::Index;
impl Index<u8> for StandardControllerState {
    type Output = bool;
    fn index(&self, index: u8) -> &bool {
        match index {
            0 => &self.a_button,
            1 => &self.b_button,
            2 => &self.select,
            3 => &self.start,
            4 => &self.up,
            5 => &self.down,
            6 => &self.left,
            7 => &self.right,
            _ => &TRUE,
        }
    }
}

impl <'a> MemoryMappedIO for StandardController<'a> {
    fn read(&self, _: &Memory) -> u8 {
        if self.state_loaded {
            let state_index = self.state_index.get();
            let result = self.state[state_index] as u8;
            self.state_index.set(state_index + 1);
            return result;
        } else {
            return self.source.load().a_button as u8;
        }
    }

    fn write(&mut self, _: &mut Memory, value: u8) {
        let strobe_bit = value & 0x01;
        if strobe_bit == 0 && !self.state_loaded {
            self.state = self.source.load();
            self.state_index.set(0);
        }
        self.state_loaded = if strobe_bit == 0  { true } else { false };
    }
}

#[cfg(test)]
mod test {

    extern crate rand;
    use memory::{MemoryMappedIO, BasicMemory};
    use super::{StandardController, StandardControllerState, Source};
    use std::cell;

    #[test]
    fn standard_controller_should_return_a_button_when_strobe_is_active() {
        let mut memory = BasicMemory::new();

        for _ in 0..100 {
            let source_sequence: Vec<u8> = (0..50).map(|_| rand::random::<u8>()).collect();
            let source = &IteratorSource::from_vec(source_sequence.clone());
            let mut controller = StandardController::new(source);
            controller.write(&mut memory, 1);

            for v in source_sequence.iter() {
                assert_eq!(v & 0x01, controller.read(&memory));
            }
        }
    }

    #[test]
    fn standard_controller_should_reload_state_when_strobe_goes_inactive() {
        let mut memory = BasicMemory::new();
        for i in 0..100 {
            let source_sequence: Vec<u8> = (0..50).map(|_| rand::random::<u8>()).collect();
            let source = &IteratorSource::from_vec(source_sequence.clone());
            let mut controller = StandardController::new(source);
            controller.write(&mut memory, 1);
            controller.write(&mut memory, 0);

            let expected_button_state = source_sequence[0];
            assert_eq!((expected_button_state & 0x01) >> 0, controller.read(&memory), "\nFailed on iteration {}, expected_state {:08b}\n{:?}", i, expected_button_state, controller.state);
            assert_eq!((expected_button_state & 0x02) >> 1, controller.read(&memory), "\nFailed on iteration {}, expected_state {:08b}\n{:?}", i, expected_button_state, controller.state);
            assert_eq!((expected_button_state & 0x04) >> 2, controller.read(&memory), "\nFailed on iteration {}, expected_state {:08b}\n{:?}", i, expected_button_state, controller.state);
            assert_eq!((expected_button_state & 0x08) >> 3, controller.read(&memory), "\nFailed on iteration {}, expected_state {:08b}\n{:?}", i, expected_button_state, controller.state);
            assert_eq!((expected_button_state & 0x10) >> 4, controller.read(&memory), "\nFailed on iteration {}, expected_state {:08b}\n{:?}", i, expected_button_state, controller.state);
            assert_eq!((expected_button_state & 0x20) >> 5, controller.read(&memory), "\nFailed on iteration {}, expected_state {:08b}\n{:?}", i, expected_button_state, controller.state);
            assert_eq!((expected_button_state & 0x40) >> 6, controller.read(&memory), "\nFailed on iteration {}, expected_state {:08b}\n{:?}", i, expected_button_state, controller.state);
            assert_eq!((expected_button_state & 0x80) >> 7, controller.read(&memory), "\nFailed on iteration {}, expected_state {:08b}\n{:?}", i, expected_button_state, controller.state);
        }
    }

    #[test]
    fn reload_standard_controller() {
        let mut memory = BasicMemory::new();
        let source_sequence: Vec<u8> = (0..50).map(|_| rand::random::<u8>()).collect();
        let source = &IteratorSource::from_vec(source_sequence.clone());
        let mut controller = StandardController::new(source);
        controller.write(&mut memory, 1);

        for v in source_sequence.iter().take(10) {
            assert_eq!(v & 0x01, controller.read(&memory));
        }

        controller.write(&mut memory, 0);
        assert_reloaded_state(&controller, source_sequence[10]);

        controller.write(&mut memory, 1);
        controller.write(&mut memory, 0);
        assert_reloaded_state(&controller, source_sequence[11]);
    }

    fn assert_reloaded_state(controller: &StandardController, expected_button_state: u8) {
        let memory = BasicMemory::new();
        assert_eq!((expected_button_state & 0x01) >> 0, controller.read(&memory), "\nFailed: expected_state {:08b}\n{:?}", expected_button_state, controller.state);
        assert_eq!((expected_button_state & 0x02) >> 1, controller.read(&memory), "\nFailed: expected_state {:08b}\n{:?}", expected_button_state, controller.state);
        assert_eq!((expected_button_state & 0x04) >> 2, controller.read(&memory), "\nFailed: expected_state {:08b}\n{:?}", expected_button_state, controller.state);
        assert_eq!((expected_button_state & 0x08) >> 3, controller.read(&memory), "\nFailed: expected_state {:08b}\n{:?}", expected_button_state, controller.state);
        assert_eq!((expected_button_state & 0x10) >> 4, controller.read(&memory), "\nFailed: expected_state {:08b}\n{:?}", expected_button_state, controller.state);
        assert_eq!((expected_button_state & 0x20) >> 5, controller.read(&memory), "\nFailed: expected_state {:08b}\n{:?}", expected_button_state, controller.state);
        assert_eq!((expected_button_state & 0x40) >> 6, controller.read(&memory), "\nFailed: expected_state {:08b}\n{:?}", expected_button_state, controller.state);
        assert_eq!((expected_button_state & 0x80) >> 7, controller.read(&memory), "\nFailed: expected_state {:08b}\n{:?}", expected_button_state, controller.state);

        for _ in 0..10 {
            assert_eq!(1, controller.read(&memory), "All reads after the 8 first should return 1");
        }
    }

    impl Source for u8 {
        fn load(&self) -> StandardControllerState {
            StandardControllerState {
                a_button: (self & 0x01) > 0,
                b_button: (self & 0x02) > 0,
                select: (self & 0x04) > 0,
                start: (self & 0x08) > 0,
                up: (self & 0x10) > 0,
                down: (self & 0x20) > 0,
                left: (self & 0x40) > 0,
                right: (self & 0x80) > 0,
            }
        }
    }

    struct IteratorSource {
        source: Vec<u8>,
        item: cell::Cell<usize>,
    }
    impl IteratorSource {
        fn from_vec(source: Vec<u8>) -> IteratorSource {
            IteratorSource {
                source: source,
                item: cell::Cell::new(0),
            }
        }
    }
    impl Source for IteratorSource {
        fn load(&self) -> StandardControllerState {
            let value = self.source[self.item.get()];
            self.item.set(self.item.get() + 1);
            return value.load();
        }
    }
}
