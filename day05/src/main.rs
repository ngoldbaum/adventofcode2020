use std::io::Read;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, PartialOrd)]
struct Seat(usize);

impl FromStr for Seat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Seat> {
        let boarding_pass = s
            .to_string()
            .replace(&['F', 'L'][..], "0")
            .replace(&['B', 'R'][..], "1");
        let seat_id = usize::from_str_radix(&boarding_pass, 2)?;
        Ok(Seat(seat_id))
    }
}

fn main() -> Result<()> {
    let contents = get_contents("input")?;

    let mut seats: Vec<Seat> = contents.lines().map(|x| x.parse().unwrap()).collect();

    // part 1
    let max_id = seats.iter().fold(0, |acc, Seat(x)| std::cmp::max(acc, *x));

    dbg!(max_id);

    // part 2
    seats.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let Seat(open_seat) = seats
        .windows(2)
        .filter(|x| x[0].0 + 1 != x[1].0)
        .next()
        .unwrap()[0];
    let open_seat = open_seat + 1;

    dbg!(open_seat);

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = std::fs::File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_col() {
        assert!("BFFFBBFRRR".parse::<Seat>().unwrap() == Seat(567));
        assert!("FFFBBBFRRR".parse::<Seat>().unwrap() == Seat(119));
        assert!("BBFFBBFRLL".parse::<Seat>().unwrap() == Seat(820));
    }
}
