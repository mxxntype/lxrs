#[test]
fn tokenize() {
    let token_stream = codecrafters_interpreter::tokenize("tests/data/empty_file.lox").unwrap();
    assert_eq!(token_stream, "EOF  null");
}
