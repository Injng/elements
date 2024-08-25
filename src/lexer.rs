use crate::lang::functions;
use crate::lang::types::{Operation, Value};
use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Variable(Variable),
    Literal(Literal),
    Function(Function),
    LeftParen,
    RightParen,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub var: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Literal {
    pub value: Value,
}

pub struct Function {
    pub name: String,
    pub args: Vec<Token>,
    pub function: Box<dyn Operation>,
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            name: self.name.clone(),
            args: self.args.clone(),
            function: self.function.box_clone(),
        }
    }
}

impl Debug for Function {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Function: {}", self.name)
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/// Given the name of a function, return the appropriate function struct
fn match_fn(name: String) -> Function {
    match name.as_str() {
        // basic arithmetic functions
        "+" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnAdd),
        },
        "-" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnSub),
        },
        "*" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnMul),
        },
        "/" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnDiv),
        },

        // setq function
        "setq" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnSet),
        },

        // basic geometric components
        "angle" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnAngle),
        },
        "iangle" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnInscribedAngle),
        },
        "point" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnPoint),
        },
        "midpoint" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnMidpoint),
        },
        "lineseg" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnLineseg),
        },
        "circumcenter" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnCircumcenter),
        },
        "incenter" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnIncenter),
        },
        "orthocenter" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnOrthocenter),
        },
        "centroid" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnCentroid),
        },

        // functions that return properties
        "intersect" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnIntersect),
        },
        "inradius" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnInradius),
        },

        // basic geometric functions
        "circle" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnCircle),
        },
        "triangle" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnTriangle),
        },
        _ => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnNop),
        },
    }
}

/// Given a token string, and whether the previous token was a parentheses, return the appropriate token
fn match_token(token: String, prev_paren: bool) -> Token {
    // if previous token was a left paren, this token must be a function
    if prev_paren {
        return Token::Function(match_fn(token.clone()));
    }

    // otherwise, match for other tokens
    match token.as_str() {
        "(" => Token::LeftParen,
        ")" => Token::RightParen,
        _ => {
            if token.parse::<i32>().is_ok() {
                Token::Literal(Literal {
                    value: Value::Int(token.parse::<i64>().unwrap()),
                })
            } else {
                Token::Variable(Variable {
                    name: token,
                    var: Value::Indeterminate,
                })
            }
        }
    }
}

/// Given a string, tokenize it into a vector of tokens
pub fn tokenize(s: String, is_debug: bool) -> Vec<Token> {
    // split the string into a vector of strings based on whitespace
    let separated: Vec<String> = s
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace(";", " ; ")
        .replace("\n", " \\n ")
        .split_whitespace()
        .map(String::from)
        .collect();

    if is_debug {
        println!("{:?}", separated);
    }

    // match the tokens
    let mut tokens: Vec<Token> = Vec::new();
    let mut prev_paren = false;
    let mut is_comment = false;
    for word in separated {
        // catch comments
        if word == ";" {
            is_comment = true;
            continue;
        }

        // handle comments
        if is_comment {
            if word == "(" || word == ")" || word == "\\n" {
                is_comment = false;
            } else {
                continue;
            }
        }

        // catch newlines
        if word == "\\n" {
            continue;
        }

        // match and push the appropriate token
        let token: Token = match_token(word, prev_paren);
        prev_paren = token == Token::LeftParen;
        tokens.push(token);
    }

    tokens
}
