use codespan_reporting::diagnostic::Severity;
pub use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFile;
use codespan_reporting::term::{
    emit,
    termcolor::{Color, ColorChoice, ColorSpec, StandardStream},
    Chars,
    Config,
    Styles,
};

/// A context used for keeping track of diagnostics with Flycatcher.  This struct also provides
/// functionality for emitting such diagnostics to the terminal, as well as configuring how they look
/// via the `codespan-reporting` crate.
#[derive(Clone)]
pub struct Context<'a> {
    /// The configuration structure provided by `codespan-reporting` that customizes how diagnostic
    /// messages may be displayed in the console.  By default, it's configured to a style similar to
    /// Rustc's diagnostic messages.
    pub config: Config,

    /// The name of the file that diagnostic messages use, to help locate where an error occurred.
    /// This path should be relative to the current working directory.
    pub filename: &'a str,

    /// The source of the file, used by diagnostic messages to display short snippets of code that
    /// help locate the source of an error.
    pub source: &'a str,

    /// A list of diagnostics emitted in the context.
    pub diagnostics: Vec<Diagnostic<()>>,
}

impl<'a> Context<'a> {
    /// Creates a new context for the file name and source provided.  It produces a default
    /// `codespan-reporting` configuration that looks similar to Rustc's.
    pub fn new(filename: &'a str, source: &'a str) -> Self {
        // We must make a configuration and use Flycatcher's defaults.

        let mut config = Config::default();
        //               ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑ This is the default configuration that we will modify to look
        //                                 more like Rustc diagnostics.

        config.chars = Chars::ascii();
        //             ↑↑↑↑↑↑↑↑↑↑↑↑↑↑ By default, we want to use the ASCII character set in diagnostics,
        //                            for the sole purpose of system support.  (Oh, and I think it looks
        //                            nicer)

        let mut styles = Styles::default();
        //               ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑ These are the default styles provided by the
        //                                 `codespan-reporting` crate, which we will customize below to
        //                                 look more like Rustc style diagnostics.

        {
            // The `ColorSpec` is a single field in a `Styles` object that allows more precise
            // customization of colors.  We will use that to get the style that we want.
            //                 ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
            let mut red_spec = ColorSpec::new();
            // Like the name suggests, this will be used as the default red color in Flycatcher
            // diagnostics.

            red_spec.set_bold(true); // All main colors should be bold.
            red_spec.set_fg(Some(Color::Red)); // The foreground color is, of course, red.
            red_spec.set_intense(true); // This makes the diagnostic brighter and better with a lot of
                                        // terminal themes.  At least, on VSCode.

            //     ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓ The color of the primary label(s) for bug diagnostics.
            styles.primary_label_bug = red_spec.clone();

            //     ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓ The color of primary label(s) for error diagnostics.
            styles.primary_label_error = red_spec.clone();
        }

        {
            // Now we can get to the different colors in the diagnostic styles, which is mostly cyan.
            let mut cyan_spec = ColorSpec::new();

            cyan_spec.set_bold(true);
            cyan_spec.set_fg(Some(Color::Cyan)); // TODO: test if this causes any issues on non-windows
                                                 // systems.
            cyan_spec.set_intense(true);

            //     ↓↓↓↓↓↓↓↓↓↓↓ The color of the `note:` part of note diagnostics.
            styles.header_note = cyan_spec.clone();

            // This one is self explanatory, it's the line numbers before the `source_border` character.
            styles.line_number = cyan_spec.clone();

            //     ↓↓↓↓↓↓↓↓↓↓↓ The `=` bullet before a note at the end of a diagnostic message.
            styles.note_bullet = cyan_spec.clone();

            //     ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓ the color of primary labels for note diagnostics.
            styles.primary_label_note = cyan_spec.clone();

            //     ↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓ the color of primary labels for help diagnostics.
            styles.primary_label_help = cyan_spec.clone();

            styles.secondary_label = cyan_spec.clone();

            // note: Cyan is known as Blue on non-windows devices, according to `codespan-reporting`'s
            // documentation:
            // https://docs.rs/codespan-reporting/0.11.1/codespan_reporting/term/struct.Styles.html
            // I'm not sure if this will cause any issues with Linux systems.

            // The "source border" is the border between the line numbers and the sample snippet, if
            // any.
            //     ↓↓↓↓↓↓↓↓↓↓↓↓↓
            styles.source_border = cyan_spec.clone();
        }

        {
            // Yellow colors.
            let mut yellow_spec = ColorSpec::new();

            yellow_spec.set_bold(true);
            yellow_spec.set_fg(Some(Color::Yellow));
            yellow_spec.set_intense(true);

            styles.primary_label_warning = yellow_spec.clone();
        }

        config.styles = styles;

        Self {
            config,
            filename,
            source,
            diagnostics: vec![],
        }
    }

    /// Emits a diagnostic that were emitted to this context.
    pub fn emit_diagnostic(&self, diagnostic: Diagnostic<()>) {
        // This file is used by `codespan-reporting` to get the name of the file that the diagnostic is
        // for and to find snippets in the file.
        let file = SimpleFile::new(self.filename.to_string(), self.source);

        // We should emit diagnostics to the correct streams.  For example, error diagnostics should be
        // emitted to `stderr`, while note diagnostics should be emitted to `stdout`.
        let mut stream = match diagnostic.severity {
            Severity::Bug | Severity::Error => StandardStream::stderr(ColorChoice::Auto),
            _ => StandardStream::stdout(ColorChoice::Auto),
        };

        // And emit the stream to the console.
        emit(&mut stream, &self.config, &file, &diagnostic).unwrap();
    }

    /// Emits all diagnostics to the console.
    pub fn emit(&self) {
        for diagnostic in &self.diagnostics {
            self.emit_diagnostic(diagnostic.clone());
        }
    }

    /// Emits all diagnostics to the console and flushes (clears) the list of diagnostics.
    pub fn flush(&mut self) {
        self.emit();
        self.diagnostics.clear();
    }
}
