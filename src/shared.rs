use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub struct AOCParseError;

pub fn read_lines(filepath: &str) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let x = reader.lines().filter_map(|line| line.ok());

    Ok(x)
}
