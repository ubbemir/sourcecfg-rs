use crate::Rule;

use crate::error::Error;

pub fn traverse(input: &str) -> Result<String, Error> {
    let file = crate::parse(input)?;
    let mut result = String::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::statements => {
                let stmts = line.into_inner();
                
                for line in stmts {
                    match line.as_rule() {
                        Rule::statement => {
                            let params = line.into_inner();

                            for param in params {
                                match param.as_rule() {
                                    Rule::cvar => {
                                        result.push_str(param.as_str());
                                    },
                                    Rule::param => {
                                        result.push_str(param.as_str());
                                    },
                                    _ => ()
                                }
                                result.push_str(" ");
                            }
                            result.push_str("\n");
                        },
                        _ => ()
                    }
                }
            },
            _ => return Err(Error::FormattingError("Did not find any statements".to_string()))
        }
    }

    Ok(result)
}
