use dictionary::Dictionary;
use matrix::Matrix;
use utils::vec_sum;

pub trait WordVectorModel {
    fn word_index(&self, word: &str) -> Option<i64>;
    fn word_to_vector(&self, word: &str) -> Option<Vec<f32>>;
    fn sentence_to_vector(&self, text: &str) -> Option<Vec<f32>>;
}

pub trait WordVectorDistance {
    fn calc(&self, doc_bow1: &[f32], doc_bow2: &[f32], distance_matrix: &[f32]) -> f32;
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

    pub(crate) fn dictionary<T>(&self, doc: &[T]) -> Dictionary
        where
            T: ToString
    {
        let mut dict = Dictionary::default();

        for w in doc {
            let word: String = w.to_string();

            if let Some(_) = self.model.word_index(&word) {
                dict.insert(word);
            }
        }

        dict
    }

    pub(crate) fn doc_to_unite_core<T>(&self, doc: &[T]) -> Result<Vec<f32>, &str>
        where
            T: ToString
    {
        let doc_len = doc.len() as f32;

        let mut unite_core: Vec<f32> = vec_sum(
            doc.iter()
            .filter_map(|word|
                self.model.word_to_vector(&word.to_string())
            )
        );

        // TODO: NORMALIZE
        unite_core.iter_mut()
            .for_each(|v| *v /= doc_len);

        let distance: f32 = unite_core.iter()
            .fold(0.0f32, |acc, v| {
                acc + v * v
            })
            .sqrt();

        if distance > 0.0f32 {
            // TODO: NORMALIZE
            unite_core.iter_mut()
                .for_each(|v| *v /=  distance)
        }

        Ok(unite_core)
    }

    pub fn words_distance(&self, word1: &str, word2: &str) -> Option<f32> {
        let vec1 = self.model.word_to_vector(&word1)?;
        let vec2 = self.model.word_to_vector(&word2)?;
        let distance: f32 = vec1.iter()
            .zip(vec2.iter())
            .map(|(&v1, v2)| v1 - v2)
            .map(|v| v*v)
            .sum();

        Some(distance.sqrt())
    }

    pub fn wm_distance<T>(&self, doc1: &[T], doc2: &[T]) -> Result<f32, &str>
        where
            T: ToString
    {
        let dict1 = self.dictionary(&doc1);
        let dict2 = self.dictionary(&doc2);

        if dict1.is_empty() || dict2.is_empty() {
            return Err("empty dictionary");
        }

        let dict = dict1.join(&dict2);
        if dict.len() <= 1 {
            return Ok(1.0);
        }

        let doc_bow1 = match dict.bow_normalized(&doc1) {
            Some(bow) => bow,
            None => return Err("empty doc bow"),
        };

        let doc_bow2 = match dict.bow_normalized(&doc2) {
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

        Ok(self.distance.calc(&doc_bow1, &doc_bow2, matrix.as_slice()))
    }

    pub fn similarity<T>(&self, doc1: &[T], doc2: &[T]) -> Result<f32, &str>
        where
            T: ToString
    {
        let unit_core1 = self.doc_to_unite_core(&doc1)?;
        let unit_core2 = self.doc_to_unite_core(&doc2)?;

        // TODO: MUL
        Ok(unit_core1.iter()
            .zip(unit_core2.iter())
            .map(|(v1, v2)| v1 * v2)
            .sum())
    }
}
