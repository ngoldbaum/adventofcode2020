use anyhow::Result;

use std::collections::HashMap;
use std::io::Read;
use std::iter::FromIterator;

#[derive(Debug)]
struct Bag {
    color: String,
    contents: HashMap<String, usize>,
}

fn main() -> Result<()> {
    let contents = get_contents("input")?;
    let contents = contents.trim();
    let lines: Vec<&str> = contents.lines().collect();

    let all_bags: Vec<Bag> = lines
        .iter()
        .map(|l| {
            let color = l.split(" bags ").next().unwrap().to_string();
            let scontents = l
                .trim_end_matches('.')
                .rsplit(" bags contain ")
                .next()
                .unwrap();
            let contents: HashMap<String, usize>;
            if scontents == "no other bags" {
                contents = HashMap::new();
            } else {
                contents = HashMap::from_iter(scontents.split(", ").map(|b| {
                    let mut it = b
                        .trim_end_matches(" bags")
                        .trim_end_matches(" bag")
                        .split(" ");
                    let count = it.next().unwrap();
                    let color = it.collect::<Vec<&str>>().join(" ");
                    (color, count.parse::<usize>().unwrap())
                }));
            }
            Bag {
                color: color,
                contents: contents,
            }
        })
        .collect();

    let mut nleaves = 0;

    for b in &all_bags {
        if has_shiny_gold(&b.color, &all_bags) {
            nleaves += 1;
            continue;
        }
    }

    dbg!(nleaves);

    // part 2
    let shiny_gold = all_bags.iter().find(|b| b.color == "shiny gold").unwrap();
    dbg!(count_bags(shiny_gold, &all_bags) - 1);

    Ok(())
}

fn count_bags(b: &Bag, all_bags: &Vec<Bag>) -> usize {
    let mut nbags = 1;
    for (c, count) in &b.contents {
        let b = all_bags.iter().find(|bag| bag.color == *c).unwrap();
        nbags += count * count_bags(b, &all_bags);
    }
    nbags
}

fn has_shiny_gold(color: &str, all_bags: &Vec<Bag>) -> bool {
    let bag = all_bags.iter().find(|b| b.color == color).unwrap();
    match bag.contents.get("shiny gold") {
        Some(_) => true,
        None => {
            let keys = bag.contents.keys();
            if keys.len() > 0 {
                keys.map(|c| has_shiny_gold(c, all_bags)).any(|x| x)
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
