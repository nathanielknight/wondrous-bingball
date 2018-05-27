extern crate ggez;

use ggez::audio::{SoundData, Source};

const BOUNCE_BYTES: &'static [u8] = include_bytes!("./bounce.wav");
const OVER_BYTES: &'static [u8] = include_bytes!("./over.wav");

pub struct Sounds {
    pub bounce: Source,
    pub over: Source,
}

pub fn make_sounddata(ctx: &mut ggez::Context) -> ggez::GameResult<Sounds> {
    let bounce_data = SoundData::from_bytes(BOUNCE_BYTES);
    let bounce_sound = Source::from_data(ctx, bounce_data)?;
    let over_data = SoundData::from_bytes(OVER_BYTES);
    let over_sound = Source::from_data(ctx, over_data)?;
    Ok(Sounds{
        bounce: bounce_sound,
        over: over_sound,
    })
}

pub struct AudioSystem {
    sounds: Sounds,
}

pub enum AudioCmd {
    None,
    PlayBounce,
    PlayOver,
}

impl AudioSystem {

    pub fn create(ctx: &mut ggez::Context) -> ggez::GameResult<Self> {
        let sounds = make_sounddata(ctx)?;
        Ok(AudioSystem {
            sounds: sounds,
        })
    }

    pub fn handle_cmd(&self, cmd: AudioCmd) -> ggez::GameResult<()> {
        match cmd {
            AudioCmd::None => Ok(()),
            AudioCmd::PlayBounce => self.sounds.bounce.play(),
            AudioCmd::PlayOver => self.sounds.over.play(),
        }
    }
}
