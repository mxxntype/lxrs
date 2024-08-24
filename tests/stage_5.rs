use indoc::indoc;

#[test]
fn tokenize() {
    let token_stream = codecrafters_interpreter::tokenize("tests/data/stage_5.lox").unwrap();
    let expected_stream = indoc! {"
        LEFT_BRACE { null
        LEFT_BRACE { null
        RIGHT_BRACE } null
        RIGHT_BRACE } null
        EOF  null
    "}
    .trim();

    assert_eq!(token_stream, expected_stream);
}
