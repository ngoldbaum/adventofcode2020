use std::collections::HashSet;
use std::io::Read;
use std::iter::FromIterator;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let contents = get_contents("input")?;

    let groups = contents.trim().split("\n\n").collect::<Vec<&str>>();

    // part 1
    let p1groups = groups
        .iter()
        .map(|g| HashSet::from_iter(g.split("\n").map(|p| p.chars()).flatten()))
        .collect::<Vec<HashSet<char>>>();

    dbg!(p1groups.iter().map(|g| g.len()).sum::<usize>());

    // part 2
    let p2groups = groups
        .iter()
        .map(|g| {
            g.split("\n")
                .map(|p| HashSet::from_iter(p.chars()))
                .collect::<Vec<HashSet<char>>>()
        })
        .collect::<Vec<Vec<HashSet<char>>>>();

    let mut nall: Vec<usize> = Vec::new();

    for g in p2groups {
        let mut first = match g.iter().next() {
            Some(f) => f.clone(),
            None => continue,
        };
        for p in g {
            first = first.intersection(&p).map(|p| *p).collect();
        }
        nall.push(first.len());
    }

    dbg!(nall.iter().sum::<usize>());

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = std::fs::File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
