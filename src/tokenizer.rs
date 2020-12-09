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

pub(crate) struct Tokenizer {
    keyword_dictionary: HashMap<&'static str, TokenKind>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        let mut keyword_dictionary = HashMap::new();
        keyword_dictionary.insert(keyword_consts::TRUE_KEYWORD, TokenKind::BOOL);
        keyword_dictionary.insert(keyword_consts::FALSE_KEYWORD, TokenKind::BOOL);
        keyword_dictionary.insert(keyword_consts::OR_KEYWORD, TokenKind::RELATION);
        keyword_dictionary.insert(keyword_consts::AND_KEYWORD, TokenKind::RELATION);

        Tokenizer { keyword_dictionary }
    }

    pub fn convert_word_to_token(&self, word: &str) -> Result<Token, &'static str> {
        let value = word.to_string();
        let kind = self.get_word_token_kind(word)?;

        Ok(Token { value, kind })
    }

    fn get_word_token_kind(&self, word: &str) -> Result<TokenKind, &'static str> {
        if let Some(&token_kind) = self.keyword_dictionary.get(word) {
            Ok(token_kind)
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
        let tokenizer = Tokenizer::new();
        let token = tokenizer.convert_word_to_token(true_word).unwrap();
        assert_eq!(token.kind, TokenKind::BOOL);
        assert_eq!(token.value, true_word.to_string());
    }

    #[test]
    fn test_false_keyword_tokenization() {
        let false_word = keyword_consts::FALSE_KEYWORD;
        let tokenizer = Tokenizer::new();
        let token = tokenizer.convert_word_to_token(false_word).unwrap();
        assert_eq!(token.kind, TokenKind::BOOL);
        assert_eq!(token.value, false_word.to_string());
    }

    #[test]
    fn test_or_keyword_tokenization() {
        let or_word = keyword_consts::OR_KEYWORD;
        let tokenizer = Tokenizer::new();
        let token = tokenizer.convert_word_to_token(or_word).unwrap();
        assert_eq!(token.kind, TokenKind::RELATION);
        assert_eq!(token.value, or_word.to_string());
    }

    #[test]
    fn test_and_keyword_tokenization() {
        let and_word = keyword_consts::AND_KEYWORD;
        let tokenizer = Tokenizer::new();
        let token = tokenizer.convert_word_to_token(and_word).unwrap();
        assert_eq!(token.kind, TokenKind::RELATION);
        assert_eq!(token.value, and_word.to_string());
    }
}
