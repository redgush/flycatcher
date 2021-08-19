use clap::{App, Arg};
use flycatcherc::FlycatcherFrontend;
use flycatcher_diagnostic::DiagnosticEmitter;
use flycatcher_parser::{Parser};
use std::fs::read_to_string;

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

                dbg!(c.hir);

                let emitter = DiagnosticEmitter::new(input, &i);
                emitter.emit(c.diagnostics);
            },
            Err(_) => {}
        }

        let emitter = DiagnosticEmitter::new(input, &i);
        emitter.emit(p.diagnostics);
    }
}
