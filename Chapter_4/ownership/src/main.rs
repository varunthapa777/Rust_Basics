fn main() {
    let mut st = String::from("Hello World");

    let result = first_word(&st);
    // st.clear();

    println!("The result of {st} is {result}");
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
