use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;

fn main() {
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("failed to create: {:?}", e);
                },
            }
        },
        Err(error) => {
            panic!("failed to open: {:?}", error);
        },
    };

    // let f = File::open("hoge.txt").unwrap();
    // let f = File::open("hoge.txt").expect("FAILED!!!!!!!!!");

    let s = match read_from_file_2() {
        Ok(s) => s,
        Err(e) => {
            panic!("failed to read: {:?}", e);
        },
    };

    println!("{}", s);
}

fn read_from_file() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
