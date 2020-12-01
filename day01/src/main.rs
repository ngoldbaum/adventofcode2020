use itertools::Itertools;

use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input");

    // part 1
    let values = contents
        .trim()
        .split("\n")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for c in values.iter().combinations(2) {
        if c[0] + c[1] == 2020 {
            dbg!(c[0] * c[1]);
        }
    }

    // part 2
    for c in values.iter().combinations(3) {
        if c[0] + c[1] + c[2] == 2020 {
            dbg!(c[0] * c[1] * c[2]);
        }
    }

    Ok(())
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}
