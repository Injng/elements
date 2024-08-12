use crate::lang::functions;
use crate::lang::types::{Operation, Value};
use std::fmt::{Debug, Error, Formatter};

#[derive(Debug, PartialEq)]
pub enum Token {
    Variable(Variable),
    Literal(Literal),
    Function(Function),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    name: String,
    var: Value,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub value: Value,
}

pub struct Function {
    name: String,
    args: Vec<Token>,
    function: Box<dyn Operation>,
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

        // basic geometric functions
        "point" => Function {
            name,
            args: Vec::new(),
            function: Box::new(functions::FnPoint),
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
pub fn tokenize(s: String) -> Vec<Token> {
    // split the string into a vector of strings based on whitespace
    let separated: Vec<String> = s
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(String::from)
        .collect();

    // match the tokens
    let mut tokens: Vec<Token> = Vec::new();
    let mut prev_paren = false;
    for word in separated {
        let token: Token = match_token(word, prev_paren);
        prev_paren = token == Token::LeftParen;
        tokens.push(token);
    }

    tokens
}
