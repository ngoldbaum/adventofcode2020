use anyhow::Result;

use std::collections::HashMap;
use std::io::Read;

fn main() -> Result<()> {
    let contents = get_contents("input")?;
    let contents = contents.trim();

    let mut adapters: Vec<i64> = contents.lines().map(|l| l.parse().unwrap()).collect();
    adapters.push(0);
    let max = adapters.iter().max().unwrap() + 3;
    adapters.push(max);
    adapters.sort();

    // part 1

    let mut differences: HashMap<i64, i64> = HashMap::new();

    adapters
        .windows(2)
        .for_each(|w| *differences.entry(w[1] - w[0]).or_insert(0) += 1);

    dbg!(differences[&3] * differences[&1]);

    // part 2
    let mut counts: HashMap<i64, i64> = HashMap::new();
    counts.insert(0, 1);
    adapters.iter().enumerate().skip(1).for_each(|(i, &jolt)| {
        counts.insert(
            jolt,
            adapters[..i]
                .iter()
                .filter(|&&oj| oj < jolt && oj >= (jolt - 3))
                .map(|&m| counts[&m])
                .sum(),
        );
    });

    dbg!(counts[&max]);

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = std::fs::File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
