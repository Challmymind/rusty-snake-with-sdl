use sdl2::{rect::Rect,render::Canvas};

use self::snake::Snake;
mod snake;

pub struct Game{
    pub fields : Vec<Vec<Rect>>,
    snake : Snake,
    rgb : (u8,u8,u8),
    map_size : usize,
}

impl Game {

    pub fn new(map_size : usize, field_size : usize, fields_gap : usize) -> Self{

        let mut fields = vec![Vec::new(); map_size];

        for (i,row) in fields.iter_mut().enumerate() {
            for n in 0..map_size{
                row.push(Rect::new((i*(field_size+fields_gap)) as i32, 
                                   (n*(field_size+fields_gap)) as i32, 
                                   field_size as u32, 
                                   field_size as u32));
            }
        }

        Game{
            fields,
            snake : Snake::new(),
            rgb : (0,0,0),
            map_size
        }
    
    }

    pub fn turn_left(&mut self){
        self.snake.turn_left();
    }

    pub fn turn_right(&mut self){
        self.snake.turn_right();
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

    pub fn draw<T : sdl2::render::RenderTarget>(&mut self, 
                                                canvas : &mut Canvas<T>)
    {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for row in self.fields.iter(){
            for rect in row.iter(){
                canvas.fill_rect(*rect).unwrap();
            }
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(
                self.rgb.0, 
                self.rgb.1,
                self.rgb.2));
        for segment in self.snake.segments.iter(){
            canvas.fill_rect(
                self.fields[segment.0 as usize][segment.1 as usize]).unwrap();

                self.rgb.0 = self.rgb.0.wrapping_add(5);
                self.rgb.1 = self.rgb.1.wrapping_add(10);
                self.rgb.2 = self.rgb.2.wrapping_add(2);
        }
    }
}
