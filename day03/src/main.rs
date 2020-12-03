use std::error;
use std::fs::File;
use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input")?;

    let lines = contents.trim().split("\n").collect::<Vec<&str>>();

    let nrows = lines.len();
    let ncols = lines[0].len();

    let mut treemap = vec![vec![0; ncols]; nrows];

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                treemap[i][j] = 0;
            } else {
                treemap[i][j] = 1;
            }
        }
    }

    let mut tree_prod = 1u64;

    for (nright, ndown) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut line = 0;
        let mut col = 0;
        let mut num_trees = 0;

        while line < nrows - 1 {
            line += ndown;
            col += nright;
            col = col % ncols;

            if treemap[line][col] == 1 {
                num_trees += 1;
            }
        }

        dbg!(num_trees);

        tree_prod *= num_trees
    }

    dbg!(tree_prod);

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
