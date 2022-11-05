// file: main.rs

use correcthorse as ch;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = ch::cli::parse();

    let choices = ch::words::words(cli.short, cli.medium, cli.long);
    for _ in 0..cli.iterations {
        let words: String = ch::random::iter(&choices)
            .take(cli.count.into())
            .map(|x| String::from(*x))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{words}");
    }

    Ok(())
} // end fn main()

// main.rs
