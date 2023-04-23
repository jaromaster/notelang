use std::{env::args, fs};

use tokenise::tokenise::tokenise;

pub mod tokenise;
pub mod fileinput;
pub mod transpile;

fn main() {
    let file_name = args().nth(1).expect("invalid file name");

    // read in file
    let lines = fileinput::fileinput::read_file(&file_name);
    let tokens = tokenise(lines);

    // transpile
    let result_string = transpile::transpile::transpile(tokens);

    println!("{:?}", result_string);

    // write to html
    fs::write("out.html", result_string.join("")).expect("cannot write to out.html");
}
