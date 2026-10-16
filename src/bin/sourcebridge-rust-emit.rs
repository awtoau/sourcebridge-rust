use std::{env, fs, process};

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: sourcebridge-rust-emit SOURCE_FACTS_JSON");
        process::exit(2);
    };
    let input = fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("{path}: {error}");
        process::exit(1);
    });
    match sourcebridge_rust::emit_scalar_additions(&input) {
        Ok(output) => print!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}
