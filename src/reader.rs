pub(crate) struct Reader {
    content: String,
    pos: usize,
}

impl Reader {
    pub fn new(content: &str) -> Reader {
        Reader {
            content: content.to_string(),
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.content.chars().skip(self.pos).take(1).next()
    }

    pub fn peek_n(&self, n: usize) -> Vec<char> {
        self.content.chars().skip(self.pos).take(n).collect()
    }

    pub fn consume(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        c
    }

    pub fn consume_n(&mut self, n: usize) -> Vec<char> {
        let chars = self.peek_n(n);
        self.pos += n;
        chars
    }

    pub fn is_eof(&self) -> bool {
        self.pos == self.content.chars().count()
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
        assert_eq!(reader.peek(), Some('h'));
    }

    #[test]
    fn test_peek_n() {
        let content = "hello world!";
        let reader = Reader::new(content);
        let expected = vec!['h', 'e', 'l', 'l', 'o', ' '];
        assert_eq!(reader.peek_n(6), expected);
    }

    #[test]
    fn test_consume() {
        let content = "hello world!";
        let mut reader = Reader::new(content);
        assert_eq!(reader.consume(), Some('h'));
        assert_eq!(reader.pos, 1);
    }

    #[test]
    fn test_consume_n() {
        let content = "hello world!";
        let mut reader = Reader::new(content);
        let expected = vec!['h', 'e', 'l', 'l', 'o', ' '];
        assert_eq!(reader.consume_n(6), expected);
        assert_eq!(reader.pos, 6);
    }

    #[test]
    fn test_is_eof() {
        let content = "hello world!";
        let mut reader = Reader::new(content);
        assert!(!reader.is_eof());
        reader.consume_n(12);
        assert!(reader.is_eof());
    }
}
