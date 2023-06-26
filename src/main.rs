use rustyline::{error::ReadlineError, Editor};
use simplesl::interpreter::Interpreter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_] => run_shell(),
        [_, file] => run_from_file(file),
        _ => println!("Too many arguments"),
    }
}

fn run_shell() {
    let mut intepreter = Interpreter::default();
    let mut rl = Editor::<()>::new().expect("Unable to read user input");
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(mut line) => {
                rl.add_history_entry(&line);
                line = line.replace('\n', "");
                if !line.is_empty() {
                    if let Err(error) = intepreter.exec(&line) {
                        eprintln!("{error}");
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                eprintln!("Error: {err:?}");
                break;
            }
        }
    }
}

fn run_from_file(path: &str) {
    let mut intepreter = Interpreter::new();
    if let Err(error) = intepreter.load_and_exec(path) {
        println!("{error}")
    }
}
