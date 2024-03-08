fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true!");
    } else {
        println!("Condition was false!");
    }

    if number != 0 {
        println!("Number was different from 0!");
    }

    let number: i32 = 8;

    if number % 4 == 0 {
        println!("Number is divisible by 4")
    } else if number % 3 == 0 {
        println!("Number is divisible by 3")
    } else if number % 2 == 0 {
        println!("Number is divisible by 2")
    } else {
        println!("Number is not divisible by 4, 3 or 2")
    }

    let condition = true;

    let number = if condition {5} else {6};

    println!("Number is {number}, because the condition was {condition}")
}
