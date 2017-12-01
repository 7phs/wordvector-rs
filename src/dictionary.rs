use std::collections::{BTreeMap, btree_map};
use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug)]
pub struct Dictionary {
    data: BTreeMap<String, i64>,
    index: AtomicI64,
}

impl Default for Dictionary {
    fn default() -> Dictionary {
        Dictionary {
            data: BTreeMap::new(),
            index: AtomicI64::new(0),
        }
    }
}

impl Dictionary {
    pub fn iter(&self) -> btree_map::Keys<String, i64> {
        self.data.keys()
    }

    pub fn insert<S>(&mut self, word: S)
        where S: ToString
    {
        let w: String = word.to_string();

        if !self.contains(&w) {
            let index = self.index_inc();
            self.data.insert(w, index);
        }
    }

    pub fn reindex(&mut self) {
        self.index.store(0, Ordering::SeqCst);

        let mut index = 0i64;

        for (_, value) in self.data.iter_mut() {
            *value = index;

            index += 1;
        }

        self.index.store(index, Ordering::SeqCst);
    }

    pub fn contains<'a>(&self, word: &'a str) -> bool
    {
        self.data.contains_key(word)
    }

    pub fn word_index<'a>(&self, word: &'a str) -> Option<i64> {
        match self.data.get(word) {
            Some(&index) => Some(index),
            None => None,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn index_inc(&self) -> i64 {
        self.index.fetch_add(1, Ordering::SeqCst)
    }
}

impl PartialEq for Dictionary {
    fn eq(&self, other: &Dictionary) -> bool {
        self.data == other.data
    }
}

impl<T> Extend<T> for Dictionary
    where T: ToString
{
    #[inline]
    fn extend<I>(&mut self, iter: I)
        where
            I: IntoIterator<Item=T>,
            T: ToString
    {
        for word in iter.into_iter() {
            self.insert(word);
        }
    }
}

impl Dictionary {
    pub fn with_extend<I, T>(words: I) -> Dictionary
        where
            I: IntoIterator<Item=T>,
            T: ToString
    {
        let mut dict = Dictionary::default();

        dict.extend(words);

        dict.reindex();

        dict
    }

    pub fn join(&self, other: &Dictionary) -> Dictionary {
        let mut dict = Dictionary::with_extend(self.iter().cloned());

        dict.extend(other.iter().cloned());

        dict.reindex();

        dict
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_dictionary() {
        {
            let dict = Dictionary::default();

            assert_eq!(dict.len(), 0, "check zero length");
            assert_eq!(dict.is_empty(), true, "check empty");
        }
    }

    #[test]
    fn test_dictionary_insert() {
        let mut dict = Dictionary::default();

        {
            let word = "other".to_string();

            dict.insert("hello");
            dict.insert("мои");
            dict.insert(16);
            dict.insert("друзья");
            dict.insert("мои");
            dict.insert("други");
            dict.insert(&word);

            assert_eq!(word, "other", "check borrow");
        }

        assert_eq!(dict.len(), 6, "check length");
        assert_eq!(dict.is_empty(), false, "check empty");
    }

    #[test]
    fn test_dictionary_extend() {
        let mut dict = Dictionary::default();

        let mut dict2 = Dictionary::default();
        dict2.insert("намело");
        dict2.insert("сугробы");

        dict.extend(dict2.iter().cloned());
        dict.extend(&["hello", "мои", "друзья", "мои", "други"]);

        assert_eq!(dict.len(), 6, "check length");
        assert_eq!(dict.is_empty(), false, "check empty");
    }

    #[test]
    fn test_dictionary_new_extend() {
        let dict = Dictionary::with_extend(&["hello", "мои", "друзья", "мои", "други"]);

        assert_eq!(dict.len(), 4, "check length");
        assert_eq!(dict.is_empty(), false, "check empty");
    }

    #[test]
    fn test_dictionary_conains() {
        let dict = Dictionary::with_extend(&["hello", "мои", "друзья", "мои", "други"]);

        assert!(dict.contains("друзья"));
        assert!(!dict.contains("враги"));
    }

    #[test]
    fn test_dictionary_join() {
        let dict = Dictionary::with_extend(&["намело", "сугробы", "у", "нашего", "крыльца"]);
        let dict2 = Dictionary::with_extend(&["стонет", "стужа", "и", "намело", "сугробы"]);

        let exist = dict.join(&dict2);

        let mut expected = Dictionary::with_extend(&[
            "сугробы", "крыльца", "нашего", "намело",
            "у", "стужа", "стонет", "и"
        ]);

        expected.reindex();

        assert_eq!(exist, expected, "check join");

        assert!(exist.contains("и"))
    }

    #[test]
    fn test_dictionary_word_index() {
        let dict = Dictionary::with_extend(&["намело", "сугробы", "у", "нашего", "крыльца"]);

        {
            let exist_index = match dict.word_index("нашего") {
                Some(index) => index,
                None => -1,
            };
            let expected_index: i64 = 2;

            assert_eq!(exist_index, expected_index, "check index");
        }

        {
            let exist_index = match dict.word_index("unknown") {
                Some(index) => index,
                None => -1,
            };
            let expected_index: i64 = -1;

            assert_eq!(exist_index, expected_index, "check unknown index");
        }
    }
}