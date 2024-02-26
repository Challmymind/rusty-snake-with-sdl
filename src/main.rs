use game::Game;
use sdl2::event::Event;
use std::time;
use game::snake;

mod game;

const SCREEN_AXIS : u32 = 1000;
const FIELDS_DENSITY : u32 = 50;

fn main(){

    // Rust bindings call implicitly SDL_QUIT after droping context
    let sdl_context = sdl2::init().expect("Cannot initialize SDL library");
    let video_subsystem = sdl_context.video().expect("Cannot start video subsystem");
    let mut event_pump = sdl_context.event_pump().expect("Cannot get even pump");

    let mut canvas = video_subsystem.window("rusty snake", SCREEN_AXIS, SCREEN_AXIS)
        .build()
        .expect("Cannot create window")
        .into_canvas()
        .build()
        .expect("Cannot create canvas");


    let timer = time::Instant::now();
    let mut last = timer.elapsed();


    draw(&mut canvas, sdl2::pixels::Color::RGB(0, 0, 0));
    let mut game = Game::new(SCREEN_AXIS,FIELDS_DENSITY);

    loop {

        let now = timer.elapsed();

        if (now - last).as_millis() >= 100 {

            last = now;

            if game.run() == 0 {
                panic!("You lost!");
            }

            canvas.present();
            game.draw(&mut canvas);
            
        }

        match event_pump.poll_event() {
            Some(Event::Quit{..}) => break,
            Some(Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::A),
            ..}) => game.snake.command(snake::Rotation::Left),
            Some(Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::D),
            ..}) => game.snake.command(snake::Rotation::Right),
            Some(Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::W),
            ..}) => game.snake.command(snake::Rotation::Up),
            Some(Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::S),
            ..}) => game.snake.command(snake::Rotation::Down),
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
