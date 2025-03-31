use crate::Rule;

use crate::error::Error;

pub fn traverse(input: &str) -> Result<String, Error> {
    let file = crate::parse(input)?;

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
                                        print!("{}", param.as_str())
                                    },
                                    Rule::param => {
                                        print!("{}", param.as_str())
                                    },
                                    _ => ()
                                }
                                print!(" ");
                            }
                            println!("")
                        },
                        _ => ()
                    }
                }
            },
            _ => return Err(Error::FormattingError("Did not find any statements".to_string()))
        }
    }

    todo!()
}
