fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("first slice is {}", word);

    // below line doesn't compile because we have an immutable reference above, so it follows the rules of referencing
    // s.clear();

    // let word = first_word(&s);

    // s.clear();

    //word will still have 5 here, but the string has been emptied, so it is invalid
    //this is a very inconvenient way of handling portions of a data structure
}

// this implementation is hugely problematic
// because the int being returned is independent of the string itself
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}
