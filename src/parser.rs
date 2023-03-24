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
                } else if c.is_digit(10) {
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
                    Err((index, ParseError::UnexpectedCharacter(c)))?
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
                } else if c.is_digit(10) {
                    num.push(c);
                } else if c == '.' {
                    if num.contains('.') {
                        Err((index, ParseError::MultipleDots))?
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
                    Err((index, ParseError::UnexpectedCharacter(c)))?
                }
            }
            State::LeadingDot => {
                if c.is_digit(10) {
                    state = State::Number(c.to_string());
                } else if c == '.' {
                    Err((index, ParseError::MultipleDots))?;
                } else {
                    Err((index, ParseError::SingleDot))?;
                }
            }
        }
    }
    match state {
        State::Empty => {}
        State::LeadingDot => Err((expr.len() - 1, ParseError::SingleDot))?,
        State::Number(num) => tokens.push(Token {
            index: expr.len() - num.len(),
            value: TokenValue::Num(num.parse::<f64>().unwrap()),
        }),
    }
    Ok(tokens)
}

pub type ParseResult = Result<Vec<Token>, (usize, ParseError)>;

#[derive(Clone, Copy, Debug)]
pub enum ParseError {
    UnexpectedCharacter(char),
    SingleDot,
    MultipleDots,
}
