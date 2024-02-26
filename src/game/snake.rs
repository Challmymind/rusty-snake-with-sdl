pub struct Snake{
    pub segments : Vec<(isize,isize)>,
    heading : Rotation,
}

impl Snake{

    pub fn new() -> Self{
        Snake{
            segments : vec![(0,0),(0,1),(0,2)],
            heading : Rotation::Right,
        }
    }

    pub fn run(&mut self){

        let mut head = self.segments.first().unwrap().clone();

        match self.heading{
            Rotation::Up => head.1 -= 1,
            Rotation::Down => head.1 += 1,
            Rotation::Left => head.0 -= 1,
            Rotation::Right => head.0 += 1,
        };

        self.segments.pop();
        self.segments.insert(0, head);

    }

    pub fn command(&mut self, rotation : Rotation){
        if (self.heading as u32 + 2) % 4 == rotation as u32 {
            return;
        }
        self.heading = rotation;

    }

}

#[derive(Copy,Clone)]
pub enum Rotation{
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}
