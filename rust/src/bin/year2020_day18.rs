use aocsuite::utils;
fn exercise1(data: &str) -> i64 {
    return data.lines().map(|expr| evaluate_expression(&expr, 1)).sum();
}
fn exercise2(data: &str) -> i64 {
    return data.lines().map(|expr| evaluate_expression(&expr, 2)).sum();
}

fn main() {
    utils::run(exercise1, exercise2);
}

fn evaluate_expression(expr: &str, exercise: i32) -> i64 {
    let tokens = tokenize(expr, exercise);
    let mut output_queue = Vec::new();
    let mut operator_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output_queue.push(token),
            Token::Operator(op) => {
                while let Some(top_op) = operator_stack.last() {
                    match top_op {
                        Token::Operator(top_op) => {
                            if top_op.precedence() >= op.precedence() {
                                output_queue.push(operator_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                operator_stack.push(Token::Operator(op));
            }
            Token::LeftParen => operator_stack.push(Token::LeftParen),
            Token::RightParen => {
                while let Some(top_op) = operator_stack.pop() {
                    if matches!(top_op, Token::LeftParen) {
                        break;
                    } else {
                        output_queue.push(top_op);
                    }
                }
            }
        }
    }

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }
    println!("{}", evaluate_rpn(&output_queue));
    evaluate_rpn(&output_queue)
}

fn tokenize(expr: &str, exercise: i32) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_digit(10) {
                        num.push(digit);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            '+' => {
                let add_op = match exercise {
                    1 => Box::new(Add { precedence: 1 }),
                    2 => Box::new(Add { precedence: 2 }),
                    _ => panic!("Invalid exercise number"),
                };
                tokens.push(Token::Operator(add_op));
                chars.next();
            }
            '*' => {
                tokens.push(Token::Operator(Box::new(Mul { precedence: 1 })));
                chars.next();
            }
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ' ' => {
                chars.next();
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    tokens
}

fn evaluate_rpn(tokens: &[Token]) -> i64 {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Operator(op) => {
                let b = stack.pop().expect("Invalid expression");
                let a = stack.pop().expect("Invalid expression");
                let result = op.apply(a, b);
                stack.push(result);
            }
            _ => panic!("Unexpected token in RPN "),
        }
    }

    stack.pop().expect("Invalid RPN expression")
}

enum Token {
    Number(i64),
    Operator(Box<dyn Operator>),
    LeftParen,
    RightParen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Add {
    precedence: i32,
}

struct Mul {
    precedence: i32,
}

trait Operator {
    fn apply(&self, a: i64, b: i64) -> i64;
    fn precedence(&self) -> i32;
}

impl Operator for Add {
    fn precedence(&self) -> i32 {
        self.precedence
    }
    fn apply(&self, a: i64, b: i64) -> i64 {
        a + b
    }
}

impl Operator for Mul {
    fn precedence(&self) -> i32 {
        self.precedence
    }
    fn apply(&self, a: i64, b: i64) -> i64 {
        a * b
    }
}
