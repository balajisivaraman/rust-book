use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        //the ref is needed so that error is not moved into the guard condition
        //but merely referenced by it
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("There was a problem creating the file: {:?}", e),
            }
        }
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let f = File::open("hello.txt")?.read_to_string(&mut s)?;
    // when you use ? operator, it behaves exactly the same as above
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    Ok(s)
}
