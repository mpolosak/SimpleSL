use rustyline::{DefaultEditor, error::ReadlineError};
use simplesl::{Code, Interpreter, variable::Variable};
use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args();
    if args.len() > 2 {
        eprintln!("Too many arguments");
        return ExitCode::FAILURE;
    }
    if let Some(path) = args.nth(1) {
        return run_from_file(&path);
    }
    run_shell().map_or_else(
        |error| {
            eprintln!("{error}");
            ExitCode::FAILURE
        },
        |_| ExitCode::SUCCESS,
    )
}

fn run_shell() -> Result<(), ReadlineError> {
    let mut interpreter = Interpreter::with_stdlib();
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("> ");
        let line = match readline {
            Ok(line) => line,
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => return Ok(()),
            Err(err) => return Err(err),
        };
        rl.add_history_entry(&line)?;
        if let Some(variable) = parse_exec_print_error(&mut interpreter, &line) {
            println!("{variable:?}")
        }
    }
}

fn run_from_file(path: &str) -> ExitCode {
    let Ok(script) = fs::read_to_string(path).inspect_err(|error| eprintln!("{error}")) else {
        return ExitCode::FAILURE;
    };
    let mut interpreter = Interpreter::with_stdlib();
    if parse_exec_print_error(&mut interpreter, &script).is_some() {
        return ExitCode::SUCCESS;
    }
    ExitCode::FAILURE
}

fn parse_exec_print_error(interpreter: &mut Interpreter, script: &str) -> Option<Variable> {
    let code = Code::parse(interpreter, script)
        .inspect_err(|error| eprintln!("{error}"))
        .ok()?;
    code.exec_unscoped(interpreter)
        .inspect_err(|error| eprintln!("{error}"))
        .ok()
}
