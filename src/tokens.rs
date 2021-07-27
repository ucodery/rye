#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TokenType {
    ENDMARKER,
    NAME,
    NUMBER,
    STRING,
    NEWLINE,
    INDENT,
    DEDENT,
    LPAR,
    RPAR,
    LSQB,
    RSQB,
    COLON,
    COMMA,
    SEMI,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    VBAR,
    AMPER,
    LESS,
    GREATER,
    EQUAL,
    DOT,
    PERCENT,
    LBRACE,
    RBRACE,
    EQEQUAL,
    NOTEQUAL,
    LESSEQUAL,
    GREATEREQUAL,
    TILDE,
    CIRCUMFLEX,
    LEFTSHIFT,
    RIGHTSHIFT,
    DOUBLESTAR,
    PLUSEQUAL,
    MINEQUAL,
    STAREQUAL,
    SLASHEQUAL,
    PERCENTEQUAL,
    AMPEREQUAL,
    VBAREQUAL,
    CIRCUMFLEXEQUAL,
    LEFTSHIFTEQUAL,
    RIGHTSHIFTEQUAL,
    DOUBLESTAREQUAL,
    DOUBLESLASH,
    DOUBLESLASHEQUAL,
    AT,
    ATEQUAL,
    RARROW,
    ELLIPSIS,
    COLONEQUAL,
    OP,
    AWAIT,
    ASYNC,
    TYPE_IGNORE,
    TYPE_COMMENT,
    ERRORTOKEN,
    COMMENT,
    NL,
    ENCODING,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub size: Option<usize>,
}

pub fn get_op(c1: &str) -> Option<Token> {
    match c1.get(..3) {
        Some("**=") => Some(Token {
            token_type: TokenType::DOUBLESTAREQUAL,
            size: Some("**=".chars().count()),
        }),
        Some("...") => Some(Token {
            token_type: TokenType::ELLIPSIS,
            size: Some("...".chars().count()),
        }),
        Some("//=") => Some(Token {
            token_type: TokenType::DOUBLESLASHEQUAL,
            size: Some("//=".chars().count()),
        }),
        Some("<<=") => Some(Token {
            token_type: TokenType::LEFTSHIFTEQUAL,
            size: Some("<<=".chars().count()),
        }),
        Some(">>=") => Some(Token {
            token_type: TokenType::RIGHTSHIFTEQUAL,
            size: Some(">>=".chars().count()),
        }),
        _ => match c1.get(..2) {
            Some("!=") => Some(Token {
                token_type: TokenType::NOTEQUAL,
                size: Some("!=".chars().count()),
            }),
            Some("%=") => Some(Token {
                token_type: TokenType::PERCENTEQUAL,
                size: Some("%=".chars().count()),
            }),
            Some("&=") => Some(Token {
                token_type: TokenType::AMPEREQUAL,
                size: Some("&=".chars().count()),
            }),
            Some("**") => Some(Token {
                token_type: TokenType::DOUBLESTAR,
                size: Some("**".chars().count()),
            }),
            Some("*=") => Some(Token {
                token_type: TokenType::STAREQUAL,
                size: Some("*=".chars().count()),
            }),
            Some("+=") => Some(Token {
                token_type: TokenType::PLUSEQUAL,
                size: Some("+=".chars().count()),
            }),
            Some("-=") => Some(Token {
                token_type: TokenType::MINEQUAL,
                size: Some("-=".chars().count()),
            }),
            Some("->") => Some(Token {
                token_type: TokenType::RARROW,
                size: Some("->".chars().count()),
            }),
            Some("//") => Some(Token {
                token_type: TokenType::DOUBLESLASH,
                size: Some("//".chars().count()),
            }),
            Some("/=") => Some(Token {
                token_type: TokenType::SLASHEQUAL,
                size: Some("/=".chars().count()),
            }),
            Some(":=") => Some(Token {
                token_type: TokenType::COLONEQUAL,
                size: Some(":=".chars().count()),
            }),
            Some("<<") => Some(Token {
                token_type: TokenType::LEFTSHIFT,
                size: Some("<<".chars().count()),
            }),
            Some("<=") => Some(Token {
                token_type: TokenType::LESSEQUAL,
                size: Some("<=".chars().count()),
            }),
            Some("<>") => Some(Token {
                token_type: TokenType::NOTEQUAL,
                size: Some("<>".chars().count()),
            }),
            Some("==") => Some(Token {
                token_type: TokenType::EQEQUAL,
                size: Some("==".chars().count()),
            }),
            Some(">=") => Some(Token {
                token_type: TokenType::GREATEREQUAL,
                size: Some(">=".chars().count()),
            }),
            Some(">>") => Some(Token {
                token_type: TokenType::RIGHTSHIFT,
                size: Some(">>".chars().count()),
            }),
            Some("@=") => Some(Token {
                token_type: TokenType::ATEQUAL,
                size: Some("@=".chars().count()),
            }),
            Some("^=") => Some(Token {
                token_type: TokenType::CIRCUMFLEXEQUAL,
                size: Some("^=".chars().count()),
            }),
            Some("|=") => Some(Token {
                token_type: TokenType::VBAREQUAL,
                size: Some("|=".chars().count()),
            }),
            _ => match c1.get(..1) {
                Some("%") => Some(Token {
                    token_type: TokenType::PERCENT,
                    size: Some("%".chars().count()),
                }),
                Some("&") => Some(Token {
                    token_type: TokenType::AMPER,
                    size: Some("&".chars().count()),
                }),
                Some("(") => Some(Token {
                    token_type: TokenType::LPAR,
                    size: Some("(".chars().count()),
                }),
                Some(")") => Some(Token {
                    token_type: TokenType::RPAR,
                    size: Some(")".chars().count()),
                }),
                Some("*") => Some(Token {
                    token_type: TokenType::STAR,
                    size: Some("*".chars().count()),
                }),
                Some("+") => Some(Token {
                    token_type: TokenType::PLUS,
                    size: Some("+".chars().count()),
                }),
                Some(",") => Some(Token {
                    token_type: TokenType::COMMA,
                    size: Some(",".chars().count()),
                }),
                Some("-") => Some(Token {
                    token_type: TokenType::MINUS,
                    size: Some("-".chars().count()),
                }),
                Some(".") => Some(Token {
                    token_type: TokenType::DOT,
                    size: Some(".".chars().count()),
                }),
                Some("/") => Some(Token {
                    token_type: TokenType::SLASH,
                    size: Some("/".chars().count()),
                }),
                Some(":") => Some(Token {
                    token_type: TokenType::COLON,
                    size: Some(":".chars().count()),
                }),
                Some(";") => Some(Token {
                    token_type: TokenType::SEMI,
                    size: Some(";".chars().count()),
                }),
                Some("<") => Some(Token {
                    token_type: TokenType::LESS,
                    size: Some("<".chars().count()),
                }),
                Some("=") => Some(Token {
                    token_type: TokenType::EQUAL,
                    size: Some("=".chars().count()),
                }),
                Some(">") => Some(Token {
                    token_type: TokenType::GREATER,
                    size: Some(">".chars().count()),
                }),
                Some("@") => Some(Token {
                    token_type: TokenType::AT,
                    size: Some("@".chars().count()),
                }),
                Some("[") => Some(Token {
                    token_type: TokenType::LSQB,
                    size: Some("[".chars().count()),
                }),
                Some("]") => Some(Token {
                    token_type: TokenType::RSQB,
                    size: Some("]".chars().count()),
                }),
                Some("^") => Some(Token {
                    token_type: TokenType::CIRCUMFLEX,
                    size: Some("^".chars().count()),
                }),
                Some("{") => Some(Token {
                    token_type: TokenType::LBRACE,
                    size: Some("{".chars().count()),
                }),
                Some("|") => Some(Token {
                    token_type: TokenType::VBAR,
                    size: Some("|".chars().count()),
                }),
                Some("}") => Some(Token {
                    token_type: TokenType::RBRACE,
                    size: Some("}".chars().count()),
                }),
                Some("~") => Some(Token {
                    token_type: TokenType::TILDE,
                    size: Some("~".chars().count()),
                }),
                _ => None,
            },
        },
    }
}
