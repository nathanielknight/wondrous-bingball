use ggez::{Context, GameResult};
use ggez::graphics;

pub struct GraphicsOptions {
    pub width: f32,
    pub height: f32,
}

pub fn setup_graphics(ctx: &mut Context, opt: &GraphicsOptions) -> GameResult<()> {
    graphics::set_resolution(ctx, opt.width as u32, opt.height as u32)?;
    graphics::set_fullscreen(ctx, true)?;
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, opt.width, opt.height))?;
    Ok(())
}
