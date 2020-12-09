pub(crate) struct Reader {
    content: String,
    pos: u128,
}

impl Reader {
    fn new(content: &str) -> Reader {
        Reader {
            content: content.to_string(),
            pos: 0,
        }
    }

    fn peek(&self) -> String {
        let word: String = self.content.chars().take(1).collect();
        word
    }

    fn peek_n(&self, n: usize) -> String {
        let word: String = self.content.chars().take(n).collect();
        word
    }

    fn consume(&mut self) -> String {
        let word = self.peek();
        self.pos += 1;
        word
    }

    fn consume_n(&mut self, n: usize) -> String {
        let word = self.peek_n(n);
        self.pos += n as u128;
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader_constructor() {
        let content = "hello world!";
        let reader = Reader::new(content);
        assert_eq!(reader.content, content);
        assert_eq!(reader.pos, 0);
    }

    #[test]
    fn test_peek() {
        let content = "hello world!";
        let reader = Reader::new(content);
        assert_eq!(reader.peek(), "h");
    }

    #[test]
    fn test_peek_n() {
        let content = "hello world!";
        let reader = Reader::new(content);
        assert_eq!(reader.peek_n(6), "hello ");
    }

    #[test]
    fn test_consume() {
        let content = "hello world!";
        let mut reader = Reader::new(content);
        assert_eq!(reader.consume(), "h");
        assert_eq!(reader.pos, 1);
    }

    #[test]
    fn test_consume_n() {
        let content = "hello world!";
        let mut reader = Reader::new(content);
        assert_eq!(reader.consume_n(6), "hello ");
        assert_eq!(reader.pos, 6);
    }
}
