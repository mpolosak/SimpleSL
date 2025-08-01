use crate as simplesl;
use simplesl_macros::export;

#[export(IO)]
pub mod inner {
    use crate::join;
    pub use {crate::variable::Variable, std::io};

    pub fn print(var: &Variable) {
        println!("{var}");
    }

    pub fn print_array(array: &[Variable], sep: &str) {
        println!("{}", join(array, sep));
    }

    pub fn cgetline() -> io::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.replace('\n', "");
        Ok(input)
    }
}
