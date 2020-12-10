use crate::reader::Reader;
use trees::{tr, Tree};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub(crate) enum TokenKind {
    START,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) struct Token {
    pub value: String,
    pub kind: TokenKind,
}

pub(crate) struct Lexer {
    token_tree: Tree<Token>,
    pos: usize,
}

impl Lexer {
    fn new(mut reader: Reader) -> Lexer {
        let mut token_tree = Lexer::generate_token_tree(reader);
        let pos = 0;
        Lexer { token_tree, pos }
    }

    fn generate_token_tree(mut reader: Reader) -> Tree<Token> {
        let mut token_tree: Tree<Token> = tr(Token {
            value: "start".to_string(),
            kind: TokenKind::START,
        });
        token_tree
    }

    fn peek(&self) -> Token {
        self.token_tree.data.clone()
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
mod tests {
    use super::*;
    use crate::reader::Reader;

    const test_content: &str = "
prove p_and_tautology_is_p:
    arguments:
        p = bool
    claim:
        left:
            p and tautology
        right:
            p
        relation:
            equal
    proof:
        truth_table:
            p | p and tautology | p
            true | true | false
            false | false | false
";

    fn helper_get_lexer_on_content() -> Lexer {
        let content = test_content;
        let reader = Reader::new(content);

        Lexer::new(reader)
    }

    #[test]
    fn test_first_token() {
        let lexer = helper_get_lexer_on_content();
        let first_token = lexer.peek();
        assert_eq!(first_token.value, "start");
        assert_eq!(first_token.kind, TokenKind::START);
    }
}
