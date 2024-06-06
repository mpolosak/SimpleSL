use crate as simplesl;
use simplesl_macros::export;

#[export]
mod add_io {
    use crate::{join, variable::Variable};
    use std::io;
    fn print(var: &Variable) {
        println!("{var}");
    }

    fn print_array(array: &[Variable], sep: &str) {
        println!("{}", join(array, sep));
    }

    fn cgetline() -> io::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.replace('\n', "");
        Ok(input)
    }
}
