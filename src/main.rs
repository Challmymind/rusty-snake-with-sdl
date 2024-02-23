use sdl2::{self, render::RenderTarget};
use std::time;

fn main(){

    // Rust bindings call implicitly SDL_QUIT after droping context
    let sdl_context = sdl2::init().expect("Cannot initialize SDL library");
    let video_subsystem = sdl_context.video().expect("Cannot start video subsystem");
    let mut event_pump = sdl_context.event_pump().expect("Cannot get even pump");

    let mut canvas = video_subsystem.window("rusty snake", 500, 500)
        .build()
        .expect("Cannot create window")
        .into_canvas()
        .build()
        .expect("Cannot create canvas");


    let timer = time::Instant::now();
    let mut last = timer.elapsed();

    let mut r : u8 = 0;
    let mut g : u8 = 0;
    let mut b : u8 = 0;

    draw(&mut canvas, sdl2::pixels::Color::RGB(r, g, b));

    loop {

        let now = timer.elapsed();

        if (now - last).as_millis() >= 100 {

            last = now;
            r = r.wrapping_add(5);
            g = g.wrapping_add(10);
            b = b.wrapping_add(2);

            draw(&mut canvas, sdl2::pixels::Color::RGB(r, g, b));
            
        }

        match event_pump.poll_event() {
            Some(sdl2::event::Event::Quit{ timestamp: _}) => break,
            _ => continue
        }


    }

}

pub fn draw<T : sdl2::render::RenderTarget>(
    canva : &mut sdl2::render::Canvas<T>, 
    color : sdl2::pixels::Color
    )
{
    canva.set_draw_color(color);
    canva.clear();
    canva.present();
}
