use rustyline::{error::ReadlineError, DefaultEditor};
use simplesl::{Error, Interpreter, Result};
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Box<[String]> = env::args().collect();
    match &args[..] {
        [_] => run_shell(),
        [_, file] => run_from_file(file),
        _ => Err(Error::TooManyArguments),
    }
    .map_or_else(
        |error| {
            eprintln!("{error}");
            ExitCode::FAILURE
        },
        |_| ExitCode::SUCCESS,
    )
}

fn run_shell() -> Result<()> {
    let mut interpreter = Interpreter::with_stdlib();
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line)?;
                if !line.is_empty() {
                    if let Err(error) = interpreter.parse_and_exec(&line) {
                        eprintln!("{error}");
                    }
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => return Ok(()),
            Err(err) => return Err(err.into()),
        }
    }
}

fn run_from_file(path: &str) -> Result<()> {
    let mut interpreter = Interpreter::with_stdlib();
    interpreter.load_and_exec(path).map(|_| ())
}
