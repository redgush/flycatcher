use clap::{App, Arg};
use flycatcher_diagnostic::Context;
use flycatcher_parser::Parser;
use std::fs;
use std::path::Path;

fn main() {
    let matches = App::new("Flycatcher")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Zack Pace")
        .about("A command line interface for the Flycatcher compiler.")
        .arg(
            Arg::with_name("input")
                .help("The input file to compile.")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(i) = matches.value_of("input") {
        let path = Path::new(i);
        if path.exists() {
            let s = fs::read_to_string(i).unwrap();
            let ctx = Context::new(i, &s);

            {
                let mut tmp_ctx = ctx.clone();
                let mut parser = Parser::new(&mut tmp_ctx);

                let start = std::time::Instant::now();
                let ast = parser.parse();
                let end = start.elapsed().as_nanos();

                dbg!(ast);
                parser.context.emit();

                println!("Parsed in {}ms", end as f64 / 1e+6)
            }
        } else {
            println!("Error: provided input file doesn't exist.");
            std::process::exit(1);
        }
    }
    // let s = std::fs::read_to_string("test.flyc").unwrap();
    // let ctx = Context::new("./test.flyc", &s);
    //
    // {
    // let mut tmp_ctx = ctx.clone();
    // let mut parser = Parser::new(&mut tmp_ctx);
    //
    // let start = std::time::Instant::now();
    // let ast = parser.parse();
    // let end = start.elapsed().as_nanos();
    //
    // dbg!(ast);
    // parser.context.emit();
    //
    // println!("Parsed in {}ms", end as f64 / 1e+6)
    // }
}
