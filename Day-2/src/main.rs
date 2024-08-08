use core::f64;

//Used to define a type that groups together related data. Each field in a struct can have a different type.
struct User{
    name: String,
    age: u8,
    active: bool,
}

//enum: Used to define a type that can be one of several variants. Each variant can have different types and amounts of associated data.
enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn main() {
    //struct there are also tuple struct and unit struct- struct User;      
    let name: String = String::from("Nandini");
    let user: User = User{
        name,
        age:20,
        active: true,
    };
    println!("{}",user.name);

    let circle: Shape = Shape::Circle(5.0);
    let area: f64 = cal_area(circle);
    println!("{}",area);

    //error handling
    
}

fn cal_area(shape: Shape) -> f64{
    let ans: f64 = match shape {
        Shape::Circle(radius) => 3.14*radius*radius,
        Shape::Square(side) => side*side,
        Shape::Rectangle(length,wridth) =>  length*wridth,
    };
    return  ans;
}
