fn main() {

    let s1 = gives_ownership();

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2.clone());

    println!("{s1} {s2} {s3}");

    let s1 = String::from("Hello");

    let (s2, len) = calculate_len(s1);

    println!("The length of {s2} is {len}")

}

fn gives_ownership() -> String {
    let some_string = String::from("It's yours!");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}