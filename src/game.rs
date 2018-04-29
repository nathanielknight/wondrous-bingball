extern crate ggez;

use ggez::graphics::{Point2, Rect};

use controls::MoveState;

pub const FIELD_WIDTH: f32 = 700.0;
pub const FIELD_HEIGHT: f32 = 500.0;

fn clamp(x: f32, low: f32, high: f32) -> f32 {
    if x < low {
        low
    } else if x > high {
        high
    } else {
        x
    }
}

pub struct Ball {
    rect: Rect,
    velocity: Point2,
}

impl Ball {
    pub fn update(&mut self) -> ggez::GameResult<()> {
        self.rect.x += self.velocity.x;
        self.rect.y += self.velocity.y;
        Ok(())
    }

    pub fn new(x: f32, y: f32, v: Point2) -> Ball {
        const SIZE: f32 = 10.0;
        let r = Rect {
            x: x,
            y: y,
            w: SIZE,
            h: SIZE,
        };
        Ball {
            rect: r,
            velocity: v,
        }
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        ggez::graphics::rectangle(ctx, ggez::graphics::DrawMode::Fill, self.rect)?;
        Ok(())
    }
}


pub struct Paddle {
    rect: Rect,
}

const PADDLE_SPEED: f32 = 3.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 80.0;
const PADDLE_BUFFER: f32 = 3.0;


impl Paddle {

    pub fn new(x: f32, y: f32) -> Paddle {
        let r: Rect = Rect {x: x, y: y, w: PADDLE_WIDTH, h: PADDLE_HEIGHT};
        Paddle { rect: r }
    }

    fn move_up(&mut self) {
        self.rect.y -= PADDLE_SPEED;
        self.rect.y = clamp(self.rect.y, PADDLE_BUFFER, FIELD_HEIGHT as f32 - PADDLE_BUFFER - self.rect.h);
    }

    fn move_down(&mut self) {
        self.rect.y += PADDLE_SPEED;
        self.rect.y = clamp(self.rect.y, PADDLE_BUFFER, FIELD_HEIGHT as f32 - PADDLE_BUFFER - self.rect.h);
    }

    pub fn update(&mut self, cmd: MoveState) {
        match cmd {
            MoveState::MoveDown => self.move_down(),
            MoveState::MoveUp => self.move_up(),
            MoveState::None => (),
        };
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        ggez::graphics::rectangle(ctx, ggez::graphics::DrawMode::Fill, self.rect)?;
        Ok(())
    }
}
