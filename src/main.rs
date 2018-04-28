//! The simplest possible example that does something.

extern crate ggez;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2};
use ggez::conf;
use ggez::event;

mod game;

struct MainState {
    ball: game::Ball,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            ball: game::Ball::new(0.0, 0.0, Point2::new(1.0, 1.0)),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.ball.update()?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.ball.draw(ctx)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
