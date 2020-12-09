#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub(crate) enum TokenKind {
    BOOL,
    RELATION,
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Token {
    pub value: String,
    pub kind: TokenKind,
}

pub(crate) struct Lexer;

impl Lexer {
    fn peek(&self) -> Token {
        unimplemented!()
    }

    fn peek_n(&self) -> Vec<Token> {
        unimplemented!()
    }

    fn consume(&mut self) -> Token {
        unimplemented!()
    }

    fn consume_n(&mut self, n: usize) -> Vec<Token> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {}
