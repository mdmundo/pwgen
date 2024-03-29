use crate::generate;
use rstest::rstest;

#[rstest]
#[case(0, 0)]
#[case(4, 4)]
#[case(255, 255)]
fn it_has_length(#[case] input: usize, #[case] expected: usize) {
    let pw = generate("alpha", input);

    assert_eq!(pw.len(), expected);
}

#[rstest]
#[case(48, true)]
#[case(255, true)]
fn it_is_pin(#[case] input: usize, #[case] expected: bool) {
    let pw = generate("pin", input);

    assert_eq!(pw.chars().all(|ch| ch.is_ascii_digit()), expected);
}

#[rstest]
#[case(48, true)]
#[case(255, true)]
fn it_is_alphanum(#[case] input: usize, #[case] expected: bool) {
    let pw = generate("alpha", input);

    assert_eq!(pw.chars().all(|ch| ch.is_ascii_alphanumeric()), expected);
}

#[rstest]
#[case(128, true)]
#[case(255, true)]
fn it_is_alphanum_with_punct(#[case] input: usize, #[case] expected: bool) {
    let pw = generate("full", input);

    assert_eq!(
        pw.chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch.is_ascii_punctuation()),
        expected
    );
}
