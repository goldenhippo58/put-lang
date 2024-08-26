#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let source = "var x = 42 + 5;";
        let lexer = Lexer::new(source);
        let tokens = lexer.tokenize();

        let parser = Parser::new(lexer);
        let program = parser.parse();

        // Add assertions to check that the parsing works as expected
    }
}
