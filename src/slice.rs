
pub fn run() {
    // This implementation is weak because the usize value returned
    // from first_word immediately loses meaning once s.clear() is called on line 7.
    let mut s = String::from("hello world!");
    let word = first_word(&s);
    s.clear();

    // Managing these two pieces of state and ensuring they don't get out of sync
    // could get annoying. Thus we have string slices.
    let string_slice = String::from("hello world!");
    let hello = &string_slice[0..5];
    let world = &string_slice[6..11];


}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Giga chad rustaceans would use the string literal type by default
// fn first_word(s: &str) -> &str { ... }