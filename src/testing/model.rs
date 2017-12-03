use std::collections::BTreeMap;

use ::{WordVectorModel, WordVectorDistance};
use ::utils::vec_sum;

pub(crate) struct TestModel {
    data: BTreeMap<String, i64>,
    vectors: BTreeMap<String, Vec<f32>>,
}

impl Default for TestModel {
    fn default() -> TestModel {
        let data: BTreeMap<String, i64> = [
            ("намело".to_string(), 0),
            ("сугробы".to_string(), 1),
            ("у".to_string(), 2),
            ("нашего".to_string(), 3),
            ("крыльца".to_string(), 4)
        ].iter().cloned().collect();

        let vectors: BTreeMap<String, Vec<f32>> = [
            ("намело".to_string(), vec![0.09620774f32, 0.57043445, 0.25864983, 0.56875455, 0.013802886, 0.18820941, 0.6235244, 0.9685433, 0.33947492, 0.30321276]),
            ("сугробы".to_string(), vec![0.536739f32, 0.42155814, 0.12513518, 0.86498487, 0.2764778, 0.86605155, 0.47319448, 0.09259939, 0.013579607, 0.1396333]),
            ("у".to_string(), vec![0.1208024f32, 0.48896742, 0.72400975, 0.45157206, 0.9912764, 0.334648, 0.04849291, 0.972154, 0.53880787, 0.47627044]),
            ("нашего".to_string(), vec![0.13930714f32, 0.22024155, 0.8076284, 0.6289493, 0.18162072, 0.34256923, 0.14036989, 0.995613, 0.269382, 0.175102]),
            ("крыльца".to_string(), vec![0.032780647f32, 0.15684962, 0.8428451, 0.3130175, 0.7753111, 0.07664311, 0.4615686, 0.7738153, 0.9659138, 0.2086786])
        ].iter().cloned().collect();

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
        let doc_vec: Vec<f32> = vec_sum(
            text.split_whitespace()
                .filter_map(|word| self.word_to_vector(&word))
        );

        if doc_vec.len() > 0 {
            Some(doc_vec)
        } else {
            None
        }
    }
}

impl WordVectorDistance for TestModel {
    fn calc(&self, doc_bow1: &[f32], doc_bow2: &[f32], distance_matrix: &[f32]) -> f32 {
        doc_bow1[1] * doc_bow2[2] * distance_matrix[2*4 + 3]
    }
}
