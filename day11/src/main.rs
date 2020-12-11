use anyhow::Result;

use std::io::Read;

struct Arena(Vec<Vec<Option<i32>>>);

impl Arena {
    fn step(&mut self) {
        let grid = &mut self.0;
        let (ncol, nrow): (i32, i32) = (grid.len() as i32, grid[0].len() as i32);
        let grid_indices = (0..ncol)
            .flat_map(|i| (0..nrow).map(move |j| (i, j)))
            .collect::<Vec<(i32, i32)>>();
        let offsets = (-1..2)
            .flat_map(|i| (-1..2).map(move |j| (i, j)))
            .filter(|&c| c != (0, 0))
            .collect::<Vec<(i32, i32)>>();
        let counts: Vec<i32> = grid_indices
            .iter()
            .map(|(i, j)| {
                offsets
                    .iter()
                    .map(|(di, dj)| (i + di, j + dj))
                    .filter(|c| {
                        grid_indices.contains(c) && !grid[c.0 as usize][c.1 as usize].is_none()
                    })
                    .map(|(ti, tj)| match grid[ti as usize][tj as usize] {
                        Some(c) => c,
                        None => 0,
                    })
                    .sum::<i32>()
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
                    if c == 1 && ac >= 4 {
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
