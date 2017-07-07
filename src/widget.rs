
use std::cmp::{min,max};
use opengl_graphics::{ GlGraphics, OpenGL }; 
use graphics::*;
use state;

pub trait Widget<G> {
    fn draw(&mut self, math::Matrix2d, &mut G, state::RingDataBuffer);
    //fn set_center(&self, f64, f64);
}
