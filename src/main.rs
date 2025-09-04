use std::io;
fn main() {
    println!("Enter a mathematical expression:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let expr = input.trim();
    let tokens = tokenize(expr);
    let rpn = to_rpn(tokens);
    let result = eval_rpn(rpn);

    println!("{} = {}", expr, result);
}



#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Op(char),
    LParen,
    RParen,
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut num_buf = String::new();

    for c in expr.chars() {
        if c.is_ascii_digit() || c == '.' {
            num_buf.push(c);
        } else {
            if !num_buf.is_empty() {
                tokens.push(Token::Number(num_buf.parse().unwrap()));
                num_buf.clear();
            }
            match c {
                '+' | '-' | '*' | '/' => tokens.push(Token::Op(c)),
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                ' ' => continue,
                _ => panic!("Unexpected character: {}", c),
            }
        }
    }

    if !num_buf.is_empty() {
        tokens.push(Token::Number(num_buf.parse().unwrap()));
    }

    tokens
}


fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn to_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut stack = Vec::new();

    for t in tokens {
        match t {
            Token::Number(_) => output.push(t),
            Token::Op(op) => {
                while let Some(Token::Op(top)) = stack.last() {
                    if precedence(*top) >= precedence(op) {
                        output.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(Token::Op(op));
            }
            Token::LParen => stack.push(Token::LParen),
            Token::RParen => {
                while let Some(top) = stack.pop() {
                    if let Token::LParen = top {
                        break;
                    } else {
                        output.push(top);
                    }
                }
            }
        }
    }

    while let Some(op) = stack.pop() {
        output.push(op);
    }

    output
}

fn eval_rpn(tokens: Vec<Token>) -> f64 {
    let mut stack = Vec::new();

    for t in tokens {
        match t {
            Token::Number(n) => stack.push(n),
            Token::Op(op) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let res = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => a / b,
                    _ => panic!("Unknown operator"),
                };
                stack.push(res);
            }
            _ => panic!("Unexpected token in RPN"),
        }
    }

    stack.pop().unwrap()
}
