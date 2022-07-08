fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    let s2 = s.clone();

    println!("s={}, s2 = {}", s, s2);

    takes_ownership(s);
    
    let x = 5;
    makes_copy(x);

    println!("string: {}, int: {}", s2, x);

    let len = calculate_length(&s2);
    println!("The length of '{}' is {}", s2, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("changed? {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}
