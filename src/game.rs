use sdl2::rect::Rect;

const MAP_SIZE : usize = 10;
const FIELD_SIZE : usize = 50;

pub struct Game{
    pub fields : Vec<Vec<Rect>>,
    pub snake : Vec<(u8,u8)>,
}

impl Game {

    pub fn new() -> Self{

        let mut fields = vec![Vec::new(); MAP_SIZE];

        for (i,row) in fields.iter_mut().enumerate() {
            for n in 0..MAP_SIZE{
                row.push(Rect::new((i*FIELD_SIZE) as i32, 
                                   (n*FIELD_SIZE) as i32, 
                                   FIELD_SIZE as u32, 
                                   FIELD_SIZE as u32));
            }
        }

        Game{
            fields,
            snake : vec![(0,0)],
        }
    
    }
}
