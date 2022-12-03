pub type ParamVec = Vec<Param>;

#[derive(Clone, Debug)]
pub enum Param{
    Float(f64),
    Text(String),
    Variable(String)
}

pub trait Parse {
    fn parse(text: String) -> Result<(Option<String>,Self), String> where Self: Sized;
}

impl Parse for ParamVec {
    fn parse(mut text: String) -> Result<(Option<String>, ParamVec), String> {
        let result_var;
        if let Some((before, after)) = text.split_once("=") {
            if after.contains("="){
                return Err(String::from("Too many = in one line"))
            }
            let before_s = before.trim().to_string();
            if before_s.contains(" ") || before_s.is_empty() {
                return Err(String::from("Before = should be exactly one variable"))
            }
            if before_s.contains("\"") ||  before_s.starts_with(
                &['-', '0', '1', '2', '3', '4',
                '5', '6', '7', '8', '9']){
                return Err(format!("{} isn't correct variable name", before_s))
            }
            result_var = Some(before_s);
            text = String::from(after);
        } else {
            result_var = None
        }
        let mut vec = ParamVec::new();
        while text.len()>0 {
            text = text.trim().to_string();
            let param = if text.starts_with('"') {
                text.remove(0);
                text.push(' ');
                if let Some((begin, rest)) = text.split_once("\" "){
                    let value = String::from(begin);
                    if value.contains("\""){
                        return Err(String::from("Incorrect syntax"));
                    }
                    text = String::from(rest);
                    Param::Text(value)
                } else {
                    return Err(String::from("Incorrect syntax"));
                }
            } else {
                let param_s;
                if let Some((begin, rest)) = text.split_once(" "){
                    param_s = String::from(begin);
                    text = String::from(rest);
                } else {
                    param_s = text;
                    text = String::new();
                };
                if param_s.starts_with(
                    &['-', '0', '1', '2', '3', '4',
                    '5', '6', '7', '8', '9']){
                    match param_s.parse::<f64>(){
                        Ok(value) => Param::Float(value),
                        _ => {
                            return Err(String::from("Incorrect syntax"));
                        }
                    }
                } else {
                    Param::Variable(param_s)
                }
            };
            vec.push(param);
        }
        Ok((result_var, vec))
    }
}