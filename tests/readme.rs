use std::fs;

use markdown::{tokenize, Block};
use simplesl::{Code, Interpreter};

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
            let code =
                Code::parse(&interpreter, &content).unwrap_or_else(|error| panic!("{error}"));
            code.exec().unwrap_or_else(|error| panic!("{error}"));
        }
    }
}
