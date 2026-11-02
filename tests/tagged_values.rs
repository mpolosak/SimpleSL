use simplesl::{Code, Interpreter};
use simplesl_macros::var;

#[test]
fn tagged_values_construct_and_match_payloads() {
    let interpreter = Interpreter::without_stdlib();
    let code = Code::parse(
        &interpreter,
        r#"
        result := #Ok(42)
        match result {
            #Ok(value) => value,
            => 0,
        }
        "#,
    )
    .unwrap();

    assert_eq!(code.exec().unwrap(), var!(42));
}

#[test]
fn tagged_matches_cover_union_and_bind_precise_payload_types() {
    let interpreter = Interpreter::without_stdlib();
    let code = Code::parse(
        &interpreter,
        r#"
        make := (success: bool) -> #Ok(int) | #Err(string) {
            if success {
                return #Ok(42)
            }
            return #Err("failed")
        }
        match make(false) {
            #Ok(value) => value,
            #Err(message) => 7,
        }
        "#,
    )
    .unwrap();

    assert_eq!(code.exec().unwrap(), var!(7));
}

#[test]
fn tagged_match_requires_coverage_of_known_variants() {
    let interpreter = Interpreter::without_stdlib();
    let errors = Code::parse(
        &interpreter,
        r#"
        make := () -> #Ok(int) | #Err(string) {
            return #Ok(42)
        }
        match make() {
            #Ok(value) => value,
        }
        "#,
    )
    .unwrap_err();

    assert!(errors.contains(&simplesl::Error::MatchNotCovered));
}
