use std::fmt;
use std::mem;

macro_rules! print_title {
    ($text:expr, $width:expr) => {
        println!("{:*<width$} {text} {:*<width$}", "", "", text=$text, width=$width);
    };
}

fn print_primitive_sizes() {
    let var_i8:i8 = 0;
    let var_i16:i16 = 0;
    let var_i32:i32 = 0;
    let var_i64:i64 = 0;
    let var_i128:i128 = 0;
    let var_isize:isize = 0;

    let var_u8:u8 = 0;
    let var_u16:u16 = 0;
    let var_u32:u32 = 0;
    let var_u64:u64 = 0;
    let var_u128:u128 = 0;
    let var_usize:usize = 0;

    let var_f32:f32 = 0.0;
    let var_f64:f64 = 0.0;
    let var_char:char = 'a';
    let var_bool:bool = true;

    let array:[i8; 3] = [1,2,3];
    let tuple:(i8, bool) = (1,true);

    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_i8), bytes=std::mem::size_of_val(&var_i8));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_i16), bytes=std::mem::size_of_val(&var_i16));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_i32), bytes=std::mem::size_of_val(&var_i32));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_i64), bytes=std::mem::size_of_val(&var_i64));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_i128), bytes=std::mem::size_of_val(&var_i128));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_isize), bytes=std::mem::size_of_val(&var_isize));

    println!("");
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_u8), bytes=std::mem::size_of_val(&var_u8));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_u16), bytes=std::mem::size_of_val(&var_u16));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_u32), bytes=std::mem::size_of_val(&var_u32));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_u64), bytes=std::mem::size_of_val(&var_u64));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_u128), bytes=std::mem::size_of_val(&var_u128));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_usize), bytes=std::mem::size_of_val(&var_usize));

    println!("");
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_f32), bytes=std::mem::size_of_val(&var_f32));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_f64), bytes=std::mem::size_of_val(&var_f64));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_char), bytes=std::mem::size_of_val(&var_char));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(var_bool), bytes=std::mem::size_of_val(&var_bool));

    println!("");
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(array), bytes=std::mem::size_of_val(&array));
    println!("Variable {var_name} has {bytes} bytes", var_name=stringify!(tuple), bytes=std::mem::size_of_val(&tuple));
}

fn test_mutability() {
    let mut value = 10;
    println!("Value is {value}", value=value);

    value = 12;
    println!("Value is {value}", value=value);

    let value = true;
    println!("Value is {value}", value=value);
}

fn test_literals_operators() {
    let a = 1u32+2;
    let b = true & (0x00 != 0);
    let not: bool = !true;
    let shift = 0b0001u8 << 4;
    let one_million = 1_000_000;

    println!("a: {a}");
    println!("b: {b}");
    println!("not: {not}");
    println!("shift: 0b{shift:08b}");
    println!("one_million: {one_million}");
}

fn swap(pair: (u32, bool)) -> (bool, u32) {
    /* Unpacking */
    let (val_u32, val_bool) = pair;

    /* returned value is a tuple */
    (val_bool, val_u32)
}

fn test_tuples() {
    let to_swap = (0xFF, false);
    let swapped = swap(to_swap);

    println!("to_swap: {to_swap:?}, swapped: {swapped:?}");
}

#[derive(Debug)]
struct Matrix2x2(u32, u32, u32, u32);

impl fmt::Display for Matrix2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( {} {} )\n", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn transpose_matrix_2x2(matrix:Matrix2x2) -> Matrix2x2 {
    Matrix2x2(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn activity_exercise() {
    let matrix = Matrix2x2(1,2,3,4);
    println!("Matrix is:\n{matrix}\n");

    let transpose = transpose_matrix_2x2(matrix);
    println!("Transpose is:\n{transpose}");
}

fn test_arrays_and_slices() {
    let arr = [1,2,3,4,5];
    let zeros = [0; 50];

    let arr_2:[i8; 5] = [1,2,3,4,5];
    let zeros_2:[i8; 50] = [0; 50];

    println!("arr has {arr_len} elements, zeros has {zeros_len} elements", arr_len=arr.len(), zeros_len=zeros.len());
    println!("arr_2 has {arr_2_len} elements, zeros_2 has {zeros_2_len} elements", arr_2_len=arr_2.len(), zeros_2_len=zeros_2.len());

    println!("sizeof(arr) is {arr_size}, sizeof(zero) is {zero_size}", arr_size=mem::size_of_val(&arr), zero_size=mem::size_of_val(&zeros));
    println!("sizeof(arr_2) is {arr_2_size}, sizeof(zeros_2) is {zeros_2_size}", arr_2_size=mem::size_of_val(&arr_2), zeros_2_size=mem::size_of_val(&zeros_2));

    let target = [0,1,2,3,4,5,6,7,8,9,10];
    let slice = &target[3..7];
    println!("Slice of target is {slice:?}");

    println!("5th element of target is {target_5}", target_5=target[5]);

    for i in 0..target.len()+1 {
        let result: Option<&i32> = target.get(i);
        match result {
            Some(target_val) => print!(" |{i}: {target_val}"),
            None => println!("\nIndex Out of Bounds {i}")
        }
    }
}

fn main() {

    print_title!("print_primitive_sizes()", 15);
    print_primitive_sizes();
    println!("\n");

    print_title!("test_mutability()", 15);
    test_mutability();
    println!("\n");

    print_title!("test_literals_operators()", 15);
    test_literals_operators();
    println!("\n");

    print_title!("test_tuples()", 15);
    test_tuples();
    println!("\n");

    print_title!("activity_exercise()", 15);
    activity_exercise();
    println!("\n");

    print_title!("test_arrays_and_slices()", 15);
    test_arrays_and_slices();
    println!("\n");
}
