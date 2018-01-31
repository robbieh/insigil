#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

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
extern crate hdrsample;

use piston::input::*;
use piston_window::{PistonWindow, TextureSettings};
use opengl_graphics::GlyphCache;
use piston::window::{WindowSettings};
use opengl_graphics::{ GlGraphics, OpenGL }; 
use graphics::{Transformed};

use std::thread;
use std::cmp::{min,max};

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::env;
use std::path::{Path};
use std::process;

mod data_acquisition;
mod state;
mod viz;
mod widget;
mod config;

use state::{RingDataType};
use widget::Widget;

pub struct App {
    p: Params,
    gl: GlGraphics,
    widgets: Vec<Box<Widget>>,
    rxchan: Receiver<state::ChannelData>,
    glyphs: GlyphCache<'static>,
    palette: state::Palette
}

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
    fn resize(&mut self, wevents: &[u32; 2]) {
        let widgets = & mut self.widgets;
        let wsz = min(wevents[0], wevents[1]);
        let mut sz = wsz as f64 * 0.95;  //don't go all the way to the edge
        let rwidth = sz * (DEFAULT_RING_PCT as f64 / 100.0) * 0.25;
        for mut widget in widgets.iter_mut() {
            widget.setsize(sz);
            sz -= rwidth * 2.0;
        }
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
    datType: RingDataType
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
            "-br" => {
                let f = args.next().unwrap();
                let fao = FileAndOpts { file: f, 
                                        opts: "br".to_string(), 
                                        datType: RingDataType::Int};
                p.files.push(fao);
                //println!("file {:?}", f)
            }
            "-hr" => {
                let f = args.next().unwrap();
                let fao = FileAndOpts { file: f, 
                                        opts: "hr".to_string(), 
                                        datType: RingDataType::Int};
                p.files.push(fao);
                //println!("file {:?}", f)
            }
            "-gr" => {
                let f = args.next().unwrap();
                let fao = FileAndOpts { file: f, 
                                        opts: "gr".to_string(), 
                                        datType: RingDataType::IntVec};
                p.files.push(fao);
                //println!("file {:?}", f)
            }
            "-tr" => {
                let f = args.next().unwrap();
                let fao = FileAndOpts { file: f, 
                                        opts: "tr".to_string(), 
                                         datType: RingDataType::Text};
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
    let mut sz = wsz as f64 / 3.0; 
    let rwidth = sz * (DEFAULT_RING_PCT as f64 / 100.0) * 0.25;

    let ref font = match find_folder::Search::ParentsThenKids(3,3).for_folder("assets") {
        Ok(folder) => folder.join("font/Hack-Regular.ttf"),
        Err(_) => Path::new("/usr/share/fonts/truetype/freefont/FreeSans.ttf").to_path_buf()
    };

    let glyphs = match GlyphCache::new(font, (), TextureSettings::new() ) {
        Ok(g) => g,
        Err(e) => {
            println!("Could not load font {:?}", font);
            println!("{:?}", e);
            process::exit(1);
        }
    };
            

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
        let ft = fao.datType.clone();
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
            "br" => {
                let ring = 
                    viz::BarRing::new
                    (0.0, 0.0, sz, rwidth, wcount, 
                     palette.clone(),
                     );
                app.widgets.push(Box::new(ring));
            },
            "hr" => {
                let ring = 
                    viz::HistoRing::new
                    (0.0, 0.0, sz, rwidth, wcount, 
                     palette.clone(),
                     );
                app.widgets.push(Box::new(ring));
            },
            "gr" => {
                let ring = 
                    viz::GaugesRing::new
                    (0.0, 0.0, sz, rwidth, wcount, 
                     palette.clone(),
                     );
                app.widgets.push(Box::new(ring));
            }
            "tr" => {
                let ring = 
                    viz::TextRing::new
                    (0.0, 0.0, sz, rwidth, wcount, 
                     palette.clone(),
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
        if let Some(w) = e.resize_args() { app.resize(&w); }
    }

}
