use crate::lang::functions;
use crate::lang::types::{Operation, Value};
use crate::lexer::{Function, Literal, Token};

/// Given a list of tokens, return a subset with matching parentheses
fn get_section(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    // check if first token is a left paren
    if tokens[0] != Token::LeftParen {
        return Err("Expected left parenthesis".to_string());
    }

    let mut paren_count = 0;
    let mut section: Vec<Token> = Vec::new();
    for token in tokens {
        // find parantheses
        match token {
            Token::LeftParen => {
                paren_count += 1;
                section.push(token);
            }
            Token::RightParen => {
                paren_count -= 1;
                section.push(token);
                if paren_count == 0 {
                    return Ok(section);
                }
            }
            _ => {
                section.push(token);
            }
        }

        // if parantheses are matching, return the subset
        if paren_count == 0 {
            return Ok(section);
        }
    }
    Err("Mismatched parentheses".to_string())
}

/// Given a function with matching parantheses, reduce it to a value
fn reduce(tokens: Vec<Token>) -> Result<Value, String> {
    // check for empty tokens
    if tokens.is_empty() {
        return Err("Empty tokens".to_string());
    }

    // check for a single token
    if tokens.len() == 1 {
        return match &tokens[0] {
            Token::Literal(l) => Ok(l.value.clone()),
            _ => Err("Single token must be a literal".to_string()),
        };
    }

    // check if first token is a left paren
    if tokens[0] != Token::LeftParen {
        return Err("Expected left parenthesis".to_string());
    }

    // get current function
    let mut func: Function;
    match &tokens[1] {
        Token::Function(f) => {
            func = f.clone();
        }
        _ => {
            return Err("Expected function".to_string());
        }
    }

    // iterate through tokens and reduce
    let mut i = 2;
    while i < tokens.len() - 1 {
        match &tokens[i] {
            Token::LeftParen => {
                let section = get_section(tokens[i..].to_vec())?;
                let length = section.len();
                let value = reduce(section)?;
                func.args.push(Token::Literal(Literal { value }));
                i += length;
            }
            Token::Literal(l) => {
                func.args.push(Token::Literal(l.clone()));
                i += 1;
            }
            _ => {
                return Err(format!("Unexpected token: {:?}", tokens[i]));
            }
        }
    }

    // convert function args to value args
    let mut value_args: Vec<Value> = Vec::new();
    for arg in func.args {
        match arg {
            Token::Literal(l) => {
                value_args.push(l.value);
            }
            _ => {
                return Err("Expected literal".to_string());
            }
        }
    }

    // call the function
    match func.function.call(&value_args) {
        Ok(value) => Ok(value),
        Err(e) => Err(e),
    }
}

/// Given a vector of tokens, evaluate it to a vector of values
pub fn evaluate(tokens: Vec<Token>) -> Result<Vec<Value>, String> {
    let mut values: Vec<Value> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::LeftParen => {
                let section = get_section(tokens[i..].to_vec())?;
                let length = section.len();
                let value = reduce(section)?;
                values.push(value);
                i += length;
            }
            Token::Literal(l) => {
                values.push(l.value.clone());
                i += 1;
            }
            _ => {
                return Err("Unexpected token when evaluating".to_string());
            }
        }
    }
    Ok(values)
}
