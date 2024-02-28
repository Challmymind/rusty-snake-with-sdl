pub struct Map{
    pub size : (i32,i32),
    pub fruit : (i32,i32),
    pub colliders : Vec<(i32,i32)>,
}

impl Map{
    pub fn new(x : i32, y : i32) -> Self {
        Map{
            size : (x,y),
            fruit : (10,10),
            colliders : Vec::new(),
        }
    }
}

