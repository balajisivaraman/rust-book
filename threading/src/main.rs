use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle1 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle1.join();

    let handle = thread::spawn(|| for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join();
}
