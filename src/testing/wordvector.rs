use dictionary::Dictionary;
use utils::doc_parse;
use {WordVector};

use testing::model::TestModel;

#[test]
fn test_wordvector_dictionary() {
    let model = TestModel::default();
    let vector = WordVector::new(&model, &model);

    let exist_dict = vector.dictionary(&doc_parse("намело сугробы за калиткой"));

    let expected_dict = Dictionary::with_extend(&doc_parse("намело сугробы"));

    assert_eq!(exist_dict, expected_dict, "check dict");
}

#[test]
fn test_wordvector_doc_to_unit_core() {
    let model = TestModel::default();
    let vector = WordVector::new(&model, &model);

    match vector.doc_to_unite_core(&["намело", "сугробы"]) {
        Ok(exist_unite_core) => {
            let s: f32 = exist_unite_core.iter().sum();
            assert!(s > 0.0f32, "failed to calc non zero unit core");
        }
        Err(err) => assert!(false, "failed to calc unit core {:?}", err),
    }
}

#[test]
fn test_wordvector_words_distance() {
    let model = TestModel::default();
    let vector = WordVector::new(&model, &model);

    let exist_distance = match vector.words_distance("намело", "сугробы") {
        Some(distance) => distance,
        None => {
            assert!(false, "failed to calc distance - one of words wasn't find");
            0.0f32
        }
    };
    let expected_distance = 1.331586f32;

    assert_eq!(exist_distance, expected_distance, "check distance calc");
}

#[test]
fn test_wordvector_wm_distance() {
    let model = TestModel::default();
    let vector = WordVector::new(&model, &model);

    let exist_distance = match vector.wm_distance(
        &doc_parse("намело сугробы"),
        &doc_parse("сугробы у крыльца")
    ) {
        Ok(distance) => distance,
        Err(err) => {
            assert!(false, "failed to calc distance {:?}", err);
            0.0f32
        }
    };
    let expected_distance = 0.28105024f32;

    assert_eq!(exist_distance, expected_distance, "check distance");
}

#[test]
fn test_wordvector_similarity() {
    let model = TestModel::default();
    let vector = WordVector::new(&model, &model);

    let exist_similarity = match vector.similarity(
        &doc_parse("намело сугробы"),
        &doc_parse("сугробы у крыльца")
    ) {
        Ok(similarity) => similarity,
        Err(err) => {
            assert!(false, "failed to calc similarity {:?}", err);
            0.0f32
        }
    };
    let expected_similarity = 0.8433072f32;

    assert_eq!(exist_similarity, expected_similarity, "check similarity value");
}