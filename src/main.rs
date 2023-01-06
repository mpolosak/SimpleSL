use std::env;
use std::io;
use std::io::Write;
mod intepreter;
use intepreter::Intepreter;
mod parse;
mod iofunctions;
mod stdfunctions;
mod function;
mod variable;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..]{
        [_] => run_shell(),
        [_, file] => run_from_file(file.clone()),
        _ => println!("Too many arguments")
    }
}

fn run_shell(){
    let mut intepreter = Intepreter::new();
    let mut rl = Editor::<()>::new().expect("Unable to read user input");
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(mut line) => {
                rl.add_history_entry(line.as_str());
                line = line.replace("\n", "");
                if !line.is_empty(){
                    if let Err(error) = intepreter.exec(line){
                        eprintln!("{}", error);
                    }
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof)  => {
                break
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break
            }
        }
    }
}

fn run_from_file(path: String) {
    let mut intepreter = Intepreter::new();
    intepreter.load_and_exec(path).unwrap();
}
