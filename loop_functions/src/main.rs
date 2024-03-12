fn main() {
    let s: String = String::from("Hello");

    takes_ownership(s);

    let x: i32 = 5;

    makes_copy(x)
}


fn takes_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}")
}
