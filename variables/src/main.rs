use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

fn main() {
    // let mut x = 5;
    // println!("The value of x is {x}");
    // x = 6;
    // println!("The value of x is {x}");
    // const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;
    // println!("The value of x is {THREE_HOURS_IN_SECONDS}");

    // let x = 5;
    // let x = x + 1;
    // println!("The value of x is {x}");

    // {
    //     let x = x * 2;
    //     println!("The value of x is {x}");
    // }

    // println!("The value of x is {x}");

    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!("Value of spaces is {spaces}")

    // let mut spaces = "    ";
    // println!("here are my spaces: {}", spaces);
    // println!("here is the len: {}", spaces.len());
    // println!("Value of spaces is {spaces}")

    // let guess: u32 = "33".parse().expect("bad number");
    // println!("Value of guess is {guess}");

    // let x = 2.0; // f64
    // let y: f32 = 3.0;
    // print_type_of(&x);
    // println!("x is {x} and y is {y}");

    // let sum = 5 + 10;
    // let diff = 95.5 - 43.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.3;
    // let truncated = -5 / 3;
    // let remainder = -5 % 3;
    // println!("remainder is {remainder}");

    // let _t = true;
    // let _f: bool = false;

    // let this_is_a_string = "c";
    // print_type_of(&this_is_a_string);
    // let _z: char = 'Z';
    // let heart_eyed_cat = '😻';
    // println!("heart eyed cat {heart_eyed_cat}");

    // let tup: (i32, f64, char) = (5000, 100.5, 'c');
    // let (a, b, c) = tup;
    // print_type_of(&tup);
    // println!("tup is ({a}, {b}, {c})");
    // let x = tup.0;
    // let y = tup.1;
    // let z = tup.2;
    // println!("tup is ({x}, {y}, {z})");

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let b = a[0];
    // let a = [3; 5];
    // let b = a[3];
    // println!("list is {b}");

    println!("Hi Marissa and Regan")
}
