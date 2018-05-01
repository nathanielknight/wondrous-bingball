extern crate ggez;

use std;
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
    pub fn update(&mut self, player: &Paddle, computer: &ComputerPaddle) {
        self.rect.x += self.velocity.x;
        self.rect.y += self.velocity.y;

        if self.rect.y < 0.0 || self.rect.y > FIELD_HEIGHT {
            self.velocity.y *= -1.0;
            self.rect.y = clamp(self.rect.y, 0.0, FIELD_HEIGHT);
        }

        if self.rect.overlaps(&player.rect) {
            self.bounce_off(&player.rect);
        }
        if self.rect.overlaps(&computer.paddle.rect) {
            self.bounce_off(&computer.paddle.rect);
        }
    }

    // Reverse x direction, set y based on
    fn bounce_off(&mut self, r: &Rect) {
        println!("Bouncing: v = {:?}", self.velocity);
        let c_slf = self.rect.y + self.rect.h * 0.5;
        let c_oth = r.y + r.h * 0.5;
        let dy = c_slf - c_oth;
        let theta = std::f32::consts::PI / 4.0 * dy / r.h;
        let speed =
            (self.velocity.x * self.velocity.x + self.velocity.y * self.velocity.y).sqrt() * 1.1;
        // set velocity direction
        self.velocity.x = -1.0 * self.velocity.x.signum() * theta.cos();
        self.velocity.y = theta.sin();
        // set velocity magnitude
        self.velocity *= speed;
        println!("bounced: v = {:?}", self.velocity);
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
pub const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 80.0;
const PADDLE_BUFFER: f32 = 3.0;

impl Paddle {
    pub fn new(x: f32, y: f32) -> Paddle {
        let r: Rect = Rect {
            x: x,
            y: y,
            w: PADDLE_WIDTH,
            h: PADDLE_HEIGHT,
        };
        Paddle { rect: r }
    }

    fn move_up(&mut self) {
        self.rect.y -= PADDLE_SPEED;
        self.rect.y = clamp(
            self.rect.y,
            PADDLE_BUFFER,
            FIELD_HEIGHT as f32 - PADDLE_BUFFER - self.rect.h,
        );
    }

    fn move_down(&mut self) {
        self.rect.y += PADDLE_SPEED;
        self.rect.y = clamp(
            self.rect.y,
            PADDLE_BUFFER,
            FIELD_HEIGHT as f32 - PADDLE_BUFFER - self.rect.h,
        );
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

pub struct ComputerPaddle {
    paddle: Paddle,
}

impl ComputerPaddle {
    pub fn update(&mut self, b: &Ball) {
        let x = self.paddle.rect.x;
        const LIMIT: f32 = FIELD_WIDTH * 0.75;
        if x - b.rect.x > LIMIT {
            return;
        }
        let pos = self.paddle.rect.y + self.paddle.rect.h * 0.5;
        let ball_pos = b.rect.y + b.rect.h * 0.5;
        let ind = pos - ball_pos; // negative if ball is below self
        if ind < 5.0 {
            self.paddle.move_down();
        }
        if ind > 5.0 {
            self.paddle.move_up();
        }
    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        ggez::graphics::rectangle(ctx, ggez::graphics::DrawMode::Fill, self.paddle.rect)?;
        Ok(())
    }

    pub fn new(x: f32, y: f32) -> Self {
        let p = Paddle::new(x, y);
        Self { paddle: p }
    }
}
