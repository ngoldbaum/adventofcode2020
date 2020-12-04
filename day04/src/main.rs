use anyhow::{bail, Error, Result};
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGB {
    type Err = Error;

    // Parses a color hex code of the form '#rRgGbB..' into an
    // instance of 'RGB'
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        if hex_code.starts_with('#') && hex_code.len() == 7 {
            let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
            let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
            let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

            Ok(RGB { r, g, b })
        } else {
            bail!("")
        }
    }
}

#[derive(Debug)]
struct Passport {
    fields: Vec<PassportField>,
}

#[derive(Debug)]
enum PassportField {
    BirthYear(u64),
    IssueYear(u64),
    ExpirationYear(u64),
    Height(u64),
    HairColor(RGB),
    EyeColor(String),
    PassportID(u64),
    CountryID(u64),
}

impl FromStr for PassportField {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let v: Vec<&str> = s.split(":").collect();
        let (field_name, content) = (v[0], v[1]);
        match field_name {
            "byr" => {
                let byr: u64 = content.parse()?;
                if byr >= 1920 && byr <= 2002 {
                    Ok(PassportField::BirthYear(byr))
                } else {
                    bail!("");
                }
            }
            "iyr" => {
                let iyr: u64 = content.parse()?;
                if iyr >= 2010 && iyr <= 2020 {
                    Ok(PassportField::IssueYear(iyr))
                } else {
                    bail!("");
                }
            }
            "eyr" => {
                let eyr: u64 = content.parse()?;
                if eyr >= 2020 && eyr <= 2030 {
                    Ok(PassportField::ExpirationYear(eyr))
                } else {
                    bail!("");
                }
            }
            "hgt" => {
                let hgt = content.to_string();
                if hgt.ends_with("in") {
                    let hgt: u64 = hgt.strip_suffix("in").unwrap().parse()?;
                    if hgt >= 59 && hgt <= 76 {
                        return Ok(PassportField::Height(hgt));
                    }
                } else if hgt.ends_with("cm") {
                    let hgt: u64 = hgt.strip_suffix("cm").unwrap().parse()?;
                    if hgt >= 150 && hgt <= 193 {
                        return Ok(PassportField::Height(hgt));
                    }
                }
                bail!("");
            }
            "hcl" => Ok(PassportField::HairColor(content.parse()?)),
            "ecl" => {
                let ecl = content.to_string();
                match ecl.as_str() {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                        Ok(PassportField::EyeColor(ecl))
                    }
                    _ => bail!(""),
                }
            }
            "pid" => {
                if content.len() == 9 {
                    return Ok(PassportField::PassportID(content.parse()?));
                } else {
                    bail!("");
                }
            }
            "cid" => Ok(PassportField::CountryID(content.parse()?)),
            _ => panic!(),
        }
    }
}

impl Passport {
    fn validate(&self) -> bool {
        let num_valid_fields: usize = self
            .fields
            .iter()
            .map(|f| match f {
                PassportField::CountryID(_) => 0,
                _ => 1,
            })
            .sum();
        if num_valid_fields == 7 {
            true
        } else {
            false
        }
    }
}

fn main() -> Result<()> {
    let contents = get_contents("input")?;

    let passports: Vec<Passport> = contents
        .split("\n\n")
        .map(|p| Passport {
            fields: p
                .split_whitespace()
                .filter_map(|pf| pf.parse().ok())
                .collect::<Vec<PassportField>>(),
        })
        .collect();

    let num_valid: usize = passports
        .iter()
        .map(|p| match p.validate() {
            true => 1,
            false => 0,
        })
        .sum();

    dbg!(num_valid);

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = std::fs::File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
