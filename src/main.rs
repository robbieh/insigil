extern crate time;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use piston::window::Window;
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

use time::Timespec;

mod data_acquisition;
mod state;
mod viz;
mod widget;

use widget::Widget;
use graphics::*;

pub struct App {
    gl: GlGraphics,
    widgets: Vec<Box<Widget<GlGraphics>>>
    //irs: state::intRingState
}


const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

impl App {
    fn render(&mut self, args: &RenderArgs) {

        let square = rectangle::centered_square(0.0, 0.0, 10.0);
        let (x,y) = ((args.width as f64/2.0), (args.height as f64/2.0));
        //let ringBounds = rectangle::rectangle_by_corners(-x, -y, x , y );
        //let ringBounds1 = rectangle::rectangle_by_corners(-240.0,-240.0,240.0,240.0);
        //let ringBounds2 = rectangle::rectangle_by_corners(-160.0,-160.0,160.0,160.0);
        //let rdbi = &mut self.rdbints;
        //
        let widgets = & mut self.widgets;
        
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK,gl);
            let transform = c.transform.trans(110.0,530.0);
            for widget in widgets.iter_mut() {
                //widget.draw(transform, gl);
            }
        rectangle(GREEN, square, transform, gl);
            //viz::ring(ringBounds, transform, gl, rdbi, 64.0);
            //viz::ring(ringBounds1, transform, gl, rdbi, 64.0);
            //viz::ring(ringBounds2, transform, gl, rdbi, 64.0);
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


pub fn handle_io_rx_old(rxdata: &mut Receiver<state::RingData>, 
                    rdbints: &mut state::RingDataBuffer
                    ) {
    let maxints = 256;
    for rdin in rxdata.try_iter() {
        match rdin {
            state::RingData::Int(i) => {
                println!("Got an int {:?}", i.clone());
                match rdbints {
                    &mut state::RingDataBuffer::Ints(ref mut intvec) => {
                        intvec.push_back(i);
                        if intvec.len() >= maxints 
                            {let x = intvec.pop_front();}
                    },
                    _ => {}
                }
            },
            state::RingData::Text(s) => {},
            state::RingData::Date(i) => {},

        }
    }
}

pub fn handle_io_rx(rxdata: &mut Receiver<state::RingData>, 
                    rdbints: &mut VecDeque<i32>
                   ) {
    let maxints = 256;
    for rdin in rxdata.try_iter() {
        match rdin {
            state::RingData::Int(i) => {
                println!("Got an int {:?}", i.clone());
                rdbints.push_back(i);
                if rdbints.len() >= maxints 
                {let x = rdbints.pop_front();}
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
    //let mut rdbints = state::RingDataBuffer::new(state::RingDataBufferType::Ints);
    let mut rdbints = VecDeque::<i32>::new();

    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("twirl", [640,640])
        .opengl(opengl)
        .samples(8)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        //irs: intRingState { sliding: false, targetTmMs:  }
        widgets: Vec::new(),
           
    };
    let (x,y) = (window.size().width as f64, window.size().height as f64);
    //let ringBounds = rectangle::rectangle_by_corners(-x, -y, x , y );
    //let ringBounds1 = rectangle::rectangle_by_corners(-240.0,-240.0,240.0,240.0);
    //let ringBounds2 = rectangle::rectangle_by_corners(-160.0,-160.0,160.0,160.0);
    let hr1 = viz::HistoRing::new(0.0, 0.0, 320.0);
    let hr2 = viz::HistoRing::new(0.0, 0.0, 160.0);
    let hr3 = viz::HistoRing::new(0.0, 0.0, 80.0);
    app.widgets.push(Box::new(hr1));
    app.widgets.push(Box::new(hr2));
    app.widgets.push(Box::new(hr3));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        //handle_io_rx(&mut rxdata, &mut app.rdbints);
        handle_io_rx(&mut rxdata, &mut rdbints);
        if let Some(r) = e.render_args() { app.render(&r); }
        if let Some(u) = e.update_args() { app.update(&u); }
    }

}
