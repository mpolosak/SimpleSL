pub type ParamVec = Vec<Param>;

#[derive(Clone, Debug)]
pub enum Param{
    Float(f64),
    Text(String),
    Variable(String)
}

pub trait Parse {
    fn parse(text: String) -> Self;
}

impl Parse for ParamVec {
    fn parse(mut text: String) -> ParamVec {
        let mut vec = ParamVec::new();
        while text.len()>0 {
            text = text.trim().to_string();
            let param = if text.starts_with('"') {
                text.remove(0);
                text.push(' ');
                if let Some((begin, rest)) = text.split_once("\" "){
                    let value = String::from(begin);
                    if value.contains("\""){
                        eprintln!("Incorrect syntax");
                        return ParamVec::new();
                    }
                    text = String::from(rest);
                    Param::Text(value)
                } else {
                    eprintln!("Incorrect syntax");
                    return ParamVec::new();
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
                            eprintln!("Incorrect syntax");
                            return ParamVec::new();
                        }
                    }
                } else {
                    Param::Variable(param_s)
                }
            };
            vec.push(param);
        }
        vec
    }
}