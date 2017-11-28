pub(crate) fn doc_parse<'a>(doc: &'a str) -> Vec<&'a str> {
    doc.split_whitespace().collect()
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_doc_parse() {
        let exists = doc_parse("намело сугробы у нашего двора");
        let expected = vec!["намело", "сугробы", "у", "нашего", "двора"];

        assert_eq!(exists, expected);
    }
}