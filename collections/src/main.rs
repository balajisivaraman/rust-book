fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // this is unsafe
    let third: &i32 = &v[0];
    // below code will not compile because we've created a immutable borrow above
    // v.push(6);
    let maybeThird: Option<&i32> = v.get(2);
    println!("the third element is {}", third);
    let s1 = "hello";
    println!("{}", &s1[0..1]);
    let hello = "Здравствуйте";
    // above string represented using 2 bytes for each character
    // so below would result in first 2 characters above
    let s = &hello[0..4];
    // however below code is a runtime error
    // it doesn't make sense to get the first character out here
    // let s = &hello[0..1];
    println!("{}", s);
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
