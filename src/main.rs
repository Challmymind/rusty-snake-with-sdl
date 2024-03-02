use game::Game;
use sdl2::event::Event;
use std::time;

mod game;

const SCREEN_AXIS : u32 = 1000;
const FIELDS_DENSITY : u32 = 30;

fn main(){

    // Rust bindings call implicitly SDL_QUIT after droping context
    let sdl_context = sdl2::init().expect("Cannot initialize SDL library");
    let video_subsystem = sdl_context.video().expect("Cannot start video subsystem");
    let mut event_pump = sdl_context.event_pump().expect("Cannot get event pump");

    let mut canvas = video_subsystem.window("rusty snake", SCREEN_AXIS, SCREEN_AXIS)
        .build()
        .expect("Cannot create window")
        .into_canvas()
        .build()
        .expect("Cannot create canvas");


    let timer = time::Instant::now();
    let mut last = timer.elapsed();

    let mut game = Game::new(SCREEN_AXIS,FIELDS_DENSITY);

    loop {

        let now = timer.elapsed();

        if (now - last).as_millis() >= 110{

            last = now;

            if !game.run() {
                //print!("Your score: {}\n", game.score());
                return;
            }

            canvas.present();
            game.draw(&mut canvas);
            
        }

        match event_pump.poll_event() {
            Some(Event::Quit{..}) => break,
            Some(Event::KeyDown { keycode : Some(x),
            ..}) => game.send_command(x),
            _ => continue
        }
    }

}
