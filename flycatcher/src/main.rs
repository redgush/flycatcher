use clap::{App, Arg};
use flycatcherc::FlycatcherFrontend;
use flycatcherc_clif::{CraneliftBackend, Triple};
use flycatcher_diagnostic::DiagnosticEmitter;
use flycatcher_parser::{Parser};
use std::fs::read_to_string;
use std::str::FromStr;

fn main() {
    let matches = App::new("Flycatcher")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Zack Pace")
        .about("Command line utility for Flycatcher source.")
        .arg(Arg::with_name("input")
            .help("The input file to compile.")
            .required(true)
            .index(1))
        .get_matches();
    
    if let Some(input) = matches.value_of("input") {
        let i = read_to_string(input).unwrap();

        let mut p = Parser::new(input, &i);
        match p.parse() {
            Ok(ast) => {
                //dbg!(ast);
                let mut c = FlycatcherFrontend::new(input, &i);
                c.convert(ast);

                //dbg!(c.hir);

                let emitter = DiagnosticEmitter::new(input, &i);
                emitter.emit(p.diagnostics);
                let emitter = DiagnosticEmitter::new(input, &i);
                emitter.emit(c.diagnostics.clone());

                if c.successful() {
                    // Compile into an object.
                    let path = std::path::Path::new(input).file_stem().unwrap().to_str().unwrap();
                    
                    let mut backend = CraneliftBackend::new(
                        Triple::from_str("x86_64-pc-unknown-gnu-coff").unwrap(),
                        path.to_string() + ".o"
                    );
                    backend.compile(c);
                }
            },
            Err(_) => {
                let emitter = DiagnosticEmitter::new(input, &i);
                emitter.emit(p.diagnostics);
            }
        }
    }
}
