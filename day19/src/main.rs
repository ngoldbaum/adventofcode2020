use std::io::Read;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let contents = get_contents("input.test")?;

    println!("{}", contents);

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = std::fs::File::open(filename)?;

    let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    Ok(contents)
}
