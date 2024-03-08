fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    let x = five();

    println!("{x}");

    let x = plus_one(5);

    println!("{x}");
}

fn another_function(x: i32) {
    println!("Another, function with the parameter {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// function with return value ->
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; would not work, since ; makes the function not return anything
}