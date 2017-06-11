use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];

    // v[100];

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file but there was a problem: {:?}", e);
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!("There was a problem opening the file: {:?}", error);
    //     }
    // };

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
