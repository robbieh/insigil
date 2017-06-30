extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL }; 

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

pub struct App {
    gl: GlGraphics
}


const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x,y) = ((args.width/2) as f64, (args.height/2) as f64);
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK,gl);
            let transform = c.transform.trans(x,y);
            rectangle(GREEN, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        //
    }

}

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


fn ring(//mut canvas: &mut Canvas<Window>, 
        rdbints: &mut state::RingDataBuffer
        ) {
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
            (sum,max,avg)
            },
        _ => (0,0,0.0)
    };
    println!("s,m,a: {:?} {:?} {:?}", sum, max, avg);

    let rad: f32 = if half_width < half_height {half_height as f32} else {half_width as f32};
    //draw stuff
    //match rdbints {
    //    &mut state::RingDataBuffer::Ints(ref intvec) => 
    //}

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

    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("twirl", [640,640])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() { app.render(&r); }
        if let Some(u) = e.update_args() { app.update(&u); }
    }

}
