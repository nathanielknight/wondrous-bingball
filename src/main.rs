extern crate ggez;

use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::conf;
use ggez::event;

mod audio;
mod controls;
mod game;
mod util;

struct MainState {
    game: game::Game,
    control: controls::ControlState,
    player_score: u32,
    computer_score: u32,
    audio_system: audio::AudioSystem,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let audio_system = audio::AudioSystem::create(_ctx)?;
        let s = MainState {
            game: game::Game::default(),
            control: controls::ControlState::new(),
            player_score: 0,
            computer_score: 0,
            audio_system: audio_system,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let cmd = self.control.move_state();
        let audio_cmd = self.game.update(cmd);
        self.audio_system.handle_cmd(audio_cmd)?;
        match self.game.status() {
            game::Status::Ongoing => (),
            game::Status::Over(game::Belligerent::Player) => {
                self.player_score += 1;
                self.game.reset();
            }
            game::Status::Over(game::Belligerent::Computer) => {
                self.computer_score += 1;
                self.game.reset();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        util::draw_centreline(ctx)?;
        util::draw_score(ctx, self.player_score, self.computer_score)?;
        self.game.draw(ctx)?;
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
    let ctx = &mut Context::load_from_conf("wondrous_bingball", "ggez", c).expect("error creating context");
    util::setup_graphics(
        ctx,
        &util::GraphicsOptions {
            width: game::FIELD_WIDTH,
            height: game::FIELD_HEIGHT,
        },
    ).expect("error setting up graphics");
    let state = &mut MainState::new(ctx).expect("error creating main state");
    event::run(ctx, state).expect("error running game");
}
