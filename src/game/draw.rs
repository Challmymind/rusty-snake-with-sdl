pub trait Draw{
    fn get_drawable(&self) -> &[(i32,i32)];
}
