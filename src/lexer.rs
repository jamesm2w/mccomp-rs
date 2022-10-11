use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenType {
    Identity, // IDENT [a-zA-Z][a-zA-Z0-9]*
    Assign,   // =

    LBrace, // {
    RBrace, // }
    LParen, // (
    RParen, // )
    Scolon, // ;
    Comma,  // ,

    Integer, // int
    Void,    // void
    Float,   // float
    Bool,    // bool

    Extern, // extern
    IfStmt,     // if
    Else,   // else
    While,  // while
    Return, // return

    IntLit, // [0-9]+
    FloatLit,   // [0-9]+.[0-9]+
    BoolLit,    // "true" | "false"

    LAnd, // ||
    LOr,  // &&
    Plus,       // +
    Minus,      // -
    Mult,       // *
    Div,        // /
    Mod,        // %
    Not,        // !

    Eq, // ==
    Ne, // !=
    Le, // <
    Lt, // <=
    Ge, // >
    Gt, // >=

    EOF,
    INVALID,
}

impl Default for TokenType {
    fn default() -> Self {
        Self::INVALID
    }
}

#[derive(Debug, Default, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line_no: usize,
    pub column_no: usize,
}

#[derive()]
pub struct Lexer {
    line_no: usize,
    column_no: usize,
    token_buffer: Vec<Token>,
    file_name: String,
}

impl Lexer {
    pub fn read(file_name: String) -> Self {
        let mut inst = Self {
            file_name,
            token_buffer: Vec::new(),
            line_no: 0,
            column_no: 0,
        };

        let mut tokens = Vec::new();
        let mut iter = inst.get_iter().peekable();
        let mut token_type = TokenType::INVALID;
        while token_type != TokenType::EOF {
            let token = inst.get_tok(&mut iter);
            token_type = token.token_type.clone();
            tokens.push(token);
        }

        // for token in tokens.iter() {
        //     println!("({:2}, {:2})  Token <{:?}>    \tLexeme {:?}", token.line_no, token.column_no, token.token_type, token.lexeme);
        // }
            
        println!("Lexer Finished");
        inst.token_buffer = tokens;

        inst
    }

    pub fn into_token_stream(self) -> Vec<Token> {
        self.token_buffer
    }

    pub fn get_iter(&self) -> impl Iterator<Item = char> {
        let buf = BufReader::new(File::open(self.file_name.as_str()).expect("open file failed"));

        let line_iter = buf
            .lines()
            .map(|line| line.map(|st| st.chars().collect::<Vec<_>>()))
            .flat_map(|f| {
                let mut vec = f.unwrap();
                vec.push('\n');
                vec
            });

        // for ch in line_iter {
        //     print!("{:?}", ch);
        // }
        line_iter
    }

    pub fn get_tok(&mut self, iter: &mut Peekable<impl Iterator<Item = char>>) -> Token {
        // let mut iter = iter.peekable(); //self.get_iter().peekable(); might not even need peekable
        let mut last_char;
        if iter.peek().is_some() {
            last_char = iter.next().unwrap();
        } else {
            last_char = 0x0 as char;
        }
        self.column_no += 1;

        while last_char.is_whitespace() {
            if last_char == '\n' {
                self.line_no += 1;
                self.column_no = 1;
            }

            if iter.peek().is_some() {
                last_char = iter.next().unwrap();
            } else {
                last_char = 0x0 as char;
            }
            self.column_no += 1;
        }

        let mut id_str;

        if last_char.is_alphabetic() || last_char == '_' {
            id_str = String::from(last_char);
            self.column_no += 1;

            while iter.peek() == Some(&'_') || iter.peek().map_or(false, |ch| ch.is_alphanumeric())
            {
                last_char = iter.next().unwrap_or('\0');
                id_str.push(last_char);
                self.column_no += 1;
            }

            return match id_str.as_str() {
                "int" => self.return_token(id_str, TokenType::Integer), // int token,
                "float" => self.return_token(id_str, TokenType::Float),
                "bool" => self.return_token(id_str, TokenType::Bool),
                "void" => self.return_token(id_str, TokenType::Void),
                "extern" => self.return_token(id_str, TokenType::Extern),
                "if" => self.return_token(id_str, TokenType::IfStmt),
                "else" => self.return_token(id_str, TokenType::Else),
                "while" => self.return_token(id_str, TokenType::While),
                "return" => self.return_token(id_str, TokenType::Return),
                "true" => self.return_token(id_str, TokenType::BoolLit),
                "false" => self.return_token(id_str, TokenType::BoolLit),
                // lex other keywords
                _ => self.return_token(id_str, TokenType::Identity), // Identifier token
            };
        }

        let mut num_str: String;
        if last_char.is_ascii_digit() || last_char == '.' {
            num_str = String::from(last_char);
            
            if last_char == '.' {
                // float .[0-9]+
                
                while iter.peek().map_or(false, |ch| ch.is_ascii_digit()) {
                    last_char = iter.next().unwrap();
                    num_str.push(last_char);
                    self.column_no += 1;
                }
                // iter.next();
                // loop {
                //     num_str.push(last_char);
                //     last_char = iter.next().unwrap_or('\0');
                //     self.column_no += 1;
                //     if !last_char.is_ascii_digit() {
                //         break;
                //     }
                // }

                return self.return_token(num_str, TokenType::FloatLit);
            } else {
                while iter.peek().map_or(false, |ch| ch.is_ascii_digit()) {
                    last_char = iter.next().unwrap();
                    num_str.push(last_char);
                    self.column_no += 1;
                }

                if iter.peek() == Some(&'.') {
                    last_char = iter.next().unwrap_or('\0');
                    num_str.push(last_char);
                    // float [0-9]+.[0-9]+
                    while iter.peek().map_or(false, |ch| ch.is_ascii_digit()) {
                        last_char = iter.next().unwrap();
                        num_str.push(last_char);
                        self.column_no += 1;
                    }
                    // iter.next();
                    return self.return_token(num_str, TokenType::FloatLit);
                } else {
                    return self.return_token(num_str, TokenType::IntLit);
                }
            }
        }

        match last_char {
            '=' => {
                // lex eq
                let next_char = iter.peek();
                if next_char == Some(&'=') {
                    iter.next();
                    return self.return_token(String::from("=="), TokenType::Eq);
                } else {
                    return self.return_token(String::from("="), TokenType::Assign);
                }
            }
            '{' => return self.return_token(String::from("{"), TokenType::LBrace),
            '}' => return self.return_token(String::from("}"), TokenType::RBrace),
            '(' => return self.return_token(String::from("("), TokenType::LParen),
            ')' => return self.return_token(String::from(")"), TokenType::RParen),
            ';' => return self.return_token(String::from(";"), TokenType::Scolon),
            ',' => return self.return_token(String::from(","), TokenType::Comma),

            '+' => return self.return_token(String::from(last_char), TokenType::Plus),
            '-' => return self.return_token(String::from(last_char), TokenType::Minus),
            '*' => return self.return_token(String::from(last_char), TokenType::Mult),
            '%' => return self.return_token(String::from(last_char), TokenType::Mod),

            '&' => {
                let next_char = iter.peek();
                if next_char == Some(&'&') {
                    iter.next();
                    return self.return_token(String::from("&&"), TokenType::LAnd);
                } else {
                    return self.return_token(String::from("&"), TokenType::INVALID);
                }
            }
            '|' => {
                let next_char = iter.peek();
                if next_char == Some(&'|') {
                    iter.next();
                    return self.return_token(String::from("||"), TokenType::LOr);
                } else {
                    return self.return_token(String::from("|"), TokenType::INVALID);
                }
            }
            '!' => {
                let next_char = iter.peek();
                if next_char == Some(&'=') {
                    iter.next();
                    return self.return_token(String::from("!="), TokenType::Ne);
                } else {
                    return self.return_token(String::from("!"), TokenType::Not);
                }
            }
            '<' => {
                let next_char = iter.peek();
                if next_char == Some(&'=') {
                    iter.next();
                    return self.return_token(String::from("<="), TokenType::Le);
                } else {
                    return self.return_token(String::from("<"), TokenType::Lt);
                }
            }
            '>' => {
                let next_char = iter.peek();
                if next_char == Some(&'=') {
                    iter.next();
                    return self.return_token(String::from(">="), TokenType::Ge);
                } else {
                    return self.return_token(String::from(">"), TokenType::Gt);
                }
            }
            '/' => {
                let next_char = iter.peek();
                if next_char == Some(&'/') {
                    // comment
                    loop {
                        last_char = iter.next().unwrap_or('\0');
                        if last_char == '\0' || last_char == '\n' {
                            break self.get_tok(iter);
                        }
                    }
                } else {
                    return self.return_token(String::from("/"), TokenType::Div);
                }
            }
            // lex other control chars
            '\0' => return self.return_token(String::from(""), TokenType::EOF), // match EOF
            _ => return self.return_token(String::from(last_char), TokenType::INVALID), // match other bad chars
        }
    }

    fn return_token(&self, lexeme: String, token_type: TokenType) -> Token {
        Token {
            line_no: self.line_no,
            column_no: self.column_no - lexeme.len() - 1,
            token_type,
            lexeme,
        }
    }
}
