use sdl2;

fn main(){

    // Rust bindings call implicitly SDL_QUIT after droping context
    let sdl_context = sdl2::init().expect("Cannot initialize SDL library");
    let video_subsystem = sdl_context.video().expect("Cannot start video subsystem");
    
    let window = video_subsystem.window("rusty snake", 500, 500);
    

}
