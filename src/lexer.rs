use crate::keyword_consts;
use std::collections::HashMap;

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

pub(crate) struct Lexer {
    keyword_dictionary: HashMap<&'static str, TokenKind>,
}

impl Lexer {
    pub fn new() -> Lexer {
        let mut keyword_dictionary = HashMap::new();
        keyword_dictionary.insert(keyword_consts::TRUE_KEYWORD, TokenKind::BOOL);
        keyword_dictionary.insert(keyword_consts::FALSE_KEYWORD, TokenKind::BOOL);
        keyword_dictionary.insert(keyword_consts::OR_KEYWORD, TokenKind::RELATION);
        keyword_dictionary.insert(keyword_consts::AND_KEYWORD, TokenKind::RELATION);

        Lexer { keyword_dictionary }
    }

    pub fn convert_word_to_token(&self, word: &str) -> Result<Token, &'static str> {
        let value = word.to_string();
        if let Some(&kind) = self.keyword_dictionary.get(word) {
            Ok(Token { value, kind })
        } else {
            Err("Failed to convert word to token")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_keyword_tokenization() {
        let true_word = keyword_consts::TRUE_KEYWORD;
        let tokenizer = Lexer::new();
        let token = tokenizer.convert_word_to_token(true_word).unwrap();
        assert_eq!(token.kind, TokenKind::BOOL);
        assert_eq!(token.value, true_word.to_string());
    }

    #[test]
    fn test_false_keyword_tokenization() {
        let false_word = keyword_consts::FALSE_KEYWORD;
        let tokenizer = Lexer::new();
        let token = tokenizer.convert_word_to_token(false_word).unwrap();
        assert_eq!(token.kind, TokenKind::BOOL);
        assert_eq!(token.value, false_word.to_string());
    }

    #[test]
    fn test_or_keyword_tokenization() {
        let or_word = keyword_consts::OR_KEYWORD;
        let tokenizer = Lexer::new();
        let token = tokenizer.convert_word_to_token(or_word).unwrap();
        assert_eq!(token.kind, TokenKind::RELATION);
        assert_eq!(token.value, or_word.to_string());
    }

    #[test]
    fn test_and_keyword_tokenization() {
        let and_word = keyword_consts::AND_KEYWORD;
        let tokenizer = Lexer::new();
        let token = tokenizer.convert_word_to_token(and_word).unwrap();
        assert_eq!(token.kind, TokenKind::RELATION);
        assert_eq!(token.value, and_word.to_string());
    }
}
