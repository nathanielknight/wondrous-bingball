//! The simplest possible example that does something.

extern crate ggez;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2};
use ggez::conf;
use ggez::event;

mod controls;
mod game;

struct MainState {
    ball: game::Ball,
    player_paddle: game::Paddle,
    control: controls::ControlState,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            ball: game::Ball::new(0.0, 0.0, Point2::new(1.0, 1.0)),
            player_paddle: game::Paddle::new(10.0, 250.0),
            control: controls::ControlState::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.ball.update()?;
        let cmd = self.control.move_state();
        self.player_paddle.update(cmd);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.ball.draw(ctx)?;
        self.player_paddle.draw(ctx)?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _: &mut Context, kc: ggez::event::Keycode, _: ggez::event::Mod, _: bool) {
        self.control.key_down(kc);
    }

    fn key_up_event(&mut self, _: &mut Context, kc: ggez::event::Keycode, _: ggez::event::Mod, _: bool) {
        self.control.key_up(kc);
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
