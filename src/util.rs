use ggez::{Context, GameResult};
use ggez::graphics;
use game;

pub struct GraphicsOptions {
    pub width: f32,
    pub height: f32,
}

pub fn setup_graphics(ctx: &mut Context, opt: &GraphicsOptions) -> GameResult<()> {
    graphics::set_resolution(ctx, opt.width as u32, opt.height as u32)?;
    graphics::set_fullscreen(ctx, true)?;
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, opt.width, opt.height))?;
    graphics::set_background_color(ctx, graphics::BLACK);
    Ok(())
}
const LINE_WIDTH: f32 = 4.0;

pub fn draw_centreline(ctx: &mut Context) -> GameResult<()> {
    let cl_x = (game::FIELD_WIDTH - LINE_WIDTH) * 0.5;
    let cl_points: Vec<graphics::Point2> = vec![
        graphics::Point2::new(cl_x, 0.0),
        graphics::Point2::new(cl_x, game::FIELD_HEIGHT),
    ];
    graphics::line(ctx, &cl_points, LINE_WIDTH)?;
    Ok(())
}
