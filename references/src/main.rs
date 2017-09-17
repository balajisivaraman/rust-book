fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
    let mut str = String::from("Hello");
    // below code will not compile because we're creating
    // two mutable references to the same piece of data in the same lexical scope
    // let r1 = &mut str;
    // let r2 = &mut str;
    {
        let r1 = &mut str;
    }
    let r2 = &mut str;
    //above two lines will compile, because r1 is created within its own lexical scope, but below line won't
    // change_mut(&mut str);

    // Below code will also not compile in the third line
    // first 2 lines are OK, but we cannot create a mutable reference
    // when we already have an immutable reference to the same piece of data
    // let r1 = &str;
    // let r2 = &str;
    // let r3 = &mut str;
}

//s is s reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
} //Here s goes out of scope, but because it does not have ownership, nothing happens. Drop is not called.

// Below code will not compile
// Just as variables are immutable by default, so are references
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { dangle returns a reference to a string
//     let s = String::from("hello"); s is a new string
//     &s we return a reference to s
// } but s goes out of scope, so the above line now refers to nothing basically

// fn no_dangle() -> String {
//     let s = String::from("hello"); s is a new string
//     s // we transfer ownership of s this way, so no issues
// } s goes out of scope, but ownership has been transferred, so there are no issues
