extern crate flycatcher_diagnostic;

use flycatcher_diagnostic::{
    codespan_reporting::diagnostic::{
        Diagnostic,
        Label,
        LabelStyle,
    },
    DiagnosticEmitter
};

fn main() {
    // Create the file for diagnostics.
    let source = "Hello, world!";

    let label = Label::new(LabelStyle::Primary, (), 0..5)
        .with_message("This token is unliked.");

    let label2 = Label::new(LabelStyle::Secondary, (), 0..5)
        .with_message("This is an example.");

    let label3 = Label::new(LabelStyle::Primary, (), 7..12)
        .with_message("Of course, I am mad at this one too.");
    
    let d = Diagnostic::error()
        .with_message("An invalid token was found.".to_string())
        .with_labels(vec![label, label2, label3])
        .with_notes(vec![
            "This is a note.".into(),
            "These can provide useful info.".into(),
            "Or not!\nAlso, notes may be multiline!".into()
        ]);
    
    let emitter = DiagnosticEmitter::new("@anonymous", source);
    emitter.emit_diagnostic(d);
}