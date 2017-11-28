use rand;
use rand::Rng;
use std::collections::BTreeMap;

use ::wordvector::{WordVectorModel, WordVectorDistance};

const TEST_DICT_TEXT: [&str; 5] = ["намело", "сугробы", "у", "нашего", "крыльца"];
const VEC_LEN: i32 = 10;

pub(crate) struct TestModel {
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
            // TODO: SUM
            .fold(Vec::new(), |mut acc, word_vector| {
                if let Some(word_vector) = word_vector {
                    // acc += word_vector;
                    if acc.is_empty() {
                        acc.resize_default(word_vector.len());
                    }

                    acc.iter_mut()
                        .zip(word_vector.iter())
                        .for_each(|(v1, v2)| *v1 += v2);
                }

                acc
            });

        if doc_vec.len() > 0 {
            Some(doc_vec)
        } else {
            None
        }
    }
}

impl WordVectorDistance for TestModel {
    fn calc(&self, _doc_bow1: &[f32], _doc_bow2: &[f32], _distance_matrix: &[&[f32]]) -> f32 {
        15.692f32
    }
}
