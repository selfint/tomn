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
        if word == keyword_consts::TRUE_KEYWORD {
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
    fn test_token_identification() {
        let true_word = "true";
        let tokenizer = Tokenizer;
        let token = tokenizer.convert_word_to_token(true_word).unwrap();
        assert_eq!(token.kind, TokenKind::BOOL);
        assert_eq!(token.value, true_word.to_string());
    }
}
