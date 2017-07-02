
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use state;
use std::cmp::{min,max};
use opengl_graphics::{ GlGraphics, OpenGL }; 
use graphics::*;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const GREEN_05: [f32; 4] = [0.0, 1.0, 0.0, 0.5];
pub fn ring<G>(
        ringbounds: [f64; 4],
        transform: math::Matrix2d,
        g: &mut G,
        rdbints: &mut state::RingDataBuffer,
        size: f64
        ) where G: Graphics {
    let width = ((ringbounds[0]).abs() - (ringbounds[2]).abs()).abs();
    let height = ((ringbounds[1]).abs() - (ringbounds[3]).abs()).abs();
    let half_height = height / 2.0;
    let half_width = width / 2.0;
    let radius = f64::min(width, height) / 2.0;
    let buffer = 5.0;

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

    
    //draw stuff
    circle_arc(GREEN_05, 1.0, 0.0, 6.282, ringbounds, transform, g);
    match rdbints {
        &mut state::RingDataBuffer::Ints(ref intvec) => {
            for (idx, i) in intvec.iter().enumerate() {
                let t = transform.rot_rad(0.031415 * idx as f64);
                let line = rectangle::rectangle_by_corners(
                    3.0, (2.0 * radius - buffer),
                    -3.0, (2.0 * radius - buffer) - i.clone() as f64,
                                                           );
                //println!("{:?}", line);
                rectangle(GREEN_05, line, t, g);

            }
        }
        _ => {}
            
    }

}
