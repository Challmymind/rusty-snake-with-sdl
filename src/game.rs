use sdl2::{rect::Rect,render::Canvas, keyboard::Keycode, pixels::Color};
use noise::{self, NoiseFn};

use self::{snake::Snake, map::Map, draw::Draw};
mod snake;
pub mod draw;
pub mod map;

const GAP_SIZE : u32 = 1;

pub struct Game{
    fields : Vec<Rect>,
    snake : Snake,
    rgb : (u8,u8,u8),
    map : Map,
    noise : noise::Perlin,
    step : f64,
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

        let fields_dens = fields_dens as i32;

        Game{
            fields,
            snake : Snake::new(),
            rgb : (0,0,0),
            map : Map::new(fields_dens, fields_dens),
            noise : noise::Perlin::new(1),
            step : 0.,
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

    pub fn run(&mut self) -> bool{
        return self.snake.run(&mut self.map);
    }

    //pub fn score(&self) -> u32{
    //    return self.snake.segments.len() as u32;
    //}

    pub fn draw<T : sdl2::render::RenderTarget>(&mut self, 
                                                canvas : &mut Canvas<T>)
    {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for rect in &self.fields{
            let x : f64 = (rect.x() as f64 + self.step)/1000.;
            let y : f64 = (rect.y() as f64 + self.step)/1000.;
            let perlin = (self.noise.get([x,y]) + 1.) * 10.;
            match perlin {
                perlin if perlin < 8. => {
                    canvas.set_draw_color(Color::RGB(0, 153, 0));
                },
                perlin if perlin > 12. => {
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                },
                _ => {
                    canvas.set_draw_color(Color::RGB(0, 200, 0));
                },
            }
            canvas.fill_rect(*rect).unwrap();

        }
        self.step += self.fields.first().unwrap().width() as f64;
        canvas.set_draw_color(sdl2::pixels::Color::RGB(
                self.rgb.0, 
                0,
                self.rgb.2));
        for segment in self.snake.get_drawable().iter(){
            let field = segment.1 + (segment.0 * self.map.size.0);
            if let Some(i) = self.fields.get(field as usize){
                canvas.fill_rect(*i).unwrap();
            }
        }
        let field = self.map.fruit.1 + (self.map.fruit.0 * self.map.size.0);
        if let Some(i) = self.fields.get(field as usize){
            canvas.fill_rect(*i).unwrap();
        }
        self.rgb.0 = self.rgb.0.wrapping_add(10);
        self.rgb.2 = self.rgb.2.wrapping_add(5);
    }
}
