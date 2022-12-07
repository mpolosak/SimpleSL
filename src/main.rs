use std::env;
use std::io;
use std::io::Write;
mod intepreter;
use intepreter::Intepreter;
mod params;
mod iofunctions;
mod stdfunctions;
mod function;
mod variable;

fn main() {
    let args: Vec<String> = env::args().collect();
    match &args[..]{
        [_] => run_shell(),
        [_, file] => run_from_file(file),
        _ => print!("Too many arguments")
    }
}

fn run_shell(){
    let mut intepreter = Intepreter::new();
    loop{
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read user input");
        input = input.replace("\n", "");
        if !input.is_empty(){
            if let Err(error) = intepreter.exec(input){
                eprintln!("{}", error);
            }
        }
    }
}

fn run_from_file(_path: &String){

}
