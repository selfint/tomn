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

impl Lexer {}

#[cfg(test)]
mod tests {}
