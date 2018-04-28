extern crate ggez;

use ggez::graphics::{Point2, Rect};

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

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        ggez::graphics::rectangle(ctx, ggez::graphics::DrawMode::Fill, self.rect)?;
        Ok(())
    }
}
