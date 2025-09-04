use std::io;
use postgres::{Client, NoTls};

fn main() {
    // Connect to Postgres
    let mut client = Client::connect(
        "host=localhost user=calculi_user password=kaloy dbname=calculi_db",
        NoTls,
    ).expect("Failed to connect to database");

    loop {
        println!("Enter a mathematical expression (or :history / :h to view past calculations, :q to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let expr = input.trim();

        // Commands
        if expr == ":q" {
            println!("Goodbye!");
            break;
        } else if expr == ":history" || expr == ":h" {
            show_history(&mut client);
            continue;
        }

        // Normal expression
        if expr.is_empty() {
            continue;
        }

        let tokens = tokenize(expr);
        let rpn = to_rpn(tokens);
        let result = eval_rpn(rpn);

        println!("{} = {}", expr, result);

        // Save history
        client.execute(
            "INSERT INTO history (expression, result) VALUES ($1, $2)",
            &[&expr, &result.to_string()],
        ).expect("Failed to insert into history");
    }
}

fn show_history(client: &mut Client) {
    println!("--- Calculation History ---");
    for row in client.query("SELECT id, expression, result, created_at FROM history ORDER BY id DESC LIMIT 10", &[])
        .expect("Failed to fetch history") 
    {
        let id: i32 = row.get(0);
        let expr: String = row.get(1);
        let result: String = row.get(2);
        let created_at: chrono::NaiveDateTime = row.get(3);

        println!("[{}] {} = {} ({})", id, expr, result, created_at);
    }
    println!("----------------------------");
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
 