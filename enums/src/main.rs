enum IpAddrKind {
    //better safety through use of different structures for the enum types
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    println!("Hello, world!");
    // this is how you create instances of enum variants
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

// This is one possible way to create new IP addresses
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
