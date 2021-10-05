extern crate flycatcher;

use flycatcher::lexer::Lexer;

fn main() {
    // This test prints out all tokens in the lexer, which is initialized below.
    let mut lexer = Lexer::new("/// Hello, world!\n".to_string());

    loop {
        let item = lexer.next();

        if item == None {
            // If there is no token left in the lexer, then we must end the loop.
            break;
        }

        // Print the token type, starting and ending location, and the slice of the current token.
        // Currently, this may have some formatting issues with struct enum items.
        let loc = lexer.loc();
        println!("{:#?}@{}:{} '{}'", item, loc.start, loc.end, lexer.slice());
    }
}