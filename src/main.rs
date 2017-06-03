extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};

use sdl2::gfx::primitives::DrawRenderer;

use std::time::Duration;
use std::thread;

fn ring(canvas: &mut Canvas<Window>) {
    let (width, height) = canvas.window().size();
    //print!("width: {:?}, height: {:?}", width, height);

    let color = Color::RGB(0,255,0);
    let _ = canvas.circle(0,0,10,color);
    let _ = canvas.circle(0,0,100,color);
    let _ = canvas.circle(0,0,1000,color);
    canvas.present();

}

pub fn main() {
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
        ring(&mut canvas)
    }
}
