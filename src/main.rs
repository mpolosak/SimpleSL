use rustyline::{error::ReadlineError, DefaultEditor};
use simplesl::{Code, Error, Interpreter, Result};
use std::{env, fs, process::ExitCode};

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
                if let Err(error) = Code::parse(&interpreter, &line)
                    .and_then(|code| code.exec_unscoped(&mut interpreter))
                {
                    eprintln!("{error}");
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => return Ok(()),
            Err(err) => return Err(err.into()),
        }
    }
}

fn run_from_file(path: &str) -> Result<()> {
    let script = fs::read_to_string(path)?;
    let interpreter = Interpreter::with_stdlib();
    Code::parse(&interpreter, &script)?.exec().map(|_| ())
}
