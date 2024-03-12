fn main() {
    let mut counter = 0;

    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                break;
            }
            if count == 5 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count +=1
    }
    println!("End count = {count}");

    let mut number = 10;

    while number != 0 {
        println!("{number}");
        number -= 1
    };

    println!("LIFT OFF!");

    let a = [10, 20, 30, 40 ,50];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("The value with for loop is: {element}");
    }
}
