pub mod fileinput {
    use std::{fs::File, io::{BufReader, BufRead}};

    /// read in notelang file and return lines
    pub fn read_file(path: &str) -> Vec<String> {
        let mut lines = Vec::new();

        let f = File::open(path);
        if f.is_err() {
            return lines;
        }
        lines = BufReader::new(f.unwrap()).lines().map(|el|el.unwrap()).collect();

        return lines;
    }
}