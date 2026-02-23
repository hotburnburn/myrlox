use std::io::{BufRead, BufReader, Write, stdin};
use std::{fs, io};
use thiserror::Error;

pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        Lox {}
    }

    fn run(&self, code: String) -> Result<(), RuntimeError> {
        println!("{code}");
        Ok(())
    }

    pub fn run_file(&self, path: &str) -> Result<(), RuntimeError> {
        let file = fs::read_to_string(path).map_err(RuntimeError::InvalidPath)?;
        self.run(file)
    }

    pub fn run_repl(&self) -> Result<(), RuntimeError> {
        print!(">");
        io::stdout().flush().unwrap();

        let stdin = stdin();
        let reader = BufReader::new(stdin.lock());

        for line in reader.lines() {
            let line = line.unwrap();
            self.run(line)?;

            print!(">");
            io::stdout().flush().unwrap();
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("invalid path: {0}")]
    InvalidPath(#[from] std::io::Error),
}
