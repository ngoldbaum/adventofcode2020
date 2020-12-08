use anyhow::Result;

use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;

type Bag = HashMap<String, usize>;

fn main() -> Result<()> {
    let contents = get_contents("input")?;
    let contents = contents.trim();
    let lines: Vec<&str> = contents.lines().collect();

    let all_bags: HashMap<String, Bag> = HashMap::from_iter(lines.iter().map(|l| {
        let color = l.split(" bags ").next().unwrap().to_string();
        let scontents = l
            .trim_end_matches('.')
            .rsplit(" bags contain ")
            .next()
            .unwrap();
        let contents: Bag = match scontents {
            "no other bags" => HashMap::new(),
            _ => HashMap::from_iter(scontents.split(", ").map(|b| {
                let mut it = b
                    .trim_end_matches(" bags")
                    .trim_end_matches(" bag")
                    .split(" ");
                let count = it.next().unwrap();
                let color = it.collect::<Vec<&str>>().join(" ");
                (color, count.parse::<usize>().unwrap())
            })),
        };
        (color, contents)
    }));

    // part 1
    let mut nleaves = 0;

    for b in &all_bags {
        if has_shiny_gold(b.1, &all_bags) {
            nleaves += 1;
            continue;
        }
    }

    dbg!(nleaves);

    // part 2
    let shiny_gold = all_bags.iter().find(|(k, _)| *k == "shiny gold").unwrap();
    dbg!(count_bags(shiny_gold.1, &all_bags) - 1);

    Ok(())
}

fn count_bags(b: &Bag, all_bags: &HashMap<String, Bag>) -> usize {
    let mut nbags = 1;
    for (cb, count) in b {
        let ob = all_bags.get(cb).unwrap();
        nbags += count * count_bags(ob, &all_bags);
    }
    nbags
}

fn has_shiny_gold(b: &Bag, all_bags: &HashMap<String, Bag>) -> bool {
    let bag = all_bags.values().find(|ob| ob == &b).unwrap();
    match bag.get("shiny gold") {
        Some(_) => true,
        None => {
            if bag.len() > 0 {
                bag.iter()
                    .map(|(k, _)| has_shiny_gold(all_bags.get(k).unwrap(), all_bags))
                    .any(|x| x)
            } else {
                false
            }
        }
    }
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = std::fs::File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
