extern crate regex;

use std::io::Read;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add, Mul, Init, Err,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    
    let mut sum = 0;
    let mut result = Vec::<(i64, Op)>::new();
    for line in input.lines() {
        result.push((0, Op::Init));
        for c in line.chars() {
            match c {
                c if '0' <= c && c <= '9' => {
                    let value = c.to_digit(10).unwrap() as i64;
                    apply_op(result.last_mut().unwrap(), value);
                }
                '(' => {
                    result.push((0, Op::Init));
                }
                ')' => {
                    let value = result.pop().unwrap().0;
                    apply_op(result.last_mut().unwrap(), value);
                }
                '+' => {
                    result.last_mut().unwrap().1 = Op::Add;
                }
                '*' => {
                    result.last_mut().unwrap().1 = Op::Mul;
                }
                ' ' => {},
                _ => { panic!(); }
            }
        }
        sum += result.pop().unwrap().0;
        assert!(result.is_empty());
    }
    println!("{}", sum);

    let mut sum = 0;
    for line in input.lines() {
        match parse_expr(line) {
            Ok((rest, expr)) => {
                if rest.is_empty() {
                    sum += eval_expr2(&expr);
                } else {
                    panic!("Rest not empty `{}`", rest);
                }
            }
            Err(e) => { panic!("{}", e); }
        }
    }
    println!("{}", sum);
}

fn apply_op(lhs: &mut (i64, Op), rhs: i64) {
    match lhs.1 {
        Op::Init => { lhs.0 = rhs; }
        Op::Mul => { lhs.0 *= rhs; }
        Op::Add => { lhs.0 += rhs; }
        Op::Err => { unreachable!(); }
    }
    lhs.1 = Op::Err;
}

fn eval_expr2(expr: &Expr) -> i64 {
    match expr {
        Expr::Add(terms) => terms.iter().map(|t| eval_expr2(t)).sum(),
        Expr::Mul(terms) => terms.iter().map(|t| eval_expr2(t)).product(),
        Expr::Num(n) => *n,
    }
}

// Expression parser
//////////////////////

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Add(Vec<Expr>),
    Mul(Vec<Expr>),
    Num(i64),
}

fn parse_expr(input: &str) -> ParseResult<Expr> {
    parse_mul(input)
}

fn parse_mul(input: &str) -> ParseResult<Expr> {
    let (input, lhs) = parse_add(input)?;
    let (input, mut result) = parse_rep0(input, |input| {
        let (input, ()) = parse_char_eq(input, ' ')?;
        let (input, ()) = parse_char_eq(input, '*')?;
        let (input, ()) = parse_char_eq(input, ' ')?;
        parse_add(input)
    })?;

    if result.is_empty() {
        Ok((input, lhs))
    } else {
        result.insert(0, lhs);
        Ok((input, Expr::Mul(result)))
    }
}

fn parse_add(input: &str) -> ParseResult<Expr> {
    let (input, lhs) = parse_prim(input)?;
    let (input, mut result) = parse_rep0(input, |input| {
        let (input, ()) = parse_char_eq(input, ' ')?;
        let (input, ()) = parse_char_eq(input, '+')?;
        let (input, ()) = parse_char_eq(input, ' ')?;
        parse_prim(input)
    })?;

    if result.is_empty() {
        Ok((input, lhs))
    } else {
        result.insert(0, lhs);
        Ok((input, Expr::Add(result)))
    }
}

fn parse_prim(input: &str) -> ParseResult<Expr> {
    parse_or(input, &[
        &parse_num,
        &parse_subexpr,
    ])
}

fn parse_num(input: &str) -> ParseResult<Expr> {
    let (input, c) = parse_char(input)?;
    if let Some(n) = c.to_digit(10) {
        Ok((input, Expr::Num(n as i64)))
    } else {
        Err("invalid digit".to_owned())
    }
}

fn parse_subexpr(input: &str) -> ParseResult<Expr> {
    let (input, ()) = parse_char_eq(input, '(')?;
    let (input, result) = parse_expr(input)?;
    let (input, ()) = parse_char_eq(input, ')')?;
    Ok((input, result))
}

// Parser primitives
//////////////////////

type ParseResult<'a, T> = Result<(&'a str, T), String>;

fn parse_rep0<T, Parser: FnMut(&str) -> ParseResult<T>>(mut input: &str, mut parser: Parser) -> ParseResult<Vec<T>> {
    let mut result = Vec::new();
    while let Ok((new_input, value)) = parser(input) {
        result.push(value);
        input = new_input;
    }
    Ok((input, result))
}

fn parse_or<'a, T>(input: &'a str, options: &[&dyn Fn(&'a str) -> ParseResult<'a, T>]) -> ParseResult<'a, T> {
    for option in options {
        if let Ok((input, result)) = option(input) {
            return Ok((input, result));
        }
    }
    Err("out of options".to_owned())
}

fn parse_char_eq(input: &str, c: char) -> ParseResult<()> {
    let (input, inc) = parse_char(input)?;
    if c == inc {
        Ok((input, ()))
    } else {
        Err(format!("expected {} but found {}", c, inc))
    }
}

fn parse_char(input: &str) -> ParseResult<char> {
    if input.is_empty() {
        Err("end of input".to_owned())
    } else {
        Ok((&input[1..], input.chars().nth(0).unwrap()))
    }
}
