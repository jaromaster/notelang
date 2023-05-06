use std::{env::args, fs};

use tokenise::tokenise::tokenise;

pub mod tokenise;
pub mod fileinput;
pub mod transpile;
pub mod html_structure;

fn main() {
    let file_name = args().nth(1).expect("invalid file name");

    // read in file
    let lines = fileinput::fileinput::read_file(&file_name);
    let tokens = tokenise(lines);
    let (file_name, _) = file_name.split_once(".").unwrap();

    // transpile
    let result_string = transpile::transpile::transpile(tokens, &file_name);

    // println!("{:?}", result_string);

    // write to html
    let out_file = format!("{}.html", file_name);
    fs::write(out_file, result_string.join("")).expect("cannot write to html file");
}
