use markdown::{ParseOptions, mdast};
use simplesl::{Code, Interpreter};
use std::fs;

#[test]
fn test_code_samples_in_readme() {
    let file = fs::read_to_string("README.md").unwrap();
    let mdast::Node::Root(root) = markdown::to_mdast(&file, &ParseOptions::default()).unwrap()
    else {
        panic!()
    };
    let interpreter = Interpreter::with_stdlib();
    for node in root.children.iter() {
        let mdast::Node::Code(mdast::Code {
            value,
            lang: Some(lang),
            ..
        }) = node
        else {
            continue;
        };
        if lang == "SimpleSL" {
            Code::parse(&interpreter, value).unwrap().exec().unwrap();
        }
    }
}
