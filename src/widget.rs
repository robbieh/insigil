
use std::cmp::{min,max};
use opengl_graphics::{ GlGraphics, OpenGL }; 
use opengl_graphics::glyph_cache::GlyphCache;
use graphics::{Context, Graphics, math};
use piston_window::{G2dTexture};
use state;

pub trait Widget {
    fn draw(&mut self,  &mut GlyphCache,  &Context, 
            math::Matrix2d, &mut GlGraphics);
    //fn set_center(&self, f64, f64);
    fn getid(&mut self) -> i32;
    fn push(&mut self, state::RingData);
}
