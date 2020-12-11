use anyhow::Result;

use std::io::Read;

struct Arena(Vec<Vec<Option<i32>>>);

impl Arena {
    fn step(&mut self) {
        let grid = &mut self.0;
        let (nrow, ncol): (i32, i32) = (grid.len() as i32, grid[0].len() as i32);
        let grid_indices = (0..nrow)
            .flat_map(|i| (0..ncol).map(move |j| (i, j)))
            .collect::<Vec<(i32, i32)>>();
        let counts: Vec<i32> = grid_indices
            .iter()
            .map(|&(i, j)| {
                let mut iters: Vec<Box<dyn std::iter::Iterator<Item = (i32, i32)>>> = vec![
                    Box::new((0..i).rev().zip(std::iter::repeat(j))),
                    Box::new((i + 1..ncol).zip(std::iter::repeat(j))),
                    Box::new(std::iter::repeat(i).zip(j + 1..nrow)),
                    Box::new(std::iter::repeat(i).zip((0..j).rev())),
                    Box::new((i + 1..ncol).zip(j + 1..nrow)),
                    Box::new((i + 1..ncol).zip((0..j).rev())),
                    Box::new((0..i).rev().zip(j + 1..nrow)),
                    Box::new((0..i).rev().zip((0..j).rev())),
                ];
                iters
                    .iter_mut()
                    .map(
                        |iter| match iter.find_map(|(ii, jj)| grid[ii as usize][jj as usize]) {
                            Some(c) => c,
                            None => 0,
                        },
                    )
                    .sum()
            })
            .collect();
        grid_indices
            .into_iter()
            .zip(counts.into_iter())
            .for_each(|((i, j), ac)| match grid[i as usize][j as usize] {
                Some(c) => {
                    if c == 0 && ac == 0 {
                        grid[i as usize][j as usize] = Some(1);
                    }
                    if c == 1 && ac > 4 {
                        grid[i as usize][j as usize] = Some(0);
                    }
                }
                None => (),
            });
    }

    fn sum(&self) -> i32 {
        self.0
            .iter()
            .map(|r| {
                r.iter()
                    .filter(|c| c.is_some())
                    .map(|p| p.unwrap())
                    .sum::<i32>()
            })
            .sum()
    }
}

impl std::fmt::Display for Arena {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|l| l
                    .iter()
                    .map(|c| match c {
                        Some(c) => match c {
                            0 => 'L',
                            1 => '#',
                            _ => panic!(),
                        },
                        None => '.',
                    })
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn main() -> Result<()> {
    let contents = get_contents("input")?;
    let contents = contents.trim();

    let mut grid: Arena = Arena(
        contents
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'L' => Some(0),
                        '.' => None,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect(),
    );

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", grid);
        let last_step = grid.0.clone();
        grid.step();

        if last_step == grid.0 {
            dbg!(grid.sum());
            break;
        }
    }

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = std::fs::File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
