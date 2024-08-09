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

//generic enum
//struct Point<T,V>{
//x: T,
//y: T,
//z: V,
//}

//option enum- use to handle null as it does not have a null(option is a prebuild enum no need to define it.)

fn find_a(name: String) -> Option<i32> {
    for (index,char) in name.chars().enumerate(){
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
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

    //error handling- via enum
    //null handling- via pub enum
    let name: String = String::from("Nandini");
    let res: Option<i32> = find_a(name);
    match res{
        Option::Some(index) => println!("there is a {}",index),
        Option::None => println!("not there"),
    }
}

//cargo == npm

fn cal_area(shape: Shape) -> f64{
    let ans: f64 = match shape {
        Shape::Circle(radius) => 3.14*radius*radius,
        Shape::Square(side) => side*side,
        Shape::Rectangle(length,wridth) =>  length*wridth,
    };
    return  ans;
}

//actix: exxtremely fast http serrvice
//serde: serialization and decerialization 
//tokio: async runtime rust
//request: http request
//sqlx: connect sql db
//chrono: date time
