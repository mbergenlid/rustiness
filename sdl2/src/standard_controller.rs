use nes::input::standard_controller::{Source, StandardControllerState};
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SdlEvents(pub Rc<RefCell<EventPump>>);

impl Source for SdlEvents {
    fn load(&self) -> StandardControllerState {
        self.0.borrow_mut().pump_events();
        let event_pump = self.0.borrow();
        let state = event_pump.keyboard_state();
        StandardControllerState {
            a_button: state.is_scancode_pressed(Scancode::L),
            b_button: state.is_scancode_pressed(Scancode::K),
            start: state.is_scancode_pressed(Scancode::Space),
            select: state.is_scancode_pressed(Scancode::J),
            up: state.is_scancode_pressed(Scancode::W),
            down: state.is_scancode_pressed(Scancode::S),
            left: state.is_scancode_pressed(Scancode::A),
            right: state.is_scancode_pressed(Scancode::D),
        }
    }
}

