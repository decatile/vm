use crate::parser::{OpType, Token, TokenValue};

pub enum Expr {
    Number(f64),
    Binary(Box<Expr>, Box<Expr>, OpType),
}

enum IntermediateExpr {
    Owned(Expr),
    Token(Token),
    Empty,
}

pub fn lex<I: IntoIterator<Item = Token>>(tokens: I) -> LexerResult {
    let mut tokens = tokens
        .into_iter()
        .map(|x| IntermediateExpr::Token(x))
        .collect::<Vec<_>>();
    process(&mut tokens)
}

fn process<'a>(tokens: &mut [IntermediateExpr]) -> LexerResult {
    while let Some((lpar_index, rpar_index)) = find_parenthesises(&tokens)? {
        tokens[lpar_index] = IntermediateExpr::Empty;
        tokens[rpar_index] = IntermediateExpr::Empty;
        process(&mut tokens[lpar_index + 1..rpar_index])?;
    }
    todo!()
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
            if let IntermediateExpr::Token(Token {
                value: TokenValue::RP,
                ..
            }) = x
            {
                true
            } else {
                false
            }
        });
        if let Some(offset) = offset {
            Ok(Some((lpar_index, offset + lpar_index + 1)))
        } else {
            Err(LexError {
                token: lpar_token.clone(),
                value: LexErrorValue::UnmatchedParenthesis,
            })
        }
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
}
