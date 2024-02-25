pub struct Snake{
    pub segments : Vec<(isize,isize)>,
    heading : Rotation,
}

impl Snake{

    pub fn new() -> Self{
        Snake{
            segments : vec![(0,0)],
            heading : Rotation::Right,
        }
    }

    pub fn run(&mut self){

        let mut head = self.segments.first().unwrap().clone();

        match self.heading{
            Rotation::Up => head.1 += 1,
            Rotation::Down => head.1 -= 1,
            Rotation::Left => head.0 -= 1,
            Rotation::Right => head.0 += 1,
        };

        self.segments.pop();
        self.segments.insert(0, head);

    }

    pub fn turn_right(&mut self){
        self.heading = match self.heading {
            Rotation::Up => Rotation::Right,
            Rotation::Right => Rotation::Down,
            Rotation::Down => Rotation::Left,
            Rotation::Left => Rotation::Up
        }
    }

    pub fn turn_left(&mut self){
        self.heading = match self.heading {
            Rotation::Up => Rotation::Left,
            Rotation::Right => Rotation::Up,
            Rotation::Down => Rotation::Right,
            Rotation::Left => Rotation::Down
        }
    }

}

enum Rotation{
    Up,
    Down,
    Right,
    Left,
}
