use std::mem;

use crate::parser::{OpType, Token, TokenValue};

#[derive(Clone, Debug)]
pub enum Expr {
    Number(f64),
    Binary(OpType, Box<Expr>, Box<Expr>),
}

enum IntermediateExpr {
    Owned(Expr),
    Token(Token),
    Empty,
}

pub fn lex<I: IntoIterator<Item = Token>>(tokens: I) -> LexerResult {
    let mut tokens = tokens
        .into_iter()
        .map(|x| {
            if let TokenValue::Num(num) = x.value {
                IntermediateExpr::Owned(Expr::Number(num))
            } else {
                IntermediateExpr::Token(x)
            }
        })
        .collect::<Vec<_>>();
    process(&mut tokens)?;
    Ok(tokens
        .into_iter()
        .find_map(|x| {
            if let IntermediateExpr::Owned(expr) = x {
                Some(expr)
            } else {
                None
            }
        })
        .unwrap())
}

fn process(tokens: &mut [IntermediateExpr]) -> Result<(), LexError> {
    while let Some((lpar_index, rpar_index)) = find_parenthesises(tokens)? {
        tokens[lpar_index] = IntermediateExpr::Empty;
        tokens[rpar_index] = IntermediateExpr::Empty;
        process(&mut tokens[lpar_index + 1..rpar_index])?;
    }
    while let Some((op_index, arg_indexes)) = find_expr(tokens)? {
        if let IntermediateExpr::Token(
            token @ Token {
                value: TokenValue::Op(op),
                ..
            },
        ) = tokens[op_index]
        {
            if arg_indexes.len() != 2 {
                Err(LexError {
                    token,
                    value: LexErrorValue::InvalidNumberOfArguments,
                })?
            }
            let lhs = mem::replace(&mut tokens[arg_indexes[0]], IntermediateExpr::Empty);
            let rhs = mem::replace(&mut tokens[arg_indexes[1]], IntermediateExpr::Empty);
            if let (IntermediateExpr::Owned(lhs), IntermediateExpr::Owned(rhs)) = (lhs, rhs) {
                tokens[op_index] =
                    IntermediateExpr::Owned(Expr::Binary(op, Box::new(lhs), Box::new(rhs)));
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
    Ok(())
}

fn find_parenthesises(tokens: &[IntermediateExpr]) -> Result<Option<(usize, usize)>, LexError> {
    let lpar = tokens.iter().enumerate().find_map(|(i, x)| {
        if let IntermediateExpr::Token(
            token @ Token {
                value: TokenValue::LP,
                ..
            },
        ) = x
        {
            Some((i, token))
        } else {
            None
        }
    });
    if let Some((lpar_index, lpar_token)) = lpar {
        let offset = tokens.iter().skip(lpar_index + 1).position(|x| {
            matches!(
                x,
                IntermediateExpr::Token(Token {
                    value: TokenValue::RP,
                    ..
                })
            )
        });
        if let Some(offset) = offset {
            Ok(Some((lpar_index, offset + lpar_index + 1)))
        } else {
            Err(LexError {
                token: *lpar_token,
                value: LexErrorValue::UnmatchedParenthesis,
            })
        }
    } else {
        Ok(None)
    }
}

fn find_expr(tokens: &[IntermediateExpr]) -> Result<Option<(usize, Vec<usize>)>, LexError> {
    let op = tokens.iter().position(|x| {
        matches!(
            x,
            IntermediateExpr::Token(Token {
                value: TokenValue::Op(..),
                ..
            })
        )
    });
    if let Some(op) = op {
        let args = tokens
            .iter()
            .skip(op + 1)
            .enumerate()
            .filter_map(|(index, x)| {
                if let IntermediateExpr::Owned(..) = x {
                    Some(index + op + 1)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Ok(Some((op, args)))
    } else {
        Ok(None)
    }
}

pub type LexerResult = Result<Expr, LexError>;

#[derive(Clone, Copy, Debug)]
pub struct LexError {
    token: Token,
    value: LexErrorValue,
}

#[derive(Clone, Copy, Debug)]
pub enum LexErrorValue {
    UnmatchedParenthesis,
    InvalidNumberOfArguments,
}
