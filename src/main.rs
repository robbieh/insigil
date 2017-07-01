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


mod data_acquisition;
mod state;
mod viz;


pub struct App {
    gl: GlGraphics,
    rdbints: state::RingDataBuffer
}


const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let (x,y) = ((args.width/2) as f64, (args.height/2) as f64);
        let rdbi = &mut self.rdbints;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK,gl);
            let transform = c.transform.trans(x,y);
            rectangle(GREEN, square, transform, gl);
            viz::ring(gl, rdbi);
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
        gl: GlGraphics::new(opengl),
        rdbints: rdbints
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() { app.render(&r); }
        if let Some(u) = e.update_args() { app.update(&u); }
    }

}
