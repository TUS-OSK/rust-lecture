use std::{fs::File, io::{Read, self, BufRead}};

fn main() -> Result<(), std::io::Error> {
    let f = File::open("hello.txt");

    //let f = f.unwrap();
    //let f = f.expect("Failed to open hello.txt");

    let f = match f {
        Ok(value) => value,
        Err(value) => panic!("err: {:?}", value),
    };

    let hello = io::BufReader::new(f)
    .lines()
    .into_iter()
    .next()
    .unwrap()?;

    println!("{}", hello);

    Ok(())
}

fn read_hello_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(value) => value,
        Err(value) => return Err(value),
    };

    let hello = io::BufReader::new(f)
    .lines()
    .into_iter()
    .next()
    .expect("Failed to read line in hello.txt");

    let hello = match hello {
        Ok(value) => value,
        Err(value) => return Err(value),
    };

    Ok(hello)
}

fn read_hello_file_with_shortcut() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt")?;

    let hello = io::BufReader::new(f)
    .lines()
    .into_iter()
    .next()
    .expect("Failed to read line in hello.txt")?;

    Ok(hello)
}