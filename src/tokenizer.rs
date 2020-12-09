use crate::keyword_consts;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum TokenKind {
    BOOL,
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Token {
    pub value: String,
    pub kind: TokenKind,
}

pub(crate) struct Tokenizer;

impl Tokenizer {
    pub fn convert_word_to_token(&self, word: &str) -> Result<Token, &'static str> {
        if word == keyword_consts::TRUE_KEYWORD || word == keyword_consts::FALSE_KEYWORD {
            let token = Token {
                value: word.to_string(),
                kind: TokenKind::BOOL,
            };
            Ok(token)
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
        let tokenizer = Tokenizer;
        let token = tokenizer.convert_word_to_token(true_word).unwrap();
        assert_eq!(token.kind, TokenKind::BOOL);
        assert_eq!(token.value, true_word.to_string());
    }

    #[test]
    fn test_false_keyword_tokenization() {
        let false_word = keyword_consts::FALSE_KEYWORD;
        let tokenizer = Tokenizer;
        let token = tokenizer.convert_word_to_token(false_word).unwrap();
        assert_eq!(token.kind, TokenKind::BOOL);
        assert_eq!(token.value, false_word.to_string());
    }
}
