fn main() {
    let s = "hello";

    {   // s is not valid here since its not defined yet
        let s = "hello again"; // s is valid from this point

        println!("{s}"); 
    }   // the scope is now over and s is not valid anymore

    println!("{s}");

    let mut s = String::from("Hello");

    println!("{s}");

    s.push_str(", world!");

    println!("{s}");

    //let s1 = String::from("Hello");

    //let s2 = s1;

    //println!("{s1}"); Error: value borrowed here after move

    let s1 = String::from("Hello");

    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}
