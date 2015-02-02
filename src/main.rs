// to import things, use 'use', double colon (::) indicates namespacing:
use std::cmp::Ordering;

// simplest function def; no args or return types
fn main() {
    // variable binding:
    let x = 5;

    // variables are immutable by default
    // to make a mutable variable
    let mut y = 1;
    println!("y is {}", y);
    y = 2;
    println!("now it's {}", y);

    // basic if expression:
    if x == 5 {
        println!("x is five!");
    }

    // rust also has else { } (and else if)
    if y == 1 {
        println!("yo");
    } else {
        println!("hey");
    }

    // in rust, expressions return a value; statements do not
    // in rust, if is an expression
    let y = if x == 5 { 10 } else { 20 };

    // there are 2 kinds of statements:
    // let/bindings:
    let z = y;

    // expression statements - turns expressions into statements:
    // i.e. semicolons
    // let y = if x == 5 { 10; } else { 20; }
    // will not compile

    // basic function (see below)
    print_number(x);

    // basic function that takes an argument (see below) and returns var
    let b = add_one(z);

    // compound data types:
    // tuples:
    let a = tuples();
    let (c, _) = a; // underscore for unused destructured variables

    // and structs:
    let origin = Point{ x: c, y: b };
    println!("The origin is at ({}, {})", origin.x, origin.y);

    // structs can be mutable:
    let mut point  = Point { x: 1, y: 2 };
    println!("This point is mutable: {}", point.x);
    point = Point { x: 3, y: 4};
    println!("Now the x coord is {}", point.x);

    // example of struct tuple
    let rgb = Color(2, 3, 4);
    let Color(r, _, _) = rgb;
    println!("rgb r value is {}", r);

    // example of a newtype
    let length = Inches(10);
    // get the value from a destructuring let
    let Inches(int_length) = length;
    println!("length is {} inches", int_length);
}

// fn that takes args:
fn print_number(x : i32) {
    println!("x is: {}", x);
}

// fn that returns something:
fn add_one(x: i32) -> i32 {
    x + 1 // note that lack of semicolon, this makes this
          // an expression, so we return the result
}

fn tuples() -> (i32, i32) {
    // tuples:
    let x = (1, "hello");
    let y : (i32, &str) = (2, "hello2");

    // let bindings can destructure tuples
    let (a, b) = x;
    println!("{}, {}", a, b);

    // can compare tuples of the same type:
    if x == y {
        println!("yes!");
    }

    // and can use tuples to return multiple values:
    (2, 3)
}

// structs are a record type, like tuple, which give contained elements a name
struct Point {
    x: i32,
    y: i32
}

// tuple structs -> named tuples without named elements
// almost always worse than a struct...
struct Color(i32, i32, i32);
// ..except when there's only one element (newtype):
struct Inches(i32);

// rust also has Enums
enum FooBar {
    Foo,
    Bar,
    FooBar
}

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}
