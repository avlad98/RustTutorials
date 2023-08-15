#![allow(dead_code)]

macro_rules! print_title {
    ($text:expr, $width:expr) => {
        println!("{:*<width$} {text} {:*<width$}", "", "", text=$text, width=$width);
    };
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}

fn test_destructuring() {
    let rect = Rectangle{top_left: Point{x:-1.0, y:1.0}, bottom_right: Point{x:5.0, y:5.0}};
    let Rectangle{top_left: top_left_point, bottom_right: bottom_right_point} = rect;

    println!("top_left_point:\t\t{top_left_point:?}");
    println!("bottom_right_point:\t{bottom_right_point:?}");

}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle{top_left: Point{x: x_left, y: y_top}, bottom_right: Point{x: x_right, y: y_bottom}} = rect;

    /* A = width * height = (x_right - x_left) * (y_bottom - y_top) */
    let area = (x_right - x_left) * (y_bottom - y_top);

    area
}

fn square(top_left: Point, side_length:f32) -> Rectangle {

    let Point{x: x_left, y: y_top} = top_left;
    let bottom_right = Point{x: x_left + side_length, y: y_top + side_length};
    let ret_square = Rectangle{top_left: top_left, bottom_right: bottom_right};

    ret_square
}

fn activity() {
    /* 1st exercise */
    let rect = Rectangle{top_left: Point{x:-1.0, y:1.0}, bottom_right: Point{x:5.0, y:5.0}};
    let area = rect_area(&rect);
    println!("Area of {rect:?} is {area}");

    /* 2nd exercise */
    let square_top_left = rect.top_left;
    let side_length = 2.5;
    let square_var = square(square_top_left, side_length);
    println!("Square is {square_var:?} and has area equal to {square_area}", square_area=rect_area(&square_var));
}

#[derive(Debug)]
enum Shape {
    Sedan,
    Coupe,
    Break,
    Hatchback,
    Liftback,
    SUV
}

#[derive(Debug)]
enum Vehicle {
    Bus,
    Tractor(f32),
    Car{horsepower: u16, shape: Shape}
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red = 0xFF0000,
    Green = 0x00FF00,
    Blue = 0x0000FF
}

enum Increment {
    First = 1,
    Second,
    Third
}

type MyVehicle = crate::Vehicle;

fn decode_enum(vehicle: &Vehicle) {
    match vehicle {
        Vehicle::Bus => println!("The vehicle is a Bus"),
        Vehicle::Tractor(weight_tons) => println!("The vehicle is a {weight_tons} tonnes Tractor"),
        Vehicle::Car { horsepower, shape } => println!("The vehicle is a {horsepower} HP {shape:?} Car")
    }
}

fn test_enum() {
    let bus = Vehicle::Bus;
    let tractor_4p56_tonnes = Vehicle::Tractor(4.56);
    let insignia = Vehicle::Car { horsepower: 260, shape: Shape::Hatchback };

    for (_, vehicle) in [bus, tractor_4p56_tonnes, insignia].iter().enumerate() {
        decode_enum(vehicle);
    }

    let red_color = Color::Red;
    println!("Red color has value: {:?} = 0x{:06X}", red_color, red_color.clone() as u32);

    let first = Increment::First;
    let second = Increment::Second;
    let third = Increment::Third;
    println!("First is {first}, Second is {second}, Third is {third}", first=first as u8, second=second as u8, third=third as u8);
}

//// https://betterprogramming.pub/learning-rust-building-a-linked-list-102bcb08f93b
// #[derive(Debug, Clone)]
// enum LinkedList<T> {
//     Node(T, Box<LinkedList<T>>),
//     Null
// }

// impl<T: std::fmt::Debug> LinkedList<T> {
//     fn new() -> LinkedList<T> {
//         LinkedList::Null
//     }
    
//     fn prepend(self, data: T) -> LinkedList<T> {
//         LinkedList::Node(data, Box::new(self))
//     }

//     fn change_next(self, new_data: T) -> LinkedList<T> {
//         match self {
//             LinkedList::Node(data, _) => LinkedList::Node(data, Box::new(LinkedList::Node(new_data, Box::new(LinkedList::Null)))),
//             LinkedList::Null => LinkedList::Null,
//         }
//     }

//     // fn append(&mut self, data: T) {
//     //     let list = self;

//     //     println!("Adding element: {data:?}");


//     //     println!("");
//     // }

//     fn print(&self) {
//         let mut list = self;
//         let mut index:u32 = 0;
//         while let LinkedList::Node(data, next) = list {
//             println!("Element {index}: {data:?}");
//             index += 1;
//             list = &next;
//         }
//     }
// }

// fn test_linked_list() {
//     let mut list = LinkedList::new();
    
//     list.prepend(1);
//     list.change_next(2);

//     // list = list.prepend(1);
//     // list = list.prepend(2);
//     // list = list.prepend(3);
//     // list = list.prepend(4);

//     list.print();
// }


/* GLOBALS */
static CHAPTER: &str = "Chapter 3. Custom Types";
const VERSION:f32 = 0.3;


fn main() {
    print_title!("test_destructuring()", 15);
    test_destructuring();
    println!("");

    print_title!("activity()", 15);
    activity();
    println!("");
    
    print_title!("test_enum()", 15);
    test_enum();
    println!("");

    // print_title!("test_linked_list()", 15);
    // test_linked_list();
    // println!("");
}
