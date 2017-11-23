use std::borrow::Cow;
use std::collections::BTreeMap;
use ::dictionary::Dictionary;

impl<'a> Dictionary<'a> {
    fn doc_to_bow<T, I>(&self, doc: I) -> Vec<i64>
        where
            T: Into<Cow<'a, str>>,
            I: IntoIterator<Item=T>
    {
        let mut counter: BTreeMap<Cow<'a, str>, i64> = BTreeMap::new();

        for word in doc.into_iter().map(|word| word.into()) {
            if let Some(mut value) = counter.get_mut(&word) {
                *value += 1;
                continue;
            }

            counter.insert(word, 1);
        }

        let mut res: Vec<i64> = Vec::new();
        res.resize(self.len(), 0);

        for (word, &freq) in counter.iter() {
            if let Some(index) = self.word_index(word.into()) {
                res[index as usize] = freq;
            }
        }

        res
    }

    pub fn bow_normalized<T, I>(&self, doc: I) -> Option<Vec<f32>>
        where
            T: Into<Cow<'a, str>>,
            I: IntoIterator<Item=T>
    {
        let iter = doc.into_iter();

        let normalizer: f32 = {
            match iter.size_hint() {
                (_, Some(normalizer)) => normalizer as f32,
                _ => 0.0,
            }
        };

        if normalizer == 0.0 {
            return None
        }

        Some(self.doc_to_bow(iter)
            .into_iter()
            .map(|bow| bow as f32 / normalizer)
            .collect())
    }
}


#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_dictionary_doc_to_bow() {
        let dict = Dictionary::new_extend(["крыльца", "намело", "нашего", "сугробы", "у"].iter().cloned());

        let text = ["намело", "сугробы", "намело", "вдвойне", "у", "крыльца", "намело", "намело", "за", "крыльца"];

        let exist = dict.doc_to_bow(text.iter().cloned());

        assert_eq!(exist, [2, 4, 0, 1, 1], "check doc to bow");
    }

    #[test]
    fn test_dictionary_bow_normalized() {
        let dict = Dictionary::new_extend(["крыльца", "намело", "нашего", "сугробы", "у"].iter().cloned());

        {
            let text = ["намело", "сугробы", "намело", "вдвойне", "у", "крыльца", "намело", "намело", "за", "крыльца"];

            if let Some(exist) = dict.bow_normalized(text.iter().cloned()) {
                assert_eq!(exist, [0.2f32, 0.4, 0.0, 0.1, 0.1], "check bow normalize");
            } else {
                assert!(false, "failed to get bow normalized");
            }
        }

        {
            let text = ["в", "бананово", "лимонном", "сингапуре", "в", "буре"];

            if let Some(exist) = dict.bow_normalized(text.iter().cloned()) {
                assert_eq!(exist, [0.0f32, 0.0, 0.0, 0.0, 0.0], "check whole empty bow normalize");
            } else {
                assert!(false, "failed to get bow normalized");
            }
        }

        {
            let text: Vec<&str> = Vec::new();

            if let Some(_) = dict.bow_normalized(text.iter().cloned()) {
                assert!(false, "failed to check empty bow");
            } else {
                assert!(true, "right checking empty bow");
            }
        }
    }
}