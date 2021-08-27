use flycatcher_diagnostic::Context;
use flycatcher_parser::Parser;

fn main() {
    let s = std::fs::read_to_string("test.flyc").unwrap();
    let ctx = Context::new("./test.flyc", &s);

    {
        let mut tmp_ctx = ctx.clone();
        let mut parser = Parser::new(&mut tmp_ctx);
        dbg!(parser.parse_expression());
        parser.context.emit();
    }
}
