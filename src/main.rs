mod lox;
use crate::lox::Lox;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        let result = Lox::new().run_file(&args[1]);
        if let Err(e) = result {
            println!("{e}");
        }
    } else {
        Lox::new().run_repl().unwrap();
    }
}
