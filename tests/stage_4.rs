use indoc::indoc;

#[test]
fn tokenize() {
    let token_stream = codecrafters_interpreter::tokenize("tests/data/stage_4.lox").unwrap();
    let expected_stream = indoc! {"
        LEFT_PAREN ( null
        LEFT_PAREN ( null
        RIGHT_PAREN ) null
        EOF  null
    "}
    .trim();

    assert_eq!(token_stream, expected_stream);
}
