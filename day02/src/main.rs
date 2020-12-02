use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Policy {
    min: u32,
    max: u32,
    letter: char,
}

impl FromStr for Policy {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl = s.split(|c| c == '-' || c == ' ').collect::<Vec<&str>>();
        Ok(Policy {
            min: spl[0].parse::<u32>()?,
            max: spl[1].parse::<u32>()?,
            letter: spl[2].chars().next().unwrap(),
        })
    }
}

fn main() -> Result<(), std::num::ParseIntError> {
    let contents = get_contents("input");

    let input = contents
        .trim()
        .split("\n")
        .map(|x| {
            let spl = x.split(": ").collect::<Vec<&str>>();
            let policy: Policy = spl[0].parse().unwrap();
            let password = spl[1];
            let mut counts: HashMap<char, u32> = HashMap::new();
            for c in password.chars() {
                let counter = counts.entry(c).or_insert(0);
                *counter += 1;
            }
            (policy, password, counts)
        })
        .collect::<Vec<(Policy, &str, HashMap<char, u32>)>>();

    // part 1
    let mut num_valid = 0;

    for (policy, _, counts) in &input {
        match counts.get(&policy.letter) {
            Some(count) => {
                if policy.min <= *count && *count <= policy.max {
                    num_valid += 1;
                }
            }
            None => (),
        }
    }

    dbg!(num_valid);

    // part 2
    let mut num_valid = 0;
    for (policy, password, _) in &input {
        let chars = [
            password.chars().nth((policy.min - 1) as usize).unwrap(),
            password.chars().nth((policy.max - 1) as usize).unwrap(),
        ];
        if (chars[0] == policy.letter) != (chars[1] == policy.letter) {
            num_valid += 1;
        }
    }

    dbg!(num_valid);

    Ok(())
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}
