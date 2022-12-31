use crate::variable::*;
use std::str::FromStr;

pub fn parse(mut text: String, variables: &mut VariableMap) -> Result<Variable, String> {
    text = text.trim().to_string();
    if text.starts_with('{') || (!text.starts_with('"') && text.contains('(')) {
        let result = match get_var(&mut text, variables){
            Ok(value) => value,
            Err(e) => {
                return Err(e);
            }
        };

        if text.is_empty() {
            Ok(result)
        } else {
            Err(String::from("Syntax error"))
        }
    } else if is_correct_variable_name(&text){
        match variables.get(&text) {
            Some(variable) => Ok(variable.clone()),
            _ => return Err(format!("Variable {} doesn't exist", text)),
        }
    } else {
        Variable::from_str(&text)
    }
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

fn get_args_string(text: &mut String) -> Result<String, String> {
    text.remove(0);
    let mut result = String::new();
    let mut level = 1;
    loop{
        if text.len() == 0 {
            return Err(String::from("Mismatching brackets"));
        }
        match text.remove(0) {
            '(' => {
                level += 1;
                result.push('(');
            }
            ')' => {
                level-=1;
                if level > 0 {
                    result.push(')'); 
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

fn get_var(text: &mut String, variables: &mut VariableMap)->Result<Variable, String>{
    if text.starts_with('"'){
        match get_text(text) {
            Ok(value) => Ok(Variable::Text(value)),
            Err(e) => Err(e)
        }
    } else if text.starts_with('{'){
        let mut array_text = match get_array_literal(text) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        let mut array = match get_all_vars(array_text, variables){
            Ok(value) => value,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Variable::Array(array))
    } else {
        let var_s;
        if let Some((begin, rest)) = text.split_once(" "){
            var_s = String::from(begin);
            *text = String::from(rest);
        } else {
            var_s = text.clone();
            *text = String::new();
        };

        if var_s.contains('('){
            *text = format!("{} {}", var_s,  text);
            if let Some((function_name, rest)) = text.split_once("("){
                let function = match variables.get(&String::from(function_name)) {
                    Some(Variable::Function(func)) => func.clone(),
                    Some(_) => return Err(format!("Variable {} isn't function", function_name)),
                    _ => return Err(format!("Variable {} doesn't exist", function_name)),
                };

                *text = "(".to_owned()+rest;

                let args_string = match get_args_string(text){
                    Ok(value) => value,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let args = match get_all_vars(args_string, variables){
                    Ok(value) => value,
                    Err(e) => {
                        return Err(e);
                    }
                };

                function(variables, args)
            } else {
                Err(String::from("Syntax error"))
            }
        } else if is_correct_variable_name(&var_s){
            match variables.get(&var_s) {
                Some(variable) => Ok(variable.clone()),
                _ => return Err(format!("Variable {} doesn't exist", text)),
            }
        } else {
            Variable::from_str(&var_s)
        }
    }
}

fn get_all_vars(mut text: String, variables: &mut VariableMap)->Result<Array, String>{
    let mut vars = Array::new();
    while !text.is_empty() {
        match get_var(&mut text, variables){
            Ok(var) => {
                vars.push(var);
            }
            Err(e) => {
                return Err(e);
            }
        }
        text = text.trim().to_string();
    }
    Ok(vars)
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