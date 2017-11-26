use dictionary::Dictionary;
use matrix::Matrix;

pub trait WordVectorModel {
    fn word_index(&self, word: &str) -> Option<i64>;
    fn word_to_vector(&self, word: &str) -> Option<Vec<f32>>;
    fn sentence_to_vector(&self, text: &str) -> Option<Vec<f32>>;
}

pub trait WordVectorDistance {
    fn calc(&self, doc_bow1: &[f32], doc_bow2: &[f32], distance_matrix: &[&[f32]]) -> f32;
}

pub struct WordVector<'a> {
    model: &'a WordVectorModel,
    distance: &'a WordVectorDistance,
}

impl<'a> WordVector<'a> {
    pub fn new(model: &'a WordVectorModel, distance: &'a WordVectorDistance) -> WordVector<'a> {
        WordVector {
            model,
            distance,
        }
    }

    fn dictionary<I, T>(&self, doc: I) -> Dictionary
        where
            I: IntoIterator<Item=T>,
            T: ToString
    {
        let mut dict = Dictionary::default();

        for w in doc.into_iter() {
            let word: String = w.to_string();

            if let Some(_) = self.model.word_index(&word) {
                dict.insert(word);
            }
        }

        dict
    }

    pub fn words_distance(&self, word1: &str, word2: &str) -> Option<f32> {
        let vec1 = self.model.word_to_vector(&word1)?;
        let vec2 = self.model.word_to_vector(&word2)?;

        Some(vec1.iter()
            .zip(vec2.iter())
            .map(|(&v1, v2)| v1 + v2)
            .map(|v| v*v)
            .sum())
    }

    pub fn wm_distance<I, T>(&self, doc1: I, doc2: I) -> Result<f32, &str>
        where
            I: IntoIterator<Item=T> + Clone,
            T: ToString
    {
        let dict1 = self.dictionary(doc1.clone());
        let dict2 = self.dictionary(doc2.clone());

        if dict1.is_empty() || dict2.is_empty() {
            return Err("empty dictionary");
        }

        let dict = dict1.join(&dict2);
        if dict.len() <= 1 {
            return Ok(1.0);
        }

        let doc_bow1 = match dict.bow_normalized(doc1) {
            Some(bow) => bow,
            None => return Err("empty doc bow"),
        };

        let doc_bow2 = match dict.bow_normalized(doc2) {
            Some(bow) => bow,
            None => return Err("empty doc bow"),
        };

        let mut matrix = Matrix::new(dict.len());

        {
            let mut matrix_2d = matrix.as_matrix_mut();

            for (i, word1) in dict.iter().enumerate() {
                for (j, word2) in dict.iter().enumerate() {
                    if dict1.contains(&word1) && dict2.contains(&word2) {
                        if let Some(distance) = self.words_distance(&word1, &word2) {
                            matrix_2d[i][j] = distance;
                        }
                    }
                }
            }
        }

        Ok(self.distance.calc(&doc_bow1, &doc_bow2, &matrix.as_matrix()))
    }

    pub fn similarity<I, T>(&self, doc1: I, doc2: I) -> Result<f32, &str>
        where
            I: IntoIterator<Item=T>,
            T: ToString
    {
        Err("not implemented")
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    use ::rand;
    use ::rand::Rng;
    use ::std::collections::BTreeMap;

    const TEST_DICT_TEXT: [&str; 5] = ["намело", "сугробы", "у", "нашего", "крыльца"];
    const VEC_LEN: i32 = 10;

    struct TestModel {
        data: BTreeMap<String, i64>,
        vectors: BTreeMap<String, Vec<f32>>,
    }

    impl Default for TestModel {
        fn default() -> TestModel {
            let mut data: BTreeMap<String, i64> = BTreeMap::new();
            let mut vectors: BTreeMap<String, Vec<f32>> = BTreeMap::new();
            let mut rng = rand::thread_rng();

            for (index, word) in TEST_DICT_TEXT.iter().cloned().enumerate() {
                data.insert(word.to_string().into(), index as i64);
                vectors.insert(word.to_string().into(), (0..VEC_LEN)
                    .map(|_| rng.gen_range::<f32>(0.0, 1.0))
                    .collect());
            }

            TestModel {
                data,
                vectors,
            }
        }
    }

    impl WordVectorModel for TestModel {
        fn word_index(&self, word: &str) -> Option<i64> {
            match self.data.get(&word.to_string()) {
                Some(&index) => Some(index),
                None => None
            }
        }

        fn word_to_vector(&self, word: &str) -> Option<Vec<f32>> {
            match self.vectors.get(&word.to_string()) {
                Some(index) => Some(index.clone()),
                None => None,
            }
        }

        fn sentence_to_vector(&self, text: &str) -> Option<Vec<f32>> {
            let doc_vec: Vec<f32> = text.split_whitespace()
                .map(|word| self.word_to_vector(&word))
                .fold(Vec::<f32>::new(), |acc, word_vector| {
                    if let Some(word_vector) = word_vector {
                        acc.iter()
                            .zip(word_vector.iter())
                            .map(|(&v1, v2)| v1 + v2)
                            .collect()
                    } else {
                        acc
                    }
                });

            if doc_vec.len() > 0 {
                Some(doc_vec)
            } else {
                None
            }
        }
    }

    impl WordVectorDistance for TestModel {
        fn calc(&self, doc_bow1: &[f32], doc_bow2: &[f32], distance_matrix: &[&[f32]]) -> f32 {
            doc_bow1[1] * doc_bow2[1] * distance_matrix[2][2]
        }
    }

    #[test]
    fn test_wordvector_dictionary() {
        let model = TestModel::default();
        let vector = WordVector::new(&model, &model);

        let exist_dict = vector.dictionary(&["намело", "сугробы", "за", "калиткой"]);

        let expected_dict = Dictionary::with_extend(&["намело", "сугробы"]);

        assert_eq!(exist_dict, expected_dict, "check dict");
    }

    #[test]
    fn test_wordvector_wm_dictance() {
        let model = TestModel::default();
        let vector = WordVector::new(&model, &model);

        let exist_distance = match vector.wm_distance(
            "намело сугробы".split_whitespace(),
            "сугробы у крыльца".split_whitespace()
        ) {
            Ok(distance) => distance,
            Err(err) => {
                assert!(false, "failed to calc distance {:?}", err);
                0.0f32
            },
        };
        let expected_distance = 14.56f32;

        assert_eq!(exist_distance, expected_distance, "check distance");
    }
}