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
use std::io::{stdin, BufRead};
use std::sync::{Arc, Mutex};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::collections::HashMap;
use std::collections::VecDeque;

use std::slice::Split;

use std::cmp::{min,max};

mod data_acquisition;
mod state;



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

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);


fn hist_ring(canvas: &mut Canvas<Window>, r: Rect, buf: &mut state::RingDataBuffer) {
    //^^^pass canvas? or some sort of trait, like canvas.circle?

}

fn draw_barchart_ring(canvas: &mut Canvas<Window>, x: f32, y: f32, r: f32, 
                      mut intvec: &VecDeque<i32>, max: i32, avg: f32) {
    let color = Color::RGB(0,255,0);

    canvas.line(x as i16,y as i16,100,100,color);
}


fn ring(mut canvas: &mut Canvas<Window>, 
        //world: &mut state::WorldState,
        //rdbvec: &mut Vec<state::RingDataBuffer>,
        //textq: Arc<Mutex<VecDeque<String>>>
        rdbints: &mut state::RingDataBuffer
        ) {
    let (width, height) = canvas.window().size();
    let half_height: i32 = height as i32 / 2;
    let half_width: i32 = width as i32 / 2;
    //print!("width: {:?}, height: {:?}", width, height);

    let r = rect!(half_width, half_height, width, height);

    //print!("{:?},{:?} {:?},{:?}", width, height, half_height, half_width);

    let color = Color::RGB(0,255,0);
    //let _ = canvas.set_viewport(r);
    let _ = canvas.circle(half_width as i16,half_height as i16,50,color);


    //calculate stuff
    let (sum,max,avg) = match rdbints {
        &mut state::RingDataBuffer::Ints(ref mut intvec) => 
            { 
            let sum = intvec.iter().sum();
            let max = intvec.iter().fold(0,|largest, &i| max(i, largest));
            let avg: f32 = sum as f32/ intvec.len() as f32;
            (sum,max,avg)
            },
        _ => (0,0,0.0)
    };
    println!("s,m,a: {:?} {:?} {:?}", sum, max, avg);

    let rad: f32 = if half_width < half_height {half_height as f32} else {half_width as f32};
    //draw stuff
    match rdbints {
        &mut state::RingDataBuffer::Ints(ref intvec) => 
            draw_barchart_ring(&mut canvas, half_width as f32, half_height as f32, 
                               rad,
                               intvec, max, avg),
                               _ => {}
    }
                               
        



    canvas.present();

}

pub fn handle_io_rx(rxdata: &mut Receiver<state::RingData>, 
                    rdbints: &mut state::RingDataBuffer
                    ) {
    for rdin in rxdata.try_iter() {
        match rdin {
            state::RingData::Int(i) => {
                println!("Got an int {:?}", i.clone());
                match rdbints {
                    &mut state::RingDataBuffer::Ints(ref mut intvec) => intvec.push_back(i),
                    _ => {}
                }
            },
            state::RingData::Text(s) => {},
            state::RingData::Date(i) => {},

        }
    }
}

pub fn main() {
    //let textq: Arc<Mutex<VecDeque<String>>> = Arc::new(Mutex::new(VecDeque::new()));
    //let mut world = state::WorldState {
    //    ioq: VecDeque::<state::Actions>::new(),
    //    data: Vec::<state::RingDataBuffer>::new(),
    //};

    let (txdata,mut rxdata): (Sender<state::RingData>, Receiver<state::RingData>) = mpsc::channel();
    {
        let thread_tx = txdata.clone();
        thread::spawn(move|| { data_acquisition::io_reader(thread_tx); });
    }

    let mut viztype = state::RingVizType::Hist;
    let mut rdbvec = Vec::<state::RingDataBuffer>::new();
    rdbvec.push(state::RingDataBuffer::new(state::RingDataBufferType::Ints));
    let mut rdbints = state::RingDataBuffer::new(state::RingDataBufferType::Ints);

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
        handle_io_rx(&mut rxdata, &mut rdbints);
        
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
        ring(&mut canvas, &mut rdbints);
    }
}
