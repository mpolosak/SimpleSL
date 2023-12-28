use simplesl::{Code, Interpreter};

fn main() {
    let interpreter = Interpreter::with_stdlib();
    let _ = Code::parse(&interpreter, "print(\"Hello world!\")")
        .unwrap()
        .exec();
}
