use sdl2::{rect::Rect,render::Canvas, keyboard::Keycode};

use self::snake::Snake;
mod snake;

const GAP_SIZE : u32 = 1;

pub struct Game{
    fields : Vec<Rect>,
    snake : Snake,
    rgb : (u8,u8,u8),
    map_size : u32,

}

impl Game {

    pub fn new(axis_len : u32, fields_dens : u32) -> Self{

        let dens_usize : usize = fields_dens as usize;
        let mut fields : Vec<Rect> = Vec::with_capacity(dens_usize^2);
        let field_axis = (axis_len - fields_dens + GAP_SIZE)/fields_dens;

        for n in 0..fields_dens{
            for m in 0..fields_dens{
                let n : i32 = n as i32;
                let m : i32 = m as i32;
                let x : i32 = n * (field_axis + GAP_SIZE) as i32;
                let y : i32 = m * (field_axis + GAP_SIZE) as i32;
                fields.push(Rect::new(x, y, field_axis, field_axis));
            }
        }

        Game{
            fields,
            snake : Snake::new(),
            rgb : (0,0,0),
            map_size : fields_dens
        }
    
    }

    pub fn send_command(&mut self, command : Keycode){

        match command {
            Keycode::W => self.snake.command(snake::Rotation::Up),
            Keycode::D => self.snake.command(snake::Rotation::Right),
            Keycode::S => self.snake.command(snake::Rotation::Down),
            Keycode::A => self.snake.command(snake::Rotation::Left),

            _ => return,
        }

    }

    pub fn run(&mut self) -> i32{
        self.snake.run();
        let head = self.snake.segments.first().unwrap().clone();
        let x = head.0;
        let y = head.1;
        if x < 0 || x >= self.map_size as isize{
            return 0;
        }
        if y < 0 || y >= self.map_size as isize{
            return 0;
        }
        1
    }

    pub fn score(&self) -> u32{
        return self.snake.segments.len() as u32;
    }

    pub fn draw<T : sdl2::render::RenderTarget>(&mut self, 
                                                canvas : &mut Canvas<T>)
    {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.fill_rects(&self.fields).unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(
                self.rgb.0, 
                self.rgb.1,
                self.rgb.2));
        for segment in self.snake.segments.iter(){
            canvas.fill_rect(self.fields[
                             segment.1 as usize + (segment.0 as usize * self.map_size as usize) as usize]).unwrap();
        }
        self.rgb.0 = self.rgb.0.wrapping_add(5);
        self.rgb.1 = self.rgb.1.wrapping_add(10);
        self.rgb.2 = self.rgb.2.wrapping_add(2);
    }
}
