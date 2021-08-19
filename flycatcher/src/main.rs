use clap::{App, Arg};
use flycatcherc::FlycatcherFrontend;
use flycatcherc_clif::{CraneliftBackend, Triple, target_lexicon::Environment};
use flycatcherc_link::{link, LinkerOptions};
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

                    let mut triple = Triple::host();

                    triple.environment = Environment::Gnu;

                    let mut backend = CraneliftBackend::new(
                        triple,
                        path.to_string() + ".o"
                    );
                    
                    if backend.compile(c) {
                        // Link object file since no error occurred.
                        if link(
                            vec![
                                path.to_string() + ".o"
                            ],
                            LinkerOptions {
                                output_path: Some(path.to_string() + std::env::consts::EXE_SUFFIX)
                            }
                        ) {
                            // Clean up the extra files
                            std::fs::remove_file(path.to_string() + ".o").unwrap();
                        }
                    }
                }
            },
            Err(_) => {
                let emitter = DiagnosticEmitter::new(input, &i);
                emitter.emit(p.diagnostics);
            }
        }
    }
}
