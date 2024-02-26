use sdl2::{rect::Rect,render::Canvas};

use self::snake::Snake;
pub mod snake;

const GAP_SIZE : u32 = 1;

pub struct Game{
    pub fields : Vec<Vec<Rect>>,
    pub snake : Snake,
    rgb : (u8,u8,u8),
    map_size : u32,
}

impl Game {

    pub fn new(axis_len : u32, fields_dens : u32) -> Self{

        let mut fields = vec![Vec::new(); fields_dens as usize];
        let field_axis = (axis_len - fields_dens + GAP_SIZE)/fields_dens;

        for (i,row) in fields.iter_mut().enumerate() {
            for n in 0..fields_dens{
                row.push(Rect::new((i*(field_axis + GAP_SIZE) as usize) as i32, 
                                   (n*(field_axis + GAP_SIZE)) as i32, 
                                   field_axis, 
                                   field_axis));
            }
        }

        Game{
            fields,
            snake : Snake::new(),
            rgb : (0,0,0),
            map_size : fields_dens
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
