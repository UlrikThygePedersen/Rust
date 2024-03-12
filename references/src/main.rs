fn main() {
    let s1 = String::from("Hello");

    let len = calculate_len(&s1);

    println!("{s1} is still here and the length is {len}");

    let mut s = String::from("Hello");

    change(&mut s);

    println!("{s}");



}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
