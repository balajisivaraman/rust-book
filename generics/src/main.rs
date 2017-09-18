struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 56];
    let chars = vec!['a', 'b', 'c'];

    let point = Point { x: 4, y: 3 };

    println!("The largest number is {}", largest(&numbers));
    println!("The largest char is {}", largest(&chars));
    println!(
        "The largest reference number is {}",
        largest_reference(&numbers)
    );
    println!(
        "The largest reference char is {}",
        largest_reference(&chars)
    );
    println!("The value of x is {}", point.get_x());
}

// without the Copy bound, the first line will not compile
// this is because Rust would not know how to move the value of the list
// since it wouldn't know how much memory space it would take
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest_value = list[0];

    for &item in list {
        if item > largest_value {
            largest_value = item;
        }
    }

    largest_value
}

fn largest_reference<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_value = &list[0];

    for item in list {
        if item > largest_value {
            largest_value = item;
        }
    }

    &largest_value
}
// fn largest_number(numbers: &[i32]) -> i32 {
//     let mut largest = numbers[0];

//     for &number in numbers {
//         if number > largest {
//             largest = number;
//         }
//     }

//     largest
// }

// fn largest_char(numbers: &[char]) -> char {
//     let mut largest = numbers[0];

//     for &number in numbers {
//         if number > largest {
//             largest = number;
//         }
//     }

//     largest
// }
