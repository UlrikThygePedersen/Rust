fn main() {

    const CONSTANT_THING: u32 = 42;
    
    let x = 5;

    println!("The value of X is: {x}");

    let x = x + 1;

    {
        let x = x * 2;

        println!("The value of X in the inner scope is: {x}");
    }

    println!("The value of X is now: {x}");

    println!("The constant is still constant at the value of {CONSTANT_THING}");

    let spaces = "    ";
    let spaces = spaces.len();

    println!("There are {spaces} spaces in the string");
}
