use rustyline::{error::ReadlineError, DefaultEditor};
use simplesl::{Code, Error, Interpreter};
use simplesl_macros::var;
use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    var!([-5.5; 5]);
    let mut args = env::args();
    if args.len() > 2 {
        eprintln!("Too many arguments");
        return ExitCode::FAILURE;
    }
    if let Some(path) = args.nth(1) {
        return run_from_file(&path).map_or_else(
            |error| {
                eprintln!("{error}");
                ExitCode::FAILURE
            },
            |_| ExitCode::SUCCESS,
        );
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
        match Code::parse(&interpreter, &line)
            .and_then(|code| code.exec_unscoped(&mut interpreter).map_err(Error::from))
        {
            Ok(result) => println!("{result:?}"),
            Err(error) => eprintln!("{error}"),
        }
    }
}

fn run_from_file(path: &str) -> Result<(), Error> {
    let script = fs::read_to_string(path)?;
    let interpreter = Interpreter::with_stdlib();
    Code::parse(&interpreter, &script)?.exec()?;
    Ok(())
}
