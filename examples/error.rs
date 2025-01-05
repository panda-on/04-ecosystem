use anyhow::Context;
use std::{fs, mem::size_of};
use thiserror::Error;

fn main() -> Result<(), anyhow::Error> {
    println!("size of anyhow::Error: {}", size_of::<anyhow::Error>());
    println!("size of MyError: {}", size_of::<MyError>());
    println!("size of std::io::Error: {}", size_of::<std::io::Error>());
    println!(
        "size of ParseIntError: {}",
        size_of::<std::num::ParseIntError>()
    );
    println!(
        "size of serde_json::Error: {}",
        size_of::<serde_json::Error>()
    );
    println!("size of Strng: {}", size_of::<String>());

    let filename = "non-existent-file.txt";
    let _fd =
        fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

#[allow(unused)]
#[derive(Debug, Error)]
enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Serialize json error: {0}")]
    Serialize(#[from] serde_json::Error),

    #[error("Error: {0:?}")]
    BigError(Box<BigError>),

    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 64],
    d: u64,
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}
