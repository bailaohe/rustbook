fn main() {
    let mut s: String = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let mut s2 = s;
    println!("{}", s2);

    let len = calculate_length(&s2);

    println!("The length of '{}' is {}.", s2, len);

    change(&mut s2);

    let fuck = String::from("hello world");

    let word = first_word(&fuck);

    println!("{}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", human");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}