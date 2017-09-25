use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    // println!("the list is {:?}", list);

    let mut x = 5;
    {
        let y = &mut x;

        *y += 1;
    }
    assert_eq!(6, x);

    let my_favorite_song = Mp3 {
        audio: vec![1, 2, 3],
        artist: Some(String::from("U2")),
        title: Some(String::from("One")),
    };
    assert_eq!(vec![1, 2, 3], *my_favorite_song);

    // without deref coercion, we'd have to write
    compress_mp3(my_favorite_song.audio.as_slice());
    // this aut finds that Mp3 implements deref and gives us &Vec<u8>
    // it then find that Vec<u8> implements deref and gives us &[u8]
    compress_mp3(&my_favorite_song);

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    println!("Wait for it...");

    let value1 = Rc::new(RefCell::new(5));
    let value2 = Rc::new(RefCell::new(10));
    let value3 = Rc::new(RefCell::new(3));
    let a = Rc::new(Cons(value1.clone(), Rc::new(Cons(value2, Rc::new(Nil)))));
    println!("rc = {}", Rc::strong_count(&a));
    let b = Cons(value3, a.clone());
    *value1.borrow_mut() += 10;
    println!("shared_list after = {:?}", a);
    println!("b after = {:?}", b);
    // println!("rc after creating b = {}", Rc::strong_count(&a));
    // {
    // let a = Rc::new(Cons(value1, Rc::new(Cons(value2, Rc::new(Nil)))));
    // println!("rc = {}", Rc::strong_count(&a));
    // let b = Cons(3, a.clone());
    // println!("rc after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, a.clone());
    //     println!("rc after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("rc after c goes out of scope = {}", Rc::strong_count(&a));

    let refCellData = RefCell::new(5);
    demo(&refCellData);
}

fn compress_mp3(audio: &[u8]) -> Vec<u8> {
    Vec::new()
    // actual implementation goes here
}

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>,
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer!");
    }
}

fn a_fn_that_immutably_borrows(a: &i32) {
    println!("a is {}", a);
}

fn a_fn_that_mutably_borrows(a: &mut i32) {
    *a += 1;
}

fn demo(r: &RefCell<i32>) {
    a_fn_that_immutably_borrows(&r.borrow());
    a_fn_that_mutably_borrows(&mut r.borrow_mut());
    a_fn_that_immutably_borrows(&r.borrow());
}
