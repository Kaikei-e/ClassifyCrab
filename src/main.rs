use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("testText.txt");
    let display = path.display();

    let mut f = match File::open(&path) {
        Err(why) => panic!("couldn't open {} : {}", display, why),
        Ok(f) => f,
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {} : {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s)
    }

}
