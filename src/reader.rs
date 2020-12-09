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
}
