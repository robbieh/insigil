
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use state;
use std::cmp::{min,max};
use opengl_graphics::{ GlGraphics, OpenGL }; 
use graphics::*;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub fn ring<G>(
        ringbounds: [f64; 4],
        transform: math::Matrix2d,
        g: &mut G,
        rdbints: &mut state::RingDataBuffer
        ) where G: Graphics {
    let (width, height) = (100, 100);
    let half_height: i32 = height as i32 / 2;
    let half_width: i32 = width as i32 / 2;

    //calculate stuff
    let (sum,max,avg) = match rdbints {
        &mut state::RingDataBuffer::Ints(ref mut intvec) => 
            { 
            let sum = intvec.iter().sum();
            let max = intvec.iter().fold(0,|largest, &i| max(i, largest));
            let avg: f32 = sum as f32/ intvec.len() as f32;
            print!("\rs,m,a: {:?} {:?} {:?}", sum, max, avg);
            (sum,max,avg)
            },
        _ => (0,0,0.0)
    };

    let rad: f32 = if half_width < half_height {half_height as f32} else {half_width as f32};
    
    //draw stuff
    match rdbints {
        &mut state::RingDataBuffer::Ints(ref intvec) => {
            for i in intvec.iter() {
                let bounds = rectangle::square(0.0, i.clone() as f64, 10.0);
                ellipse(GREEN, bounds, transform, g);
            }
        }
        _ => {}
            
    }

}
