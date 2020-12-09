use anyhow::Result;
use itertools::Itertools;

use std::io::Read;

fn main() -> Result<()> {
    let input_name = "input";
    let npreamble: usize = match input_name {
        "input.test" => 5,
        "input" => 25,
        _ => panic!(),
    };

    let contents = get_contents(input_name)?;
    let contents = contents.trim();

    let stream: Vec<usize> = contents.lines().map(|p| p.parse().unwrap()).collect();

    // part 1

    let p1 = match stream[..].windows(npreamble + 1).try_for_each(|w| {
        let preamble = &w[..npreamble];
        let test_digit = &w[npreamble];
        match preamble
            .iter()
            .combinations(2)
            .any(|c| c[0] + c[1] == *test_digit)
        {
            true => Ok(()),
            false => Err(*test_digit),
        }
    }) {
        Ok(_) => panic!(),
        Err(answer) => dbg!(answer),
    };

    // part 2

    let mut i = 0;
    let mut j;

    'outer: loop {
        j = i + 1;
        'inner: loop {
            let s: usize = stream[i..j].iter().sum();
            if s > p1 {
                break 'inner;
            } else if s == p1 {
                break 'outer;
            }
            j += 1;
        }
        i += 1;
    }

    dbg!(stream[i..j].iter().min().unwrap() + stream[i..j].iter().max().unwrap());

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = std::fs::File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
