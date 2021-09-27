use std::f64::consts::PI;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Circle{
    pub radius: f64
}
#[derive(Debug)]
pub struct Square{
    pub side: f64
}

pub trait Shape{
    fn area(&self) -> f64;
}

impl Shape for Square{
    fn area(&self) -> f64 {
        return self.side*self.side
    }
}

impl Shape for Circle{
    fn area(&self) -> f64 {
        return self.radius*self.radius*PI
    }
}

 pub fn print_info<T: Shape + Debug>(shape: T){
     println!("{:?}",shape);
    println!("{}",shape.area());
}