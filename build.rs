use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

fn write_words(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed={src}");

    let words = {
        let mut fin = File::open(src)?;
        let mut words: String = String::new();
        fin.read_to_string(&mut words)?;
        words
    };

    let mut path = PathBuf::from(env::var("OUT_DIR")?);
    path.push(dest);
    let mut fout = File::create(&path)?;

    write!(fout, "[")?;
    for word in words.lines() {
        writeln!(fout, "\"{word}\",")?;
    }
    write!(fout, "]")?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    write_words(
        "google-words/google-10000-english-usa-no-swears-short.txt",
        "short.in.rs",
    )?;
    write_words(
        "google-words/google-10000-english-usa-no-swears-medium.txt",
        "medium.in.rs",
    )?;
    write_words(
        "google-words/google-10000-english-usa-no-swears-long.txt",
        "long.in.rs",
    )?;
    Ok(())
}
