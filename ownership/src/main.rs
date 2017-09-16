fn main() {
    scope();
    strings();
    var_interaction();
    let s = String::from("hello"); // s comes into scope here
    takes_ownership(s); // ownership of s is passed onto the function
    // println!("{}", s);
    //above statement doesn't compile because s has been moved into the function
    let x = 5; // x comes into scope here
    makes_copy(x); // x is copied in this case being a scalar type
    println!("{}", x); // this statement is still legal
} //x goes out of scope, then s, but s has been moved so nothing happens

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// the below is basically lexical scoping, which is how we're used
// to thinking about the lifecycle of variables
fn scope() {
    //s is not valid here because it's not defined yet
    let s = "hello"; //s becomes valid from this point onward
    //do stuff with s
} //the scope is over and s is no longer valid

fn strings() {
    //This is a string that can be mutated
    //literal strings are immutable, much like those in Java
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn var_interaction() {
    //at the end of both below lines, two 5 values are pushed onto the stack
    //this is easy because 5 is a simple int value with a fixed size
    let x = 5;
    let y = x;
    //when we do the below, the contents of the stack that represent
    //the string are copied. But both have the same pointer referring
    //to the same memory location containing the string hello
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world", s1);
}

fn cloning() {
    let s1 = String::from("hello");
    //below line performs a heap copy of the string
    //this means that s1 and s2 refer to different places in memory
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
