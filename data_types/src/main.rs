fn main() {
    let x = 2.0;

    let y: f32 = 3.0;

    println!("X is {x} and Y is {y}");

    let sum = 5 + 10;

    let sub = 95.6 - 21.2;

    let multi = 51.2 * 21.99;

    let quot = 58.7 / 221.5;

    let trunc = -5 / 21;

    let remain = 54 % 5;

    println!("sum is {sum}");
    println!("sub is {sub}");
    println!("multi is {multi}");
    println!("quot is {quot}");
    println!("trunc is {trunc}");
    println!("remain is {remain}");

    let t: bool = true;

    let f: bool = false;

    println!("{t} {f}");

    let c: char = 'z';

    let emoji = '🦀';

    println!("{c} {emoji}");

    let tup = (500, 6.3, 1);

    let (x, y, z) = tup;

    println!("{x} {y} {z}");

    let first = tup.0;

    let second = tup.1;

    let third = tup.2;

    println!("Another way: {first} {second} {third}");

    let a = [1, 2, 3, 4, 5];

    let first = a[0];

    println!("{first}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];

    println!("{first}");

    let a: [i32; 5] = [3; 5];

    let first = a[0];

    println!("{first}");

}
