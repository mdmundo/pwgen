use crate::parse_option;
use rstest::rstest;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[rstest]
#[case(0, 0)]
#[case(4, 4)]
#[case(255, 255)]
fn it_has_length(#[case] input: u8, #[case] expected: usize) {
    let pw = parse_option("alpha", input);

    assert_eq!(pw.len(), expected);
}

#[rstest]
#[case(48, true)]
#[case(255, true)]
fn it_is_pin(#[case] input: u8, #[case] expected: bool) {
    let pw = parse_option("pin", input);

    assert_eq!(pw.chars().all(|ch| ch.is_ascii_digit()), expected);
}

#[rstest]
#[case(48, true)]
#[case(255, true)]
fn it_is_alphanum(#[case] input: u8, #[case] expected: bool) {
    let pw = parse_option("alpha", input);

    assert_eq!(pw.chars().all(|ch| ch.is_ascii_alphanumeric()), expected);
}

#[rstest]
#[case(128, true)]
#[case(255, true)]
fn it_is_alphanum_with_punct(#[case] input: u8, #[case] expected: bool) {
    let pw = parse_option("full", input);

    assert_eq!(
        pw.chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch.is_ascii_punctuation()),
        expected
    );
}
