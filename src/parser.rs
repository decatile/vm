#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub index: usize,
    pub value: TokenValue,
}

#[derive(Clone, Copy, Debug)]
pub enum TokenValue {
    LP,
    RP,
    Op(OpType),
    Num(f64),
}

#[derive(Clone, Copy, Debug)]
pub enum OpType {
    Add,
    Sub,
    Mul,
    Div,
}

impl OpType {
    fn try_from(value: char) -> Option<Self> {
        match value {
            '+' => Some(OpType::Add),
            '-' => Some(OpType::Sub),
            '*' => Some(OpType::Mul),
            '/' => Some(OpType::Div),
            _ => None,
        }
    }
}

enum State {
    Empty,
    LeadingDot,
    Number(String),
}

pub fn parse(expr: &str) -> ParseResult {
    let mut state = State::Empty;
    let mut tokens = vec![];
    for (index, c) in expr.char_indices() {
        match state {
            State::Empty => {
                if let Some(op) = OpType::try_from(c) {
                    tokens.push(Token {
                        index,
                        value: TokenValue::Op(op),
                    })
                } else if c.is_ascii_digit() {
                    state = State::Number(c.to_string())
                } else if c == '.' {
                    state = State::LeadingDot;
                } else if c == '(' {
                    tokens.push(Token {
                        index,
                        value: TokenValue::LP,
                    })
                } else if c == ')' {
                    tokens.push(Token {
                        index,
                        value: TokenValue::RP,
                    })
                } else if c != ' ' {
                    Err(ParseError {
                        index,
                        value: ParseErrorValue::UnexpectedCharacter,
                    })?
                }
            }
            State::Number(ref mut num) => {
                if let Some(op) = OpType::try_from(c) {
                    tokens.push(Token {
                        index,
                        value: TokenValue::Num(num.parse::<f64>().unwrap()),
                    });
                    tokens.push(Token {
                        index,
                        value: TokenValue::Op(op),
                    });
                    state = State::Empty;
                } else if c.is_ascii_digit() {
                    num.push(c);
                } else if c == '.' {
                    if num.contains('.') {
                        Err(ParseError {
                            index,
                            value: ParseErrorValue::MultipleDots,
                        })?
                    }
                    num.push(c)
                } else if c == '(' {
                    tokens.push(Token {
                        index,
                        value: TokenValue::Num(num.parse::<f64>().unwrap()),
                    });
                    tokens.push(Token {
                        index,
                        value: TokenValue::LP,
                    });
                    state = State::Empty;
                } else if c == ')' {
                    tokens.push(Token {
                        index,
                        value: TokenValue::Num(num.parse::<f64>().unwrap()),
                    });
                    tokens.push(Token {
                        index,
                        value: TokenValue::RP,
                    });
                    state = State::Empty;
                } else if c == ' ' {
                    tokens.push(Token {
                        index,
                        value: TokenValue::Num(num.parse::<f64>().unwrap()),
                    });
                    state = State::Empty
                } else {
                    Err(ParseError {
                        index,
                        value: ParseErrorValue::UnexpectedCharacter,
                    })?
                }
            }
            State::LeadingDot => {
                if c.is_ascii_digit() {
                    state = State::Number(c.to_string());
                } else if c == '.' {
                    Err(ParseError {
                        index,
                        value: ParseErrorValue::MultipleDots,
                    })?
                } else {
                    Err(ParseError {
                        index,
                        value: ParseErrorValue::SingleDot,
                    })?
                }
            }
        }
    }
    match state {
        State::Empty => {}
        State::LeadingDot => Err(ParseError {
            index: expr.len() - 1,
            value: ParseErrorValue::SingleDot,
        })?,
        State::Number(num) => tokens.push(Token {
            index: expr.len() - num.len(),
            value: TokenValue::Num(num.parse::<f64>().unwrap()),
        }),
    }
    Ok(tokens)
}

pub type ParseResult = Result<Vec<Token>, ParseError>;

#[derive(Clone, Copy, Debug)]
pub struct ParseError {
    index: usize,
    value: ParseErrorValue,
}

#[derive(Clone, Copy, Debug)]
pub enum ParseErrorValue {
    UnexpectedCharacter,
    SingleDot,
    MultipleDots,
}
