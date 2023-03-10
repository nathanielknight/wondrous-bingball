extern crate ggez;
extern crate rand;

use std;
use ggez::{Context, GameResult};
use ggez::graphics::{Point2, Rect};
use controls::MoveState;

use audio::AudioCmd;

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

struct Ball {
    rect: Rect,
    velocity: Point2,
}

impl Default for Ball {
    fn default() -> Self {
        const SIZE: f32 = 10.0;
        let pos = Rect {
            x: 1.0,
            y: 1.0,
            w: SIZE,
            h: SIZE,
        };
        let vel = Point2::new(2.0, 2.0);
        Ball {
            rect: pos,
            velocity: vel,
        }
    }
}

impl Ball {
    fn update(&mut self, player: &Paddle, computer: &ComputerPaddle) -> AudioCmd {
        self.rect.x += self.velocity.x;
        self.rect.y += self.velocity.y;

        if self.rect.y < 0.0 || self.rect.y > FIELD_HEIGHT {
            self.velocity.y *= -1.0;
            self.rect.y = clamp(self.rect.y, 0.0, FIELD_HEIGHT);
            return AudioCmd::PlayBounce
        }

        if self.rect.overlaps(&player.rect) {
            self.bounce_off(&player.rect);
            return AudioCmd::PlayBounce
        }
        if self.rect.overlaps(&computer.paddle.rect) {
            self.bounce_off(&computer.paddle.rect);
            return AudioCmd::PlayBounce
        }
        if self.out_of_bounds() {
            AudioCmd::PlayOver
        } else {
            AudioCmd::None
        }
    }

    // Reverse x direction, set y based on
    fn bounce_off(&mut self, r: &Rect) {
        let c_slf = self.rect.y + self.rect.h * 0.5;
        let c_oth = r.y + r.h * 0.5;
        let dy = c_slf - c_oth;
        let theta = std::f32::consts::PI / 4.0 * dy / r.h;
        let speed =
            (self.velocity.x * self.velocity.x + self.velocity.y * self.velocity.y).sqrt() * 1.1;
        // Set velocity direction
        self.velocity.x = -1.0 * self.velocity.x.signum() * theta.cos();
        self.velocity.y = theta.sin();
        // Set velocity magnitude
        self.velocity *= speed;
        // Set position just outside paddle (to avoid multple bounces)
        if self.velocity.x > 0.0 {
            self.rect.x = r.right() + 1.0;
        } else {
            self.rect.x = r.left() - 1.0 - r.w;
        }
    }

    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        ggez::graphics::rectangle(ctx, ggez::graphics::DrawMode::Fill, self.rect)?;
        Ok(())
    }

    fn init_random_velocity(&mut self) {
        const MAX_SPEED: f32 = 2.0;
        let x_rand = 0.5 + 0.5 * rand::random::<f32>();
        let y_rand = 0.25 + 0.75 * rand::random::<f32>();

        let new_velocity = Point2::new(x_rand, y_rand);
        self.velocity = new_velocity * MAX_SPEED;
    }

    fn out_of_bounds(&self) -> bool {
        self.rect.x < 0.0 || self.rect.x > FIELD_WIDTH
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
    fn update(&mut self, b: &Ball) {
        const LIMIT: f32 = FIELD_WIDTH * 0.75;
        let x = self.paddle.rect.x;
        if x - b.rect.x > LIMIT {
            return;
        }
        let pos = self.paddle.rect.y + self.paddle.rect.h * 0.5;
        let ball_pos = b.rect.y + b.rect.h * 0.5;
        let ind = pos - ball_pos; // negative if ball is below self
        const BUFFER: f32 = 16.0;
        if ind < -1.0 * BUFFER {
            self.paddle.move_down();
        }
        if ind > BUFFER {
            self.paddle.move_up();
        }
    }

    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        ggez::graphics::rectangle(ctx, ggez::graphics::DrawMode::Fill, self.paddle.rect)?;
        Ok(())
    }

    fn new(x: f32, y: f32) -> Self {
        let p = Paddle::new(x, y);
        Self { paddle: p }
    }
}

pub struct Game {
    ball: Ball,
    player: Paddle,
    computer: ComputerPaddle,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            ball: Ball::default(),
            player: Paddle::new(10.0, 250.0),
            computer: ComputerPaddle::new(FIELD_WIDTH - 10.0 - PADDLE_WIDTH, 250.0),
        }
    }
}

pub enum Belligerent {
    Player,
    Computer,
}

pub enum Status {
    Ongoing,
    Over(Belligerent),
}

impl Game {
    pub fn update(&mut self, cmd: MoveState) -> AudioCmd {
        let audio_cmd = self.ball.update(&self.player, &self.computer);
        self.player.update(cmd);
        self.computer.update(&self.ball);
        audio_cmd
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ball.draw(ctx)?;
        self.player.draw(ctx)?;
        self.computer.draw(ctx)?;
        Ok(())
    }

    pub fn status(&self) -> Status {
        if self.ball.rect.x < 0.0 {
            return Status::Over(Belligerent::Computer);
        } else if self.ball.rect.x > FIELD_WIDTH {
            return Status::Over(Belligerent::Player);
        } else {
            return Status::Ongoing;
        }
    }

    pub fn reset(&mut self) {
        let start_pos: Point2 = Point2::new(
            FIELD_WIDTH * 0.4,
            FIELD_HEIGHT * 0.4,
        );
        self.ball.rect.move_to(start_pos);
        self.ball.init_random_velocity();
    }
}
