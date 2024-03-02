use crate::game::draw::Draw;
use rand::{self, Rng};
use super::map::Map;

enum Collision{
    Snake,
    Border,
    Fruit,
    Nothing
}

#[derive(Copy,Clone)]
pub enum Rotation{
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

pub struct Snake{
    segments : Vec<(i32,i32)>,
    heading : Rotation,
    last_command : Option<Rotation>,
}

impl Draw for Snake{
    fn get_drawable(&self) -> &[(i32,i32)]{
        &self.segments
    }
}

impl Snake{

    pub fn new() -> Self{
        Snake{
            segments : vec![(0,0),(0,1),(0,2)],
            heading : Rotation::Right,
            last_command : Option::None
        }
    }

    pub fn run(&mut self, map : &mut Map) -> bool{

        let mut head = self.segments.first().unwrap().clone();

        if let Some(i) = self.last_command{
            self.heading = i;
            self.last_command = None;
        }

        match self.heading{
            Rotation::Up => head.1 -= 1,
            Rotation::Down => head.1 += 1,
            Rotation::Left => head.0 -= 1,
            Rotation::Right => head.0 += 1,
        };

        match self.check_collision(&head,map){
            Collision::Snake => return false,
            Collision::Fruit => {
                self.generate_fruit(map);
                self.r#move(head, true);
                },
            Collision::Border => return false,
            Collision::Nothing => self.r#move(head, false),
        }

        true

    }

    fn generate_fruit(&self, map : &mut Map){

        let mut rng = rand::thread_rng(); 
        let mut x = rng.gen_range(0..map.size.0);
        let mut y = rng.gen_range(0..map.size.0);

        loop {
            if self.segments.contains(&(x,y)){
                if x >= map.size.0 - 1{
                    if y >= map.size.0 - 1{
                        x = 0;
                        y = 0;
                        continue;
                    }
                    x = 0;
                    y += 1;
                }
                else {
                    x += 1;
                }
            }
            else {
                break;
            }
        }

        map.fruit = (x,y);

    }

    // Raw identifier so i can use move keyword as function name
    fn r#move(&mut self, new_head : (i32,i32), grow : bool){
        if !grow {
            self.segments.pop();
        }

        self.segments.insert(0, new_head);
    }

    fn check_collision(&self, new_head : &(i32,i32), map : &Map) -> Collision{
        
        let x = new_head.0;
        let y = new_head.1;

        if x < 0 || x >= map.size.0 {
            return Collision::Border;
        }
        if y < 0 || y >= map.size.1 {
            return Collision::Border;
        }

        if self.segments.contains(&new_head){
            return Collision::Snake;
        }

        if *new_head == map.fruit{
            return Collision::Fruit;
        }

        Collision::Nothing

    }

    pub fn command(&mut self, rotation : Rotation){
        if (self.heading as u32 + 2) % 4 == rotation as u32 {
            return;
        }
        self.last_command = Some(rotation);
    }

}
