use ggez::event::Keycode;

struct InputState {
    up_pressed: bool,
    down_pressed: bool,
}

pub enum MoveState {
    MoveUp,
    MoveDown,
    None,
}

pub struct ControlState {
    input: InputState,
}

impl ControlState {
    pub fn new() -> ControlState {
        ControlState {
            input: InputState {
                up_pressed: false,
                down_pressed: false,
            },
        }
    }

    pub fn key_down(&mut self, kc: Keycode) {
        match kc {
            Keycode::Up => self.input.up_pressed = true,
            Keycode::Down => self.input.down_pressed = true,
            _ => (),
        };
    }

    pub fn key_up(&mut self, kc: Keycode) {
        match kc {
            Keycode::Up => self.input.up_pressed = false,
            Keycode::Down => self.input.down_pressed = false,
            _ => (),
        };
    }

    pub fn move_state(&self) -> MoveState {
        match (self.input.up_pressed, self.input.down_pressed) {
            (false, false) => MoveState::None,
            (true, false) => MoveState::MoveUp,
            (false, true) => MoveState::MoveDown,
            (true, true) => MoveState::None,
        }
    }
}
