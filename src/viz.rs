
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
//extern crate unicode_segmentation;
use std::collections::VecDeque;

use state;
use state::{RingDataBuffer, RingData};
use std::cmp::{min,max};
use opengl_graphics::{ GlGraphics }; 
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{self, Context, Transformed };
use graphics::character::CharacterCache;
//use graphics::{Context, Graphics, Transformed, math};
use graphics::*;

use time;
use time::Tm;
use widget::Widget;
use hdrsample::Histogram;

//use unicode_segmentation::UnicodeSegmentation;

const MAX_ENTRIES: usize = 200;

pub struct HistoRing {
    sliding: bool,
    targetTmMs: Tm,
    size: f64,
    innerrad: f64,
    x: f64,
    y: f64,
    id: i32,
    h: Histogram<u64>,
    palette: state::Palette
}

impl HistoRing {
    pub fn new(x: f64, y: f64, 
               size: f64, innerrad: f64, 
               id: i32,  palette: state::Palette,
               ) -> HistoRing {
        //println!("new historing size {:?} using data id {:?}", size.clone(), id.clone());
        HistoRing { 
            sliding: false,
            targetTmMs: time::now(),
            size: size,
            innerrad: innerrad,
            x: x, y: y,
            id: id, 
            h: Histogram::<u64>::new(2).unwrap(),
            palette: palette
        }
    }
}

impl Widget for HistoRing
{
    fn draw(
        &mut self,
        //ringbounds: [f64; 4],
        glyphs: &mut GlyphCache,
        c: &Context,
        transform: math::Matrix2d,
        g: &mut GlGraphics,
        //size: f64
        ) {
        let radius = self.size * 0.5;
        let buffer = 2.0;
        let ref mut dat = self.dat;

        //calculate stuff

        let ringbounds=rectangle::centered_square
            (self.x,
             self.y,
             self.size / 2.0);
        //draw stuff
        //rectangle(GREEN,[0.0,-10.0,10.0,10.0], transform, g);
        circle_arc(self.palette.secondary, 0.5, 0.0, 6.282, ringbounds, transform, g);
        match *dat {
            RingDataBuffer::Ints(ref intsq) => {
                let (sum,mx,avg) = {
                    let sum: i32 = intsq.iter().sum();
                    let mx = intsq.iter().fold(0,|largest, &i| max(i, largest));
                    let avg: f32 = sum as f32 / intsq.len() as f32;
                    //print!("\rs,m,a: {:?} {:?} {:?}", sum, max, avg);
                    (sum,mx,avg)
                };
                let working = (radius - buffer - (radius - self.innerrad + buffer )) as f64;
                let scale = working / mx as f64;
                for (idx, i) in intsq.iter().enumerate() {
                    //println!("draw {:?} {:?}", idx, i.clone());
                    let t = transform.rot_rad(0.031415 * idx as f64);
                    let line = rectangle::rectangle_by_corners(
                        3.0, (1.0 * radius - buffer),
                        -3.0, 
                        ((1.0 * radius - buffer) - (i.clone() as f64 * scale - buffer)).min(1.0 * radius - buffer)
                        );
                    //println!("{:?}", line);
                    rectangle(self.palette.primary, line, t, g);
                }
            },
            RingDataBuffer::Text(ref text) => {
                println!("hrtext{:?}", text);
            },
            RingDataBuffer::DatedInts(ref dis) => {
                println!("hrdates{:?}", dis);
            },
            RingDataBuffer::IntVec(ref iv) => {
                println!("hrintvec{:?}", iv);
            }

        }


    }
    fn getid(&mut self) -> i32 { self.id }
    fn push (
        &mut self,
        rdata: state::RingData
        ) {
        match rdata {
            RingData::Int(i) => { 
                match self.dat {
                    RingDataBuffer::Ints(ref mut intq) => 
                    { intq.push_front(i) ;
                        //println!("pushed: {:?}", i)
                      if intq.len() > MAX_ENTRIES
                          { let _ = intq.pop_back();}
                    },
                    _ => {}
                }
            },
            RingData::Text(s) => {},
            RingData::Date(d) => {},
            RingData::IntVec(iv) => {println!("pushed hrintvec")},
        }
    }
}

pub struct GaugesRing {
    sliding: bool,
    targetTmMs: Tm,
    size: f64,
    innerrad: f64,
    x: f64,
    y: f64,
    id: i32,
    intvec: VecDeque<Vec<i32>>,
    palette: state::Palette
}

impl GaugesRing {
    pub fn new(x: f64, y: f64, 
               size: f64, innerrad: f64, 
               id: i32, palette: state::Palette, 
               ) -> GaugesRing {
        //println!("new gaugesring size {:?} using data id {:?}", size.clone(), id.clone());
        GaugesRing { 
            sliding: false,
            targetTmMs: time::now(),
            size: size,
            innerrad: innerrad,
            x: x, y: y,
            id: id,
            intvec: VecDeque::new(),
            palette: palette
        }
    }
}

impl Widget for GaugesRing
{
    fn draw(
        &mut self,
        //ringbounds: [f64; 4],
        glyphs: &mut GlyphCache,
        c: &Context,
        transform: math::Matrix2d,
        g: &mut GlGraphics,
        //size: f64
        ) {
        let radius = self.size * 0.5;
        let buffer = 2.0;

        //calculate stuff
        let ringbounds=rectangle::centered_square
            (self.x,
             self.y,
             self.size / 2.0);

        //draw stuff
        //rectangle(GREEN,[0.0,-10.0,10.0,10.0], transform, g);
        circle_arc(self.palette.secondary, 0.5, 0.0, 6.282, ringbounds, transform, g);

        let iv = self.intvec;
        //println!("griv {:?}", iv);
        let working = (radius - buffer - 
                       (radius - self.innerrad + buffer )) as f64;
        let ringbounds=rectangle::centered_square
            (self.x,
             self.y,
             self.size / 2.0 - self.innerrad / 2.0);
        //let scale = working / mx as f64;
        if let Some(v) = iv.front() {
            let count = v.len() as f64;
            let arcsize_max_half = 6.282 / count / 2.0;
            //println!("arc {:?}", arcsize_max_half) ;
            for (idx,i) in v.iter().enumerate() {
                //println!("iterating {:?} {:?}", idx, i) ;
                let arc_rot = arcsize_max_half * 2.0 * idx as f64;
                let t = transform.rot_rad(arc_rot);
                let sz = arcsize_max_half * 0.01 * *i as f64; 
                circle_arc(self.palette.primary, working * 0.5 - buffer, 
                           - sz, sz,
                           ringbounds, t, g);
                }
        }
    }
    fn getid(&mut self) -> i32 { self.id }
    fn push (
        &mut self,
        rdata: state::RingData
        ) {
        match rdata {
            RingData::Int(i) => {}, 
            RingData::Text(s) => {},
            RingData::Date(d) => {},
            RingData::IntVec(iv) => {
                    let intvecq = self.intvec;
                    { intvecq.push_front(iv.clone()) ;
                        //println!("pushed: {:?}", iv);
                      if intvecq.len() > 3
                          { let _ = intvecq.pop_back();}
                    },
                }
            },
    }
}

pub struct TextRing {
    sliding: bool,
    targetTmMs: Tm,
    size: f64,
    innerrad: f64,
    x: f64,
    y: f64,
    id: i32,
    dat: VecDeque<char>,
    palette: state::Palette
}

impl TextRing {
    pub fn new(x: f64, y: f64, 
               size: f64, innerrad: f64, 
               id: i32, palette: state::Palette,
               ) -> TextRing {
        println!("new gaugesring size {:?} using data id {:?}",
                 size.clone(), id.clone());
        TextRing { 
            sliding: false,
            targetTmMs: time::now(),
            size: size,
            innerrad: innerrad,
            x: x, y: y,
            id: id, dat: VecDeque::<char>::new(),
            palette: palette
        }
    }
}

impl Widget for TextRing
{
    fn draw(
        &mut self,
        //ringbounds: [f64; 4],
        glyphs: &mut GlyphCache,
        c: &Context,
        transform: math::Matrix2d,
        g: &mut GlGraphics,
        //size: f64
        ) {
        let radius = self.size * 0.5;
        let buffer = 2.0;
        let ref mut dat = self.dat;

        let fontsize = (0.1 * (radius - buffer * 2.0)) as u32; //wild wild guess on 0.1* to scale it down...need to look at device dpi or something? hrm.

        //calculate stuff
        let ringbounds=rectangle::centered_square
            (self.x,
             self.y,
             self.size / 2.0);

        //draw stuff
        //rectangle(GREEN,[0.0,-10.0,10.0,10.0], transform, g);
        circle_arc(self.palette.secondary, 0.5, 0.0, 6.282, ringbounds, transform, g);
        let mut cursor = 0.0;
        let text = self.dat;
        //for (idx,c) in UnicodeSegmentation::graphemes(text,true)
        //    .iter().enumerate() 
        for (idx,c) in text.iter().enumerate() 
            {

        //note ... arc length = theta * radius (when theta is in radians)
        //thus arc length / radius = theta
        let arc_length = glyphs.character(fontsize, *c).width();
        let theta = arc_length / radius;
        //println!("{:?} {:?} {:?} {:?}", c, arc_length, theta, cursor);

        //let t = transform.rot_rad(0.0314 * idx as f64).trans(0.0,radius - buffer);
        let t = transform.rot_rad(cursor).trans(0.0,radius - buffer);
        cursor = cursor + theta;

        piston_window::text(self.palette.primary, fontsize, 
                            &c.to_string(), glyphs, t, g);
        }
    }
    fn getid(&mut self) -> i32 { self.id }
    fn push (
        &mut self,
        rdata: state::RingData
        ) {
        match rdata {
            RingData::Text(s) => {
                let txtq = self.dat;
                //println!("{:?}",txtq);
                for c in s.chars() {
                    txtq.push_front(c);
                };
                //println!("{:?}",txtq.len());
                while txtq.len() > 210 { let _ = txtq.pop_back(); }
            },
            _ => {},
        }
    }
}
