use flycatcher_diagnostic::Context;
use flycatcher_parser::Parser;

fn main() {
    let s = std::fs::read_to_string("test.flyc").unwrap();
    let ctx = Context::new("./test.flyc", &s);

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
}
