fn main() {
    // 12 Days of Xmas

    let mut day_counter = 12;

    let items = ["Drummers Drumming", "Pipers Piping"];

    for item in items {
        println!("{day_counter} {item}");

        day_counter -= 1;
    }
}
