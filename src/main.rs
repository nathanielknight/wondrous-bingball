extern crate ggez;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Point2};
use ggez::conf;
use ggez::event;

mod controls;
mod game;
mod util;

struct MainState {
    ball: game::Ball,
    player_paddle: game::Paddle,
    computer_paddle: game::ComputerPaddle,
    control: controls::ControlState,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            ball: game::Ball::new(0.0, 0.0, Point2::new(1.0, 1.0)),
            player_paddle: game::Paddle::new(10.0, 250.0),
            computer_paddle: game::ComputerPaddle::new(
                game::FIELD_WIDTH - 10.0 - game::PADDLE_WIDTH,
                250.0,
            ),
            control: controls::ControlState::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.ball.update(&self.player_paddle, &self.computer_paddle);
        let cmd = self.control.move_state();
        self.player_paddle.update(cmd);
        self.computer_paddle.update(&self.ball);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::BLACK);
        self.ball.draw(ctx)?;
        self.player_paddle.draw(ctx)?;
        self.computer_paddle.draw(ctx)?;
        util::draw_centreline(ctx)?;
        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _: &mut Context,
        kc: ggez::event::Keycode,
        _: ggez::event::Mod,
        _: bool,
    ) {
        self.control.key_down(kc);
    }

    fn key_up_event(
        &mut self,
        _: &mut Context,
        kc: ggez::event::Keycode,
        _: ggez::event::Mod,
        _: bool,
    ) {
        self.control.key_up(kc);
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("wondrous_bingball", "ggez", c).unwrap();
    util::setup_graphics(
        ctx,
        &util::GraphicsOptions {
            width: game::FIELD_WIDTH,
            height: game::FIELD_HEIGHT,
        },
    ).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
