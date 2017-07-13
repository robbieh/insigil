
use std::cmp::{min,max};
use opengl_graphics::{ GlGraphics, OpenGL }; 
use graphics::{Context, Graphics, math};
use piston_window::{Glyphs,G2dTexture};
use state;

pub trait Widget<G> {
    fn draw(&mut self, glyphs: Glyphs, c: &Context, math::Matrix2d, &mut G);
    //fn set_center(&self, f64, f64);
    fn getid(&mut self) -> i32;
    fn push(&mut self, state::RingData);
}
