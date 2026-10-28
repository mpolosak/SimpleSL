use markdown::{Block, tokenize};
use simplesl::{Code, Interpreter};
use std::fs;

#[test]
fn test_code_samples_in_readme() {
    let file = fs::read_to_string("README.md").unwrap();
    let tokens = tokenize(&file);
    let interpreter = Interpreter::with_stdlib();
    for token in tokens {
        let Block::CodeBlock(Some(lang), content) = token else {
            continue;
        };
        if lang == "SimpleSL" {
            Code::parse(&interpreter, &content).unwrap().exec().unwrap();
        }
    }
}
