use rustyline::{error::ReadlineError, DefaultEditor};
use simplesl::{interpreter::Interpreter, Error, Result};
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
            eprintln!("{error:?}");
            ExitCode::FAILURE
        },
        |_| ExitCode::SUCCESS,
    )
}

fn run_shell() -> Result<()> {
    let mut interpreter = Interpreter::new();
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(mut line) => {
                rl.add_history_entry(&line)?;
                line = line.replace('\n', "");
                if !line.is_empty() {
                    if let Err(error) = interpreter.parse_and_exec(&line) {
                        eprintln!("{error}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => return Ok(()),
            Err(err) => return Err(err.into()),
        }
    }
}

fn run_from_file(path: &str) -> Result<()> {
    let mut interpreter = Interpreter::new();
    interpreter.load_and_exec(path).map(|_| ())
}
