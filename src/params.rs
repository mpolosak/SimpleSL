use crate::variable::is_correct_variable_name;

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
        let result_var = match get_result_var(&mut text) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

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
                if is_correct_variable_name(&param_s){
                    Param::Variable(param_s)
                } else if let Ok(value) = param_s.parse::<f64>(){
                    Param::Float(value)
                } else {
                    return Err(format!("{} isn't correct variable name", param_s))
                }
            };
            vec.push(param);
        }
        Ok((result_var, vec))
    }
}

fn get_result_var(text: &mut String) -> Result<Option<String>, String>{
    if let Some((before, after)) = text.split_once("=") {
        if after.contains("="){
            return Err(String::from("Too many = in one line"))
        }
        let before_s = before.trim().to_string();
        if before_s.contains(" ") || before_s.is_empty() {
            return Err(String::from("Before = should be exactly one variable"))
        }
        if !is_correct_variable_name(&before_s){
            return Err(format!("{} isn't correct variable name", before_s));
        }
        *text = String::from(after);
        Ok(Some(before_s))
    } else { Ok(None) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_get_result_var() {
        use crate::params::get_result_var;
        let mut text = String::from(" x = 14");
        assert_eq!(get_result_var(&mut text), Ok(Some(String::from("x"))));
        text = String::from(" x = 5 = 6");
        assert_eq!(get_result_var(&mut text), Err(String::from("Too many = in one line")));
        text = String::from(" x y = 6");
        assert_eq!(get_result_var(&mut text),
            Err(String::from("Before = should be exactly one variable")));
        text = String::from(" 7 = 6");
        assert_eq!(get_result_var(&mut text),
            Err(String::from("7 isn't correct variable name")));
    }
}