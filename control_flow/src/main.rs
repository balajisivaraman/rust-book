fn main() {
    let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index = index + 1;
    // }
    for elem in a.iter() {
        println!("the value is: {}", elem);
    }
}
