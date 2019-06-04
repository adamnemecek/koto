use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Cons {
    Cons(Box<Cons>, Box<Cons>),
    Symbol(String),
    Number(f64),
    Nil,
}

fn skip_whitespaces(chars: &mut Peekable<Chars>) {
    while let ch = chars.peek() {
        match ch {
            Some(c) => {
                if c.is_whitespace() {
                    chars.next();
                } else {
                    break;
                }
            },
            _ => break,
        }
    }
}

fn read_symbol(chars: &mut Peekable<Chars>) -> Cons {
    let mut name = String::new();
    while let ch = chars.peek() {
        match ch {
            Some(c) => {
                if *c == ')' || c.is_whitespace() {
                    break;
                } else {
                    name.push(chars.next().unwrap());
                }
            },
            _ => break,
        }
    }
    Cons::Symbol(name)
}

fn read_number(chars: &mut Peekable<Chars>) -> Cons {
    let mut num = String::new();
    while let ch = chars.peek() {
        match ch {
            Some(c) => {
                if *c == '.' || c.is_digit(10) {
                    num.push(chars.next().unwrap());
                } else {
                    break;
                }
            },
            _ => break,
        }
    }
    match num.parse::<f64>() {
        Ok(n) => Cons::Number(n),
        Err(e) => panic!("cannot parse '{:?}' as a number: {:?}", num, e),
    }
}

fn read_list_elem(chars: &mut Peekable<Chars>) -> Cons {
    skip_whitespaces(chars);
    let ch = chars.peek();
    match ch {
        Some(')') => {
            chars.next();
            Cons::Nil
        },
        _ => {
            Cons::Cons(Box::new(read_exp(chars)), Box::new(read_list_elem(chars)))
        },
    }
}

fn read_list(chars: &mut Peekable<Chars>) -> Cons {
    chars.next();
    read_list_elem(chars)
}

fn read_exp(chars: &mut Peekable<Chars>) -> Cons {
    skip_whitespaces(chars);
    let ch = chars.peek();
    match ch {
        None => Cons::Nil,
        Some(')') => panic!("unexpected ')'"),
        Some('(') => read_list(chars),
        Some(c) => {
            if (c.is_digit(10)) {
                read_number(chars)
            } else {
                read_symbol(chars)
            }
        },
    }
}

pub fn read(s: String) -> Cons {
    let mut chars = s.chars().peekable();
    read_exp(&mut chars)
}

pub fn print(c: Cons) {
    print!("nil");
}
