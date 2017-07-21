
use opengl_graphics::{ GlGraphics }; 
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::{Context, math};
use state;

pub trait Widget {
    fn draw(&mut self,  &mut GlyphCache,  &Context, 
            math::Matrix2d, &mut GlGraphics);
    //fn set_center(&self, f64, f64);
    fn getid(&mut self) -> i32;
    fn push(&mut self, state::RingData);
}
