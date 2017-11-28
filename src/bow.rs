use std::collections::BTreeMap;
use ::dictionary::Dictionary;

impl Dictionary {
    fn doc_to_bow<T>(&self, doc: &[T]) -> Vec<i64>
        where
            T: ToString
    {
        let mut counter: BTreeMap<String, i64> = BTreeMap::new();

        for word in doc.iter().map(|word| word.to_string()) {
            if let Some(mut value) = counter.get_mut(&word) {
                *value += 1;
                continue;
            }

            counter.insert(word, 1);
        }

        let mut res: Vec<i64> = Vec::new();
        res.resize(self.len(), 0);

        for (word, &freq) in counter.iter() {
            if let Some(index) = self.word_index(word) {
                res[index as usize] = freq;
            }
        }

        res
    }

    pub fn bow_normalized<T>(&self, doc: &[T]) -> Option<Vec<f32>>
        where
            T: ToString
    {
        let normalizer: f32 = doc.len() as f32;

        if normalizer == 0.0 {
            return None
        }

        Some(self.doc_to_bow(&doc)
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
        let dict = Dictionary::with_extend(&["крыльца", "намело", "нашего", "сугробы", "у"]);

        let text = ["намело", "сугробы", "намело", "вдвойне", "у", "крыльца", "намело", "намело", "за", "крыльца"];

        let exist = dict.doc_to_bow(&text);

        assert_eq!(exist, [2, 4, 0, 1, 1], "check doc to bow");
    }

    #[test]
    fn test_dictionary_bow_normalized() {
        let dict = Dictionary::with_extend(&["крыльца", "намело", "нашего", "сугробы", "у"]);

        {
            let text = ["намело", "сугробы", "намело", "вдвойне", "у", "крыльца", "намело", "намело", "за", "крыльца"];

            if let Some(exist) = dict.bow_normalized(&text) {
                assert_eq!(exist, [0.2f32, 0.4, 0.0, 0.1, 0.1], "check bow normalize");
            } else {
                assert!(false, "failed to get bow normalized");
            }
        }

        {
            let text = ["в", "бананово", "лимонном", "сингапуре", "в", "буре"];

            if let Some(exist) = dict.bow_normalized(&text) {
                assert_eq!(exist, [0.0f32, 0.0, 0.0, 0.0, 0.0], "check whole empty bow normalize");
            } else {
                assert!(false, "failed to get bow normalized");
            }
        }

        {
            let text: Vec<&str> = Vec::new();

            if let Some(_) = dict.bow_normalized(&text) {
                assert!(false, "failed to check empty bow");
            } else {
                assert!(true, "right checking empty bow");
            }
        }
    }
}