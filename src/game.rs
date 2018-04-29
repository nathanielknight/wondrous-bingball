extern crate ggez;

use ggez::graphics::{Point2, Rect};

use controls::MoveState;

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

const PADDLE_SPEED: f32 = 10.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 80.0;


impl Paddle {

    pub fn new(x: f32, y: f32) -> Paddle {
        let r: Rect = Rect {x: x, y: y, w: PADDLE_WIDTH, h: PADDLE_HEIGHT};
        Paddle {rect: r}
    }

    fn move_up(&mut self) {
        self.rect.y -= PADDLE_SPEED;
    }

    fn move_down(&mut self) {
        self.rect.y += PADDLE_SPEED;
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
