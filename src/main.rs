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

    // enums!
    let ordering = cmp(x, y);
    if ordering == Ordering::Less {
        println!("less");
    } else if ordering == Ordering::Greater {
        println!("more");
    } else {
        println!("eq");
    }

    // enums can also contains values
    enum StringResult {
        StringOK(String),
        ErrorReason(String)
    }

    let rest = StringResult::ErrorReason("not ok".to_string());
    let res = StringResult::StringOK("this is ok!".to_string());
    let inner = match res {
        StringResult::StringOK(x) => x,
        StringResult::ErrorReason(y) => y
    };
    let inner2 = match rest {
        StringResult::StringOK(x) => x,
        StringResult::ErrorReason(y) => y
    };
    println!("{}", inner);
    println!("{}", inner2);

    // for loops too!
    for x in 0..10 {
        println!("{}", x);
    }
    
    // and while, oh my!
    let mut done = false;
    let mut cnt = 0;
    while !done {
        cnt += 1;
        if cnt == 3 {
            done = true;
        }
    }

    // can also handle this with break:
    while done {
        cnt -= 1;
        if cnt == 0 { break; }
    }

    // continue is there too, skips to next iteration:
    for x in 0u32..10 {
        if x % 2 == 0 { continue; }
        println!("{}", x);
    }

    // rust has &str (slice) and String strings
    // &str is statically allocated, String is in the heap
    let string = "Hello there";
    let mut s = "Hello".to_string();
    // we can append to Strings
    s.push_str(", world");
    println!("&str: {}  String: {}", string, s);
    // you can coerce String to &str with an &:
    let string1 = &s;
    println!("converted: {}", string1);

    // arrays are immutable by default, and fixed size
    let arr = [1, 2, 3];
    let mut arrr = [1, 2, 3];
    arrr[1] = 3;
    // arrays can also be initialized via [T; N] (type, quantity)
    let arr2 = [0; 20]; // array of 20 0s

    // arrays are iterated over via .iter() (length via .len())
    for e in arr.iter() {
        println!("{}", e);
    }

    // arrays are accessed in a familiar fashion:
    println!("{}, {}", arrr[1], arr2[10]);

    // vectors are dynamic arrays that can grow, created via macro:
    let v = vec![1, 2, 3];
    let mut nums = v;
    nums.push(4); // add to vectors via push
    println!("the number of vals in vec is now {}", nums.len()); // length via .len()

    // vectors/arrays also allow slicing. allows a safe access to a portion of an array without
    // coping
    let a = [0, 1, 2, 3, 4];
    let middle = &a[1..4]; // slice of a
    for e in middle.iter() {
        println!("{}", e);
    }
    // you can also slice arrays, strings, Strings. Slices have type &[T]
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
fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}
