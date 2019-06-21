fn main() {
    variables();
    println!("Hello, world!");
    data_types();
    println!("return of a function {}", function_that_returns_something());
    control_flow(10);
    loops();
    pattern_matching(20);
}

// course 1
fn variables() -> (u32, i8, u8, f64, char) { // return values for tuples in fn need to have determined size
    let astring = "string";
    let a: u32 = 1;
    let b: i8 = -1;
    let c: u8 = 1;
    let f: f64 = 10.0001;
    let ch: char = astring.chars().next().unwrap(); // unwrap takes the item of a option
    (a, b, c, f, ch)
}

fn data_types() {
    let a = [1, 2, 3];
    let b = &a[0..1];

    // fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
    println!("a is an array {:?}", a);
    println!("The value of b should be 1: {:?}", b);
}

fn function_that_returns_something() -> i32 {
    let a = 1;
    let b = 2;
    a + b //no ; == return
}

fn control_flow(age: i8) {
    // parentesis not required
    let discount = if age < 10 {
        100
    } else if age < 20 {
        50
    } else {
        0
    }; // required to close the assignment

    println!("Discount of {}%!", discount);
}

fn loops() {
    for i in 9..21 {
        control_flow(i)
    }
}

fn pattern_matching(age: i8) {
    let discount = match age {
        0...10 => 100,
        10...20 => 50,
        _ => 0
    };

    println!("Discount of {}%!", discount);
}