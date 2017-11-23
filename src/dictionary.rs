use std::borrow::Cow;
use std::collections::{BTreeMap, btree_map};
use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug)]
pub struct Dictionary<'a> {
    data: BTreeMap<Cow<'a, str>, i64>,
    index: AtomicI64,
}

impl<'a> Dictionary<'a> {
    pub fn new() -> Dictionary<'a> {
        Dictionary {
            data: BTreeMap::new(),
            index: AtomicI64::new(0),
        }
    }

    pub fn iter(&self) -> btree_map::Keys<Cow<'a, str>, i64> {
        self.data.keys()
    }

    pub fn insert<S>(&mut self, word: S)
        where S: Into<Cow<'a, str>>
    {
        let w: Cow<'a, str> = word.into();

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

    pub fn contains(&self, word: &Cow<'a, str>) -> bool
    {
        self.data.contains_key(word)
    }

    pub fn word_index(&self, word: &Cow<'a, str>) -> Option<i64> {
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

impl<'a> PartialEq for Dictionary<'a> {
    fn eq<'b>(&self, other: &Dictionary<'b>) -> bool {
        self.data == other.data
    }
}

impl<'a, T: Into<Cow<'a, str>>> Extend<T> for Dictionary<'a> {
    #[inline]
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for word in iter.into_iter() {
            let w: Cow<'a, str> = word.into();

            if !self.contains(&w) {
                let index = self.index_inc();
                self.data.insert(w, index);
            }
        }
    }
}

impl<'a> Dictionary<'a> {
    pub fn new_extend<I, T>(words: I) -> Dictionary<'a>
        where
            I: IntoIterator<Item=T>,
            T: Into<Cow<'a, str>>
    {
        let mut dict = Dictionary::new();

        dict.extend(words);

        dict.reindex();

        dict
    }

    pub fn join(&self, other: &Dictionary<'a>) -> Dictionary<'a> {
        let mut dict = Dictionary::new_extend(self.iter().cloned());

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
            let dict = Dictionary::new();

            assert_eq!(dict.len(), 0, "check zero length");
            assert_eq!(dict.is_empty(), true, "check empty");
        }
    }

    #[test]
    fn test_dictionary_insert() {
        let mut dict = Dictionary::new();

        {
            let word = "other".to_string();

            dict.insert("hello");
            dict.insert("мои");
            dict.insert(16.to_string());
            dict.insert("друзья");
            dict.insert("мои");
            dict.insert("други");
            dict.insert(word.clone());

            assert_eq!(word, "other", "check borrow");
        }

        assert_eq!(dict.len(), 6, "check length");
        assert_eq!(dict.is_empty(), false, "check empty");
    }

    #[test]
    fn test_dictionary_extend() {
        let mut dict = Dictionary::new();

        let mut dict2 = Dictionary::new();
        dict2.insert("намело");
        dict2.insert("сугробы");

        dict.extend(dict2.iter().cloned());
        dict.extend(["hello", "мои", "друзья", "мои", "други"].iter().cloned());

        assert_eq!(dict.len(), 6, "check length");
        assert_eq!(dict.is_empty(), false, "check empty");
    }

    #[test]
    fn test_dictionary_new_extend() {
        {
            let dict = Dictionary::new_extend(["hello", "мои", "друзья", "мои", "други"].iter().cloned());

            assert_eq!(dict.len(), 4, "check length");
            assert_eq!(dict.is_empty(), false, "check empty");
        }
    }

    #[test]
    fn test_dictionary_conains() {
        let dict = Dictionary::new_extend(["hello", "мои", "друзья", "мои", "други"].iter().cloned());

        assert!(dict.contains(&"друзья".into()));
        assert!(!dict.contains(&"враги".into()));
    }

    #[test]
    fn test_dictionary_join() {
        let dict = Dictionary::new_extend(["намело", "сугробы", "у", "нашего", "крыльца"].iter().cloned());
        let dict2 = Dictionary::new_extend(["стонет", "стужа", "и", "намело", "сугробы"].iter().cloned());

        let exist = dict.join(&dict2);

        let mut expected = Dictionary::new_extend([
            "сугробы", "крыльца", "нашего", "намело",
            "у", "стужа", "стонет", "и"].iter().cloned());

        expected.reindex();

        assert_eq!(exist, expected, "check join");

        assert!(exist.contains(&"и".into()))
    }

    #[test]
    fn test_dictionary_word_index() {
        let dict = Dictionary::new_extend(["намело", "сугробы", "у", "нашего", "крыльца"].iter().cloned());

        {
            let exist_index = match dict.word_index(&"нашего".into()) {
                Some(index) => index,
                None => -1,
            };
            let expected_index: i64 = 2;

            assert_eq!(exist_index, expected_index, "check index");
        }

        {
            let exist_index = match dict.word_index(&"unknown".into()) {
                Some(index) => index,
                None => -1,
            };
            let expected_index: i64 = -1;

            assert_eq!(exist_index, expected_index, "check unknown index");
        }
    }
}