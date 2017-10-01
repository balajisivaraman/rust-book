use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // unlike the above, we are directly creating a raw pointer
    // from a memory location here. NOTE: NEVER DO THIS
    // let address = 0x012345;
    // let r = address as *const i32;

    // below is a compile time error
    // dont_do_this();

    unsafe {
        dont_do_this();
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5]);

    add_to_count(3);

    println!("name is {}", HELLO_WORLD);
    unsafe {
        println!("COUNTER {}", COUNTER);
    }
}

unsafe fn dont_do_this() {
    // body goes here
}

// this would be the safe way of implementing split at mut, but it doesn't compile
// Rust doesn't know we're borrowing two independent parts of the same slice
// it isn't smart enough to know that, but we are
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     (&mut slice[..mid], &mut slice[mid..])
// }

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr: *mut i32 = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }

}
