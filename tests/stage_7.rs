use indoc::indoc;

#[test]
fn tokenize() {
    let token_stream = codecrafters_interpreter::tokenize("tests/data/stage_7.lox").unwrap_err();
    let expected_stream = indoc! {"
        COMMA , null
        DOT . null
        LEFT_PAREN ( null
        EOF  null
    "}
    .trim();

    assert_eq!(token_stream.0, 65);
    assert_eq!(token_stream.1, expected_stream);
}
