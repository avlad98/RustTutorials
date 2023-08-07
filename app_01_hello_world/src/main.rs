use std::fmt::{self};

fn formatted_text() {
    let text = format!("Hello all, formatted value print: {one:b}, 0x{a:x}, 0x{fifteen:>3X}",
        one = 1, 
        a = 10,
        fifteen = 15
    );

    eprintln!("{}", text);

    let width = 5;
    println!("Test padding: {value:*>width$}", value = 9);
    println!("Test padding: {value:*<width$}", value = 9);
}

fn dead_code() {
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));
}

/* Consult std::fmt */
fn print_first_3digits_pi() {
    let pi = 3.141592;

    println!("PI is {:.3}", pi);
}

fn debug() {
    #[derive(Debug)]
    struct DebugPrintable(i32);

    println!("DebugPrintable struct is: {:?}", DebugPrintable(6));


    #[derive(Debug)]
    struct Deep(DebugPrintable);
    println!("Deep struct is: {:?}", Deep(DebugPrintable(6)));

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }
    
    let person = Person{name:"Huroaie", age:27};
    println!("Person is {:#?}", person);
}

fn implement_display_trait() {
    struct Person {
        name: String,
        age: u8
    }

    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Person: name = {} | age = {}", self.name, self.age)
        }
    }


    let mijoi = Person{name: "Mijoi".to_string(), age: 27};
    println!("{}", mijoi)
}

#[derive(Debug)]
struct Complex(f32, f32);

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.1 >= 0.0 {"+"} else {"-"};
        write!(f, "{real} {sign} {imag}i",
                real = self.0,
                imag = self.1.abs(),
                sign = sign)
    }
}

fn test_complex_number() {
    let complex = Complex(3.3, 7.2);

    println!("Complex number is: {}", complex);
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        for (idx, val) in vec.iter().enumerate() {
            write!(f, "{idx}: {val}", idx = idx, val = val)?;

            if idx != (vec.len() - 1) {
                write!(f, ", ")?;
            }

            write!(f, "")?;
        }

        write!(f, "]")
    }
}

fn test_list_impl() {
    let list = List([0,1,2,3].to_vec());

    println!("List is: {}", list);
}


#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({red}, {green}, {blue}) 0x{red:0>2X}{green:0>2X}{blue:0>2X}",
            red = self.red, green = self.green, blue = self.blue)
    }
}

fn test_color_impl() {
    let color = Color{red: 0, green: 0xB, blue: 255};

    println!("Color is {}", color);
}

/// Doc: Main function
fn main() {
    // Comment Style 1
    /* Comment Style 2 */

    //! Doc calling println macro with ! 
    println!("Hello World!");

    formatted_text();
    dead_code();
    print_first_3digits_pi();
    debug();
    implement_display_trait();
    test_complex_number();
    test_list_impl();
    test_color_impl();
}