//! A crate for emitting diagnostic messages emitted by Flycatcher to the terminal.

use codespan_reporting::diagnostic::{Diagnostic, Severity};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::{
    Chars,
    Config,
    emit,
    Styles,
    termcolor::{
        Color,
        ColorChoice,
        ColorSpec,
        StandardStream
    }
};

pub use codespan_reporting;

/// A tool for emitting diagnostic messages to the terminal.
pub struct DiagnosticEmitter<'a> {

    /// The configuration used for displaying diagnostic messages.  The default is similar to
    /// the way RustC displays diagnostic messages.
    pub config: Config,

    /// The name of the file that the diagnostic messages should say.  If you don't know what to
    /// use here, just use `@anonymous`.
    pub filename: &'a str,

    /// The source of the file, used by diagnostic messages to provide a snippet of the
    /// offending code.
    pub source: &'a str,

}

impl<'a> DiagnosticEmitter<'a> {

    /// Allocates a new DiagnosticEmitter that will emit diagnostic messages with the file path
    /// provided.
    /// 
    /// # Arguments
    /// - `filename`: This is the filename that will be displayed in the diagnostic messages
    /// when they are emitted to the console.
    /// - `source`: The contents of the file that the diagnostics are for.
    pub fn new(filename: &'a str, source: &'a str) -> Self {
        // Use only ASCII characters, similar to Rustc's diagnostic messages.
        let mut config = Config::default();
        config.chars = Chars::ascii();

        let mut styles = Styles::default();

        {
            // Set the primary label bug to be Red intense, similar to Rustc's diagnostics.
            let mut style = ColorSpec::new();
            style.set_bold(true);
            style.set_fg(Some(Color::Red));
            style.set_intense(true);
            styles.primary_label_bug = style;
        }

        {
            // Set the primary label error to be Red intense, similar to Rustc's diagnostics.
            let mut style = ColorSpec::new();
            style.set_bold(true);
            style.set_fg(Some(Color::Red));
            style.set_intense(true);
            styles.primary_label_error = style;
        }

        {
            // Set the source border to be Cyan intense, similar to Rustc's diagnostics.
            let mut style = ColorSpec::new();
            style.set_bold(true);
            style.set_fg(Some(Color::Cyan));
            style.set_intense(true);
            styles.source_border = style;
        }

        {
            // Set the note bullet to be Cyan intense, similar to Rustc's diagnostics.
            let mut style = ColorSpec::new();
            style.set_bold(true);
            style.set_fg(Some(Color::Cyan));
            style.set_intense(true);
            styles.note_bullet = style;
        }

        {
            // Set the line numbers to be Cyan intense, similar to Rustc's diagnostics.
            let mut style = ColorSpec::new();
            style.set_bold(true);
            style.set_fg(Some(Color::Cyan));
            style.set_intense(true);
            styles.line_number = style;
        }

        {
            // Set the warning color to be Yellow intense, similar to Rustc's diagnostics.
            let mut style = ColorSpec::new();
            style.set_bold(true);
            style.set_fg(Some(Color::Yellow));
            style.set_intense(true);
            styles.primary_label_warning = style;
        }

        {
            // Set the note header color to be Cyan intense, similar to Rustc's diagnostics.
            let mut style = ColorSpec::new();
            style.set_bold(true);
            style.set_fg(Some(Color::Cyan));
            style.set_intense(true);
            styles.header_note = style;
        }

        {
            // Set the note label color to be Cyan intense, similar to Rustc's diagnostics.
            let mut style = ColorSpec::new();
            style.set_bold(true);
            style.set_fg(Some(Color::Cyan));
            style.set_intense(true);
            styles.primary_label_note = style;
        }

        {
            // Set the secondary label color to be Cyan intense, similar to Rustc's diagnostics.
            let mut style = ColorSpec::new();
            style.set_bold(true);
            style.set_fg(Some(Color::Cyan));
            style.set_intense(true);
            styles.secondary_label = style;
        }
        
        // Use the styles defined above.
        config.styles = styles;

        Self {
            config,
            filename,
            source
        }
    }

    /// Emits a single diagnostic message to the console.
    /// 
    /// # Arguments
    /// - `diagnostic`: The diagnostic message to display.
    pub fn emit_diagnostic(&self, diagnostic: Diagnostic<()>) {
        // Create the `codespan_reporting` file, which contains the information about the source
        // used in the diagnostic.  This is used to get the filename and source for the
        // diagnostic previews.
        let simple_file = SimpleFile::new(self.filename.to_string(), self.source);

        // Use `stderr` for bugs and errors and `stdout` for warnings, notes and help
        // diagnostics.
        let mut stream = match diagnostic.severity {
            Severity::Bug |
            Severity::Error => StandardStream::stderr(ColorChoice::Auto),
            _ => StandardStream::stdout(ColorChoice::Auto)
        };

        // Check if an error occurs while simultaniously emitting the diagnostic to the console.
        if let Err(e) = emit(&mut stream, &self.config, &simple_file, &diagnostic) {
            panic!("{}", e);
        }
    }

    /// Emits a list of diagnostics to the terminal.
    /// 
    /// # Arguments
    /// - `diagnostics`: The vector of diagnostics to log to the console.
    pub fn emit(&self, diagnostics: Vec<Diagnostic<()>>) {
        for diag in diagnostics {
            self.emit_diagnostic(diag);
        }
    }

}