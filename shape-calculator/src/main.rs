
use std::f64::consts::PI;

//we derrive debug so we can print these structs without having  to implement fmt::display
#[derive(Debug)]
struct Rectangle{
    width: f64,
    height: f64,
}
impl Rectangle{
    fn area(&self) -> f64{
        self.width * self.height
    }
    fn perimeter(&self) -> f64{
        2.0*(self.width + self.height)
    }
}

//we derrive debug so we can print these structs without having  to implement fmt::display
#[derive(Debug)]
struct Circle{
    radius: f64,
}
impl Circle{
    fn area(&self) -> f64{
        self.radius*self.radius*PI
    }
    fn circumference(&self) -> f64{
        2.0*PI*self.radius
    }
}

enum Shape{
    Circle(Circle),
    Rectangle(Rectangle),
}

fn print_relvant_info(shape: &Shape){
    match shape {
        Shape::Circle(c) => {
            println!("{:?}", c);
            println!("Area: {:.4}", c.area());
            println!("Circumference: {:0.4}", c.circumference());
        }
        Shape::Rectangle(r) => {
            println!("{:?}", r);
            println!("Area: {:0.4}", r.area());
            println!("perimeter: {:0.4}", r.perimeter());
        }
    }
}

fn main() {
    let circle=Shape::Circle(Circle{radius: 1.0});
    print_relvant_info(&circle);
    let rectangle=Shape::Rectangle(Rectangle{height:3.2, width:3.2});
    print_relvant_info(&rectangle);
}
