use crate::variable::*;
use std::str::FromStr;

pub fn parse(mut text: String, variables: &VariableMap) -> Result<Array, String> {
    let mut vec = Array::new();
    while text.len()>0 {
        text = text.trim().to_string();
        let param = if text.starts_with('"') {
            let string = match get_text(&mut text){
                Ok(value) => value,
                Err(e) => {
                    return Err(e)
                },
            };
            Variable::Text(string)
        } else if text.starts_with('{') {
            let array_text = match get_array_literal(&mut text) {
                Ok(value) => value,
                Err(e) => return Err(e)
            };
            let array = match parse(array_text, &variables) {
                Ok(value) => value,
                Err(e) => return Err(e)
            };
            Variable::Array(array)
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
                match variables.get(&param_s) {
                    Some(variable) => variable.clone(),
                    _ => return Err(format!("Variable {} doesn't exist", param_s)),
                }
            } else {
                match Variable::from_str(&param_s){
                    Ok(variable) => variable,
                    Err(e) => return Err(e),
                }
            }
        };
        vec.push(param);
    }
    Ok(vec)
}

pub fn get_result_var(text: &mut String) -> Result<Option<String>, String>{
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

pub fn get_text(text: &mut String) -> Result<String, String>{
    text.remove(0);
    let mut result = String::new();
    loop{
        if text.len() == 0 {
            return Err(String::from("Mismatching quotation marks"))
        };
        match text.remove(0) {
            '\\' => {
                if text.len() == 0 {
                    return Err(String::from("Mismatching quotation marks"))
                };
                match text.remove(0) {
                    '\\' => {
                        result.push('\\');
                    }
                    '"' => {
                        result.push('"');
                    }
                    'n' => {
                        result.push('\n');
                    }
                    _ => {
                        return Err(String::from("Incorrect syntax"));
                    }
                }
            }
            '"' => {
                if text.len()==0 {
                    return Ok(result);
                };
                if text.remove(0) != ' ' {
                    return Err(String::from("Incorrect syntax"));
                }
                return Ok(result);
            }
            other => {
                result.push(other);
            }
        }
    }
}

fn get_array_literal(text: &mut String) -> Result<String, String> {
    text.remove(0);
    let mut result = String::new();
    let mut level = 1;
    loop{
        if text.len() == 0 {
            return Err(String::from("Mismatching array brackets"));
        }
        match text.remove(0) {
            '{' => {
                level += 1;
                result.push('{');
            }
            '}' => {
                level-=1;
                if level > 0 {
                    result.push('}'); 
                    continue
                }
                if text.len()==0 {
                    return Ok(result);
                };
                if text.remove(0) != ' ' {
                    return Err(String::from("Incorrect syntax"));
                }
                return Ok(result);
            }
            ch => {
                result.push(ch);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_get_result_var() {
        use crate::parse::get_result_var;
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
    #[test]
    fn check_get_text() {
        use crate::parse::get_text;
        let mut text = String::from(r#""print""#);
        assert_eq!(get_text(&mut text), Ok(String::from("print")));
        text = String::from(r#""print" "#);
        assert_eq!(get_text(&mut text), Ok(String::from("print")));
        text = String::from(r#""print\n""#);
        assert_eq!(get_text(&mut text), Ok(String::from("print\n")));
        text = String::from(r#""print\\n""#);
        assert_eq!(get_text(&mut text), Ok(String::from("print\\n")));
        text = String::from(r#""print\"""#);
        assert_eq!(get_text(&mut text), Ok(String::from("print\"")));
        text = String::from(r#""print\r""#);
        assert_eq!(get_text(&mut text), Err(String::from("Incorrect syntax")));
        text = String::from(r#""print"s"#);
        assert_eq!(get_text(&mut text), Err(String::from("Incorrect syntax")));
        text = String::from(r#""print"#);
        assert_eq!(get_text(&mut text),
            Err(String::from("Mismatching quotation marks")));
    }
    #[test]
    fn check_get_array_literal(){
        use crate::parse::get_array_literal;
        let mut text = String::from(r#"{x 45} addd fg"#);
        assert_eq!(get_array_literal(&mut text), Ok(String::from("x 45")));
        text = String::from(r#"{x {45 {6}}} addd fg"#);
        assert_eq!(get_array_literal(&mut text), Ok(String::from("x {45 {6}}")));
        text = String::from(r#"{x {45 {6} addd fg"#);
        assert_eq!(get_array_literal(&mut text), Err(String::from("Mismatching array brackets")));
        text = String::from(r#"{x {45 {6}}}c addd fg"#);
        assert_eq!(get_array_literal(&mut text), Err(String::from("Incorrect syntax")));
    }
}