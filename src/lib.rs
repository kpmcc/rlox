use std::fs;
use std::io;
use std::io::Write;

use crate::interpreter::{evaluate, InterpreterError};
use crate::parser::Parser;
use crate::scanner::new_scanner;

mod interpreter;
mod parser;
mod scanner;
mod token;

fn is_alpha(c: char) -> bool {
    let b = c.is_ascii_alphabetic() || (c == '_');
    b
}

fn is_alphanumeric(c: char) -> bool {
    let an = is_alpha(c) || c.is_ascii_digit();
    an
}

fn error(line: i64, message: String) {
    let s = "";
    report(line, s.to_string(), message);
}

fn report(line: i64, location: String, message: String) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}

fn run(_s: String) {
    let mut scanner = new_scanner(_s);
    let tokens = scanner.scan_tokens();
    //for t in &tokens {
    //    println!("{}", t);
    //}
    let mut parser = Parser::new(tokens);
    let r = parser.parse();
    match r {
        Ok(r) => {
            if let Some(r) = r {
                //println!("Parsed: {}", r);
                let result = evaluate(r);
                match result {
                    Ok(l) => {
                        let s = match l {
                            parser::LiteralType::Float { lit } => {
                                format!("{}", lit)
                            }
                            parser::LiteralType::String { lit } => {
                                format!("\"{}\"", lit)
                            }
                            parser::LiteralType::Bool { lit } => {
                                format!("{}", lit)
                            }
                            parser::LiteralType::Nil { lit } => {
                                format!("nil")
                            }
                        };
                        println!("{}", s)
                    }
                    Err(InterpreterError { tok, msg }) => println!("{}\n[line {}]", msg, tok.line),
                }
            }
        }
        Err(_) => return,
    }
}

pub fn run_file(path: String) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    Ok(run(contents))
}

pub fn run_prompt() -> io::Result<()> {
    loop {
        let mut buffer = String::new();
        print!("> ");
        io::stdout().flush()?;
        let read_len = io::stdin().read_line(&mut buffer)?;
        if read_len == 0 {
            break;
        } else {
            run(buffer.clone());
        }
    }
    Ok(())
}
