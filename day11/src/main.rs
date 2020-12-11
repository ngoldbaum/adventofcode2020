use anyhow::Result;

use std::io::Read;

struct Arena(Vec<Vec<Option<usize>>>);

impl Arena {
    fn step(&mut self) {
        let grid = &mut self.0;
        let (nrow, ncol): (usize, usize) = (grid.len(), grid[0].len());
        let grid_indices = (0..nrow)
            .flat_map(|i| (0..ncol).map(move |j| (i, j)))
            .collect::<Vec<(usize, usize)>>();
        let counts: Vec<usize> = grid_indices
            .iter()
            .map(|&(i, j)| {
                let mut iters: Vec<Box<dyn std::iter::Iterator<Item = (usize, usize)>>> = vec![
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
                    .map(|iter| match iter.find_map(|(ii, jj)| grid[ii][jj]) {
                        Some(c) => c,
                        None => 0,
                    })
                    .sum()
            })
            .collect();
        grid_indices
            .into_iter()
            .zip(counts.into_iter())
            .for_each(|((i, j), ac)| match grid[i][j] {
                Some(c) => {
                    if c == 0 && ac == 0 {
                        grid[i][j] = Some(1);
                    }
                    if c == 1 && ac > 4 {
                        grid[i][j] = Some(0);
                    }
                }
                None => (),
            });
    }

    fn sum(&self) -> usize {
        self.0
            .iter()
            .flat_map(|r| {
                r.iter().map(|&p| match p {
                    Some(c) => c,
                    None => 0,
                })
            })
            .sum()
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
