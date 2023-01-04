use crate::variable::*;
use std::str::FromStr;
use crate::Intepreter;

pub fn parse(mut text: String, intepreter: &mut Intepreter) -> Result<Variable, String> {
    text = text.trim().to_string();
    if text.starts_with('{') || (!text.starts_with('"') && text.contains('(')) {
        let result = get_var(&mut text, intepreter)?;
        if text.is_empty() {
            Ok(result)
        } else {
            Err(String::from("Syntax error"))
        }
    } else if is_correct_variable_name(&text){
        match intepreter.variables.get(&text) {
            Some(variable) => Ok(variable.clone()),
            _ => return Err(format!("Variable {} doesn't exist", text)),
        }
    } else {
        Variable::from_str(&text)
    }
}

pub fn get_result_var(text: &mut String) -> Result<Option<String>, String>{
    if let Some((before, after)) = text.split_once("=") {
        let before_s = before.trim().to_string();
        if before_s.is_empty() {
            Err(String::from("Before = should be variable name"))
        } else if is_correct_variable_name(&before_s){
            *text = String::from(after);
            Ok(Some(before_s))
        } else {
            Ok(None)
        }
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

fn get_var(text: &mut String, intepreter: &mut Intepreter)->Result<Variable, String>{
    if text.starts_with('"'){
        let result = get_text(text)?;
        Ok(Variable::Text(result))
    } else if text.starts_with('{'){
        let array_text = get_array_literal(text)?;
        let array = get_all_vars(array_text, intepreter)?;
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
                let function = match intepreter.variables.get(&String::from(function_name)) {
                    Some(Variable::Function(func)) => func.clone(),
                    Some(_) => return Err(format!("Variable {} isn't function", function_name)),
                    _ => return Err(format!("Variable {} doesn't exist", function_name)),
                };

                *text = "(".to_owned()+rest;

                let args_string = get_args_string(text)?;
                let args = get_all_vars(args_string, intepreter)?;

                function(intepreter, args)
            } else {
                Err(String::from("Syntax error"))
            }
        } else if is_correct_variable_name(&var_s){
            match intepreter.variables.get(&var_s) {
                Some(variable) => Ok(variable.clone()),
                _ => return Err(format!("Variable {} doesn't exist", text)),
            }
        } else {
            Variable::from_str(&var_s)
        }
    }
}

fn get_all_vars(mut text: String, intepreter: &mut Intepreter)->Result<Array, String>{
    let mut vars = Array::new();
    while !text.is_empty() {
        let var = get_var(&mut text, intepreter)?;
        vars.push(var);
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
        text = String::from("  = 6");
        assert_eq!(get_result_var(&mut text),
            Err(String::from("Before = should be variable name")));
        text = String::from(r#""x=7""#);
        assert_eq!(get_result_var(&mut text), Ok(None));
        text = String::from("print(5)");
        assert_eq!(get_result_var(&mut text), Ok(None));
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