fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        // works becuase string1 is valid in the inner scope
        // the shorter lifetime is validity till the inner scope
        // which both strings meet
        println!("The longest string is {}", result);
    }
    // though string3 has a longer lifetime in this case
    // because lifetime rules enforce usage of the shorter lifetime
    // below code will not compile
    // let string3 = String::from("long string is long");
    // let result1;
    // {
    //     let string4 = String::from("xyz");
    //     result1 = longest(string3.as_str(), string4.as_str());
    // }
    // println!("The longest string is {}", result1);
}

// below code will not compile, because we don't know
// whether the borrowed x or y will be returned, neither does the compiler
// there are no set scopes, like in straightforward examples as well
// fn longest(x: &str, y: &str) -> &str {
//     if x.len > y.len { x } else { y }
// }

// The function signature now says that for some lifetime 'a, the function will get two parameters, both of which are string slices that live at least as long as the lifetime 'a. The function will return a string slice that also will last at least as long as the lifetime 'a. This is the contract we are telling Rust we want it to enforce.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
