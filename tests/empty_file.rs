#[test]
fn tokenize() {
    let token_stream = codecrafters_interpreter::tokenize("./data/empty_file.lox");
    assert_eq!(token_stream, "EOF  null");
}
