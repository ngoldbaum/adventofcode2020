use std::io::Read;
use std::str::FromStr;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
struct Seat {
    row: usize,
    col: usize,
    seat_id: usize,
}

impl FromStr for Seat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Seat> {
        let boarding_pass = s
            .to_string()
            .replace(&['F', 'L'][..], "0")
            .replace(&['B', 'R'][..], "1");
        let seat_id = usize::from_str_radix(&boarding_pass, 2)?;
        Ok(Seat {
            row: seat_id / 8,
            col: seat_id % 8,
            seat_id: seat_id,
        })
    }
}

fn main() -> Result<()> {
    let contents = get_contents("input")?;

    let mut seats: Vec<Seat> = contents.lines().map(|x| x.parse().unwrap()).collect();

    // part 1
    let max_id = seats.iter().fold(0, |acc, x| std::cmp::max(acc, x.seat_id));

    dbg!(max_id);

    // part 2
    seats.sort_by(|a, b| a.seat_id.partial_cmp(&b.seat_id).unwrap());

    let open_seat = seats
        .windows(2)
        .filter(|x| x[0].seat_id + 1 != x[1].seat_id)
        .next()
        .unwrap()[0]
        .seat_id
        + 1;

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
        assert!(
            "BFFFBBFRRR".parse::<Seat>().unwrap()
                == Seat {
                    row: 70,
                    col: 7,
                    seat_id: 567,
                }
        );
        assert!(
            "FFFBBBFRRR".parse::<Seat>().unwrap()
                == Seat {
                    row: 14,
                    col: 7,
                    seat_id: 119,
                }
        );
        assert!(
            "BBFFBBFRLL".parse::<Seat>().unwrap()
                == Seat {
                    row: 102,
                    col: 4,
                    seat_id: 820,
                }
        );
    }
}
