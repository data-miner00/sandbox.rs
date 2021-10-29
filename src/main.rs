// use std::io;
// use std::fs::File;
// use std::io::prelude::*;

// fn main() -> io::Result<()> {

//     let mut f = File::open("src/models.rs")?;
//     let mut buffer = [0; 10];

//     let n = f.read(&mut buffer)?;

//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }

fn foo(input: Option<i32>) -> Option<i32> {
    match input {
        Some(i) if i >= 0 => Some(i),
        _ => None
    }
}

fn main() {
    let resutl = match foo(Some(41)) {
        Some(i) => i,
        None => -999
    };

    println!("{}", resutl);
}