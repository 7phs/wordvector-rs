#[cfg(test)]
pub(crate) fn doc_parse<'a>(doc: &'a str) -> Vec<&'a str> {
    doc.split_whitespace().collect()
}

// TODO: make macros
pub fn vec_sum<T>(vecs: T) -> Vec<f32>
    where T: IntoIterator<Item=Vec<f32>>
{
    vecs.into_iter().fold(Vec::new(), |mut acc, vector| {
        if acc.is_empty() {
            acc.resize_default(vector.len());
        }

        acc.iter_mut()
            .zip(vector.iter())
            .for_each(|(v1, v2)| *v1 += v2);

        acc
    })
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

    #[test]
    fn test_vec_sum() {
        let data = vec![
            vec![1.0f32, 2.0, 3.0, 4.0, 5.0],
            vec![2.0f32, 3.0, 4.0, 5.0, 6.0],
            vec![3.0f32, 4.0, 5.0, 6.0, 7.0],
        ];

        let exist = vec_sum(data);

        let expected = vec![6.0f32, 9.0, 12.0, 15.0, 18.0];

        assert_eq!(exist, expected, "check vectors sum");
    }
}