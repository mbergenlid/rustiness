use nes::input::standard_controller::{Source, StandardControllerState};
use std::cell::Cell;

pub struct FakeController {
    a_button: Cell<bool>,
    b_button: Cell<bool>,
    start: Cell<bool>,
    select: Cell<bool>,
    up: Cell<bool>,
    down: Cell<bool>,
    left: Cell<bool>,
    right: Cell<bool>,
}

impl Source for FakeController {
    fn load(&self) -> StandardControllerState {
        StandardControllerState {
            a_button: self.a_button.get(),
            b_button: self.b_button.get(),
            start: self.start.get(),
            select: self.select.get(),
            up: self.up.get(),
            down: self.down.get(),
            left: self.left.get(),
            right: self.right.get(),
        }
    }
}

impl FakeController {
    pub fn new() -> FakeController {
        FakeController {
            a_button: Cell::new(false),
            b_button: Cell::new(false),
            start: Cell::new(false),
            select: Cell::new(false),
            up: Cell::new(false),
            down: Cell::new(false),
            left: Cell::new(false),
            right: Cell::new(false),
        }
    }
}
