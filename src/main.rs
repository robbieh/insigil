extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::rect::Rect;

use std::time::Duration;
use std::thread;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{stdin};


// parse args
//  data type
//  viz type
// start a thread
// read stdin into appropriate buffer type

// some sort of data structure describing state/config of viz
// what data types? 
// series of ints
//   interpreted as ...
//     histogram - 'ticks' of different heights ||||
//     interval - space between ticks   | |   |  |    |  | |
// dates
// text
//
//
enum RingVizType {
    Hist, Interval, Text
}

// need... a data structure to fill from the thread

enum RingDataBuffer {
    Ints(VecDeque<i32>),
    Text(VecDeque<char>),
    Dates(VecDeque<i32>)
}


macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn hist_ring(canvas: &mut Canvas<Window>, r: Rect, buf: &mut RingDataBuffer) {
    //^^^pass canvas? or some sort of trait, like canvas.circle?

}

fn ring(canvas: &mut Canvas<Window>, viztype: &mut RingVizType) {
    let (width, height) = canvas.window().size();
    let half_height: i32 = height as i32 / 2;
    let half_width: i32 = width as i32 / 2;
    //print!("width: {:?}, height: {:?}", width, height);

    //let r = rect!(half_width, half_height, width, height);

    //print!("{:?},{:?} {:?},{:?}", width, height, half_height, half_width);

    let color = Color::RGB(0,255,0);
    //let _ = canvas.set_viewport(r);
    let _ = canvas.circle(half_width as i16,half_height as i16,50,color);
    canvas.present();

}

fn io_reader(textq: VecDeque<char>) {
    for line in std::io::stdin().lines() {
            println!(line.ok().unwrap());
            //line.ok().unwrap().each()
    }
}

pub fn main() {
    let textq: VecDeque <char> = VecDeque:::new();
    thread::spawn(|| { io_reader(); });

    let mut viztype = RingVizType::Hist;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { 
                    keycode: Some(Keycode::Escape), ..
                } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        ring(&mut canvas, &mut viztype);
    }
}
