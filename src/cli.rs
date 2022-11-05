// file: cli.rs

use clap::Parser;

///
/// xkcd.com/936 derived password generator. Randomly select words from a
/// memorable word list to generate long but easy to remember passwords.
///
#[derive(clap::Parser, Debug, Default)]
#[clap()]
pub struct Cli {
    /// Select words from the "long" list of words between 9 and 13 characters in length. Can be
    /// combined with `--medium` and `--short`.
    #[clap(short, long)]
    pub long: bool,
    /// Select words from the "medium" list of words between 5 and 8 characters in length. Can be
    /// combined with `--long` and `--short`.
    #[clap(short, long)]
    pub medium: bool,
    /// Select words from the "short" list of words between 1 and 4 characters in length. Can be
    /// combined with `--long` and '--medium'.
    #[clap(short, long)]
    pub short: bool,
    /// Create strings of `COUNT` number of words.
    #[clap(short, long, default_value_t=4)]
    pub count: u8,
    /// Create `ITERATIONS` different strings.
    #[clap(short, long, default_value_t=1)]
    pub iterations: u64,
} // end struct Cli

pub fn parse() -> Cli {
    Cli::parse()
} // end fn parse()

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::Parser;

    #[test]
    fn test_parse_none() {
        let args: Vec<String> = vec![];
        let _cli = Cli::try_parse_from(&args).unwrap();
    } // end test_parse_none

    #[test]
    fn test_parse_long() {
        for long in ["--long", "-l"] {
            let args: Vec<String> = vec![String::from("ch"), String::from(long)];
            let cli = Cli::try_parse_from(&args).unwrap();
            assert_eq!(true, cli.long);
        }
    } // end test_parse_long

    #[test]
    fn test_parse_medium() {
        for long in ["--medium", "-m"] {
            let args: Vec<String> = vec![String::from("ch"), String::from(long)];
            let cli = Cli::try_parse_from(&args).unwrap();
            assert_eq!(true, cli.medium);
        }
    } // end test_parse_medium
} // end mod tests

// cli.rs
