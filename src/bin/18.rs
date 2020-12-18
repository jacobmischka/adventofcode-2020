use std::io::{self, BufRead};

fn main() {
    let mut p1_sum = 0;
    let mut p2_sum = 0;

    for line in io::stdin().lock().lines().filter_map(Result::ok) {
        let mut p1_stack = Vec::new();
        let mut p2_stack = Vec::new();

        for token in line
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Token::from_char)
            .filter_map(Result::ok)
        {
            add_token(&mut p1_stack, token);
            reduce_stack_unprecedented(&mut p1_stack).expect("stack exhaustion");

            p2_stack.push(token);
            reduce_stack_precedented(&mut p2_stack).expect("stack exhaustion");
        }

        reduce_stack_unprecedented(&mut p2_stack).expect("stack exhaustion");

        assert!(p1_stack.len() == 1);
        assert!(p2_stack.len() == 1);

        match p1_stack.pop() {
            Some(Token::Val(val)) => {
                p1_sum += val;
            }
            t => {
                panic!("unexpected token: {:?}", t)
            }
        }
        match p2_stack.pop() {
            Some(Token::Val(val)) => {
                p2_sum += val;
            }
            t => {
                panic!("unexpected token: {:?}", t)
            }
        }
    }

    println!("Part 1: {}", p1_sum);
    println!("Part 2: {}", p2_sum);
}

fn add_token(stack: &mut Vec<Token>, token: Token) {
    if token == Token::CloseParen {
        let mut tmp: Vec<Token> = Vec::new();
        while let Some(token) = stack.pop() {
            if token == Token::OpenParen {
                for t in tmp.into_iter() {
                    stack.push(t);
                }
                break;
            } else {
                tmp.push(token);
            }
        }
    } else {
        stack.push(token);
    };
}

fn reduce_stack_precedented(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.is_empty() {
        return Ok(());
    }

    if stack.contains(&Token::OpenParen) {
        if let Some(Token::CloseParen) = stack.last() {
            stack.pop();

            let mut tmp = Vec::new();
            while let Some(t) = stack.pop() {
                if t == Token::OpenParen {
                    break;
                }
                tmp.push(t);
            }

            reduce_stack_precedented(&mut tmp)?;
            reduce_stack_unprecedented(&mut tmp)?;

            for t in tmp.into_iter().rev() {
                stack.push(t);
            }

            reduce_stack_precedented(stack)?;
        }
    } else {
        while let Some((i, _)) = stack
            .iter()
            .enumerate()
            .find(|(_, t)| **t == Token::Operation(Operation::Add))
        {
            if i == 0 || i == stack.len() - 1 {
                break;
            }

            let mut rest = stack.split_off(i + 2);
            let lhs = match stack.pop() {
                Some(Token::Val(x)) => Ok(x),
                x => Err(format!("unexpected addition lhs: {:?}", x)),
            }?;
            let add = match stack.pop() {
                Some(Token::Operation(add)) => Ok(add),
                x => Err(format!("unexpected addition op: {:?}", x)),
            }?;
            let rhs = match stack.pop() {
                Some(Token::Val(x)) => Ok(x),
                x => Err(format!("unexpected addition rhs: {:?}", x)),
            }?;

            stack.push(Token::Val(add.perform(lhs, rhs)));
            stack.append(&mut rest);
        }
    }

    Ok(())
}

fn reduce_stack_unprecedented(stack: &mut Vec<Token>) -> Result<(), String> {
    if stack.is_empty() {
        return Ok(());
    }

    let mut token = stack.pop().ok_or("stack exhaustion".to_string())?;

    loop {
        match token {
            Token::Val(val) => match stack.pop() {
                Some(Token::Operation(op)) => match stack.pop() {
                    Some(Token::Val(lhs)) => {
                        token = Token::Val(op.perform(lhs, val));
                    }
                    Some(t) => {
                        stack.push(t);
                        stack.push(Token::Operation(op));
                        stack.push(token);
                        break;
                    }
                    None => {
                        stack.push(Token::Operation(op));
                        stack.push(token);
                        break;
                    }
                },
                Some(t) => {
                    stack.push(t);
                    stack.push(token);
                    break;
                }
                None => {
                    stack.push(token);
                    break;
                }
            },
            t => {
                stack.push(t);
                break;
            }
        }
    }

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Token {
    Val(u64),
    Operation(Operation),
    OpenParen,
    CloseParen,
}

impl Token {
    fn from_char(c: char) -> Result<Token, String> {
        if let Ok(op) = Operation::from_char(c) {
            Ok(Token::Operation(op))
        } else if c.is_digit(10) {
            Ok(Token::Val(match c {
                '0' => Ok(0),
                '1' => Ok(1),
                '2' => Ok(2),
                '3' => Ok(3),
                '4' => Ok(4),
                '5' => Ok(5),
                '6' => Ok(6),
                '7' => Ok(7),
                '8' => Ok(8),
                '9' => Ok(9),
                c => Err(format!("invalid value: {}", c)),
            }?))
        } else {
            match c {
                '(' => Ok(Token::OpenParen),
                ')' => Ok(Token::CloseParen),
                c => Err(format!("unknown token: {}", c)),
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn from_char(c: char) -> Result<Operation, String> {
        match c {
            '+' => Ok(Operation::Add),
            '*' => Ok(Operation::Mul),
            c => Err(format!("invalid operation: {}", c)),
        }
    }

    fn perform(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operation::Add => lhs + rhs,
            Operation::Mul => lhs * rhs,
        }
    }
}
