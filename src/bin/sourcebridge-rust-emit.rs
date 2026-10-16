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
    let output = serde_json::from_str::<serde_json::Value>(&input)
        .ok()
        .and_then(|document| {
            document
                .get("fact_schema")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .map_or_else(
            || sourcebridge_rust::emit_scalar_additions(&input),
            |schema| match schema.as_str() {
                "sourcebridge.c.ast-facts.v1" => sourcebridge_rust::emit_c_main_exit(&input),
                _ => sourcebridge_rust::emit_scalar_additions(&input),
            },
        );
    match output {
        Ok(output) => print!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}
