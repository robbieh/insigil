extern crate time;
extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;
extern crate find_folder;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use piston::input::*;
use piston_window::{PistonWindow};
use opengl_graphics::glyph_cache::GlyphCache;
use piston::window::{WindowSettings};
use opengl_graphics::{ GlGraphics, OpenGL }; 
use graphics::{Transformed};

use std::thread;

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::env;

mod data_acquisition;
mod state;
mod viz;
mod widget;
mod config;

use widget::Widget;

pub struct App {
    p: Params,
    gl: GlGraphics,
    widgets: Vec<Box<Widget>>,
    rxchan: Receiver<state::ChannelData>,
    glyphs: GlyphCache<'static>,
    palette: state::Palette
}

const FONT: &str = "font/Hack-Regular.ttf";
const DEFAULT_WINDOW_SIZE: u32 = 640;
const DEFAULT_RING_PCT: u32 = 30;

impl App {
    fn render(&mut self, args: &RenderArgs) {
        let (x,y) = ((args.width as f64/2.0), (args.height as f64/2.0));
        let widgets = & mut self.widgets;
        let glyphs = &mut self.glyphs;
        let bg = self.palette.background;

        self.gl.draw(args.viewport(), |c, g| {
            piston_window::clear(bg,g);
            //let transform = c.transform.trans(110.0,530.0);
            let transform = c.transform.trans(x,y);
            for widget in widgets.iter_mut() {
                widget.draw(glyphs, &c, transform, g);
            }
        });
    }
    fn update(&mut self, args: &UpdateArgs) {
    }
    fn receive(&mut self) {
        for rdin in self.rxchan.try_iter() {
            for widget in self.widgets.iter_mut() {
                let cloneddat = rdin.dat.clone();
                if widget.getid() == rdin.id {
                    widget.push(cloneddat);
                }
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct FileAndOpts {
    file: String,
    opts: String,
    viz_type: state::RingDataBufferType
}

#[derive(Debug, Clone)]
pub struct Params {
    files: Vec<FileAndOpts>,
    //other settings
}

pub fn parse_args(mut args: std::env::Args) -> Params {
    let mut p = Params {
        files: Vec::<FileAndOpts>::new(),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-hr" => {
                let f = args.next().unwrap();
                let fao = FileAndOpts { file: f, opts: "hr".to_string(),
                                  viz_type: state::RingDataBufferType::Ints};
                p.files.push(fao);
                //println!("file {:?}", f)
            }
            "-gr" => {
                let f = args.next().unwrap();
                let fao = FileAndOpts { file: f, opts: "gr".to_string(),
                                  viz_type: state::RingDataBufferType::IntVec};
                p.files.push(fao);
                //println!("file {:?}", f)
            }
            "-tr" => {
                let f = args.next().unwrap();
                let fao = FileAndOpts { file: f, opts: "tr".to_string(),
                                  viz_type: state::RingDataBufferType::Text};
                p.files.push(fao);
                //println!("file {:?}", f)
            }
            _ => {
                //println!("misc arg {:?}", arg)
            }
        }
    };
    p
}

/// This will spwan threads paired with widgets, and create
/// the App struct with widget vec and receiver channel
pub fn setup(window: & PistonWindow, opengl: piston_window::OpenGL, p: & Params) -> App {
    //println!("params\n=====\n{:?}", p);

    let (txdata,rxdata): (Sender<state::ChannelData>, Receiver<state::ChannelData>) = mpsc::channel();

    //later: perhaps cmd line params to set this
    let wsz = DEFAULT_WINDOW_SIZE;
    let mut wcount = 0;
    // a counter to whittle down with each new widget
    let mut sz = wsz as f64 / 1.0; 
    let rwidth = sz * (DEFAULT_RING_PCT as f64 / 100.0) * 0.25;

    let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();
    let ref font = assets.join(FONT);
    let glyphs = GlyphCache::new(font).unwrap();

    let palette = config::read_palette();

    let mut app = App {
        p: p.clone(),
        gl: GlGraphics::new(opengl),
        widgets: Vec::new(),
        rxchan: rxdata,
        glyphs: glyphs,
        palette: palette.clone()
    };

    for fao in p.files.iter() {
        let thread_tx = txdata.clone();
        let f = fao.file.clone();
        let fo = fao.opts.clone();
        let ft = fao.viz_type.clone();
        if f == "-" {
            thread::spawn(move|| 
                          { data_acquisition::stdin_reader(thread_tx, 
                                                           wcount,
                                                           ft); 
                          });
        } else {
            thread::spawn(move|| 
                          {data_acquisition::file_reader(thread_tx, 
                                                         wcount, 
                                                         f,
                                                         ft); 
                          });
        }
        match fo.as_str() {
            "hr" => {
                let ring = 
                    viz::HistoRing::new
                    (0.0, 0.0, sz, rwidth, wcount, 
                     palette.clone(),
                     state::RingDataBuffer::new(state::RingDataBufferType::Ints), 
                     );
                app.widgets.push(Box::new(ring));
            },
            "gr" => {
                let ring = 
                    viz::GaugesRing::new
                    (0.0, 0.0, sz, rwidth, wcount, 
                     palette.clone(),
                     state::RingDataBuffer::new(state::RingDataBufferType::IntVec), 
                     );
                app.widgets.push(Box::new(ring));
            }
            "tr" => {
                let ring = 
                    viz::TextRing::new
                    (0.0, 0.0, sz, rwidth, wcount, 
                     palette.clone(),
                     state::RingDataBuffer::new(state::RingDataBufferType::Text), 
                     );
                app.widgets.push(Box::new(ring));
            }
            &_ => {}
        }
        sz -= rwidth * 2.0;
        wcount += 1;
    }
    app
}

pub fn main() {
    let p = parse_args(env::args());
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = 
        WindowSettings::new(
            "insigil", 
             [DEFAULT_WINDOW_SIZE, DEFAULT_WINDOW_SIZE])
            .opengl(opengl)
            .samples(8)
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut app = setup(&window, opengl, &p);

    //let mut events = Events::new(EventSettings::new());
    while let Some(e) = window.next() {
        app.receive();
        if let Some(r) = e.render_args() { app.render(&r); }
        if let Some(u) = e.update_args() { app.update(&u); }
    }

}
