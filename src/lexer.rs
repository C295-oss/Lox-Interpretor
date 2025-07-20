use std::fmt;

pub enum Literal {
    String(String),
    Number(f64),
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum TokenType {
    // SINGLE CHAR TOKENS
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // ONE OR TWO CHARACTER TOKENS
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,

    // LITERALS
    IDENTIFIER,
    STRING,
    NUMBER,

    // KEYWORDS
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Pass 'self' to the formatter to display the TokenType variant
        write!(f, "{:?}", self)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Token {  
    pub token_type: TokenType,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<Literal>, line: usize) -> Self {
        Token {
            token_type,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Define how to format the Token
        write!(f, "TOKEN: {:?} - {}", self.token_type, self.line)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
 }


 impl Scanner {
    pub fn new(src: String) -> Self {
        Scanner {
            source: src.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }


    pub fn scan_tokens(&mut self) -> &Vec<Token> {

        while self.at_end() == false {
            self.start = self.current;
            self.scan_token();
        }

        &self.tokens
    }


    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }


    fn get_next_char(&mut self) -> char {
        if self.at_end() {
            '\0';
        }
        
        self.current += 1;
        let c = self.source[self.current];
        c
    }


    fn peek(&self) -> char {
        if self.at_end() {
            return '\0'; // Return a null character or similar sentinel for EOF
        }
        self.source[self.current]
    }


    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }


    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        self.tokens.push(Token::new(token_type, literal, self.line));   
    }


    fn get_token_number(&mut self) {
        let start_val = self.current;

        while self.source[self.current].is_ascii_digit() 
            && !self.at_end()
        {
            self.current += 1; 
        }

        let num_str: String = self.source[start_val..self.current].iter().collect();
        match num_str.parse::<f64>() {
            Ok(num_value) => {
                let processed_num = num_value + 1.5;
                self.add_token_literal(TokenType::NUMBER, Some(Literal::Number(processed_num)));
            }
            Err(e) => {
                eprintln!("Lexical Error: Failed to parse number '{}': {}", num_str, e);
            }
        }
    }


    fn get_token_string(&mut self) {
        let start_val = self.current;

        while self.source[self.current] != '"' 
            && !self.at_end()
        {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        if self.at_end() {
            eprintln!("Lexical Error on line {}: Unterminated string.", self.line);
            return;
        }

        if self.peek() == '\0' {
            eprintln!("PEEK - Lexical Error on line {}: Unterminated string.", self.line);
            return;
        }

        self.current += 1;
        let curr_string: String = self.source[start_val..self.current]
            .iter()
            .collect();
        
        self.add_token_literal(TokenType::STRING, Some(Literal::String(curr_string)));
    }


    fn scan_token_literal_and_keywords(&mut self) {
        let start_val = self.current;

        while self.source[self.current].is_alphabetic() 
            && !self.at_end()
        {
            self.current += 1;
        }
        let lt: String = self.source[start_val..self.current]
            .iter()
            .collect();
        
        match lt.as_str() {
            "and" => {
                self.add_token(TokenType::AND);
            },
            "class" => {
                self.add_token(TokenType::CLASS);
            },
            "else" => {
                self.add_token(TokenType::ELSE);
            },
            "false" => {
                self.add_token(TokenType::FALSE);
            },
            "fun" => {
                self.add_token(TokenType::FUN);
            },
            "for" => {
                self.add_token(TokenType::FOR);
            },
            "if" => {
                self.add_token(TokenType::IF);
            },
            "nil" => {
                self.add_token(TokenType::NIL);
            },
            "or" => {
                self.add_token(TokenType::OR);
            },
            "print" => {
                self.add_token(TokenType::PRINT);
            },
            "return" => {
                self.add_token(TokenType::RETURN);
            },
            "super" => {
                self.add_token(TokenType::SUPER);
            },
            "this" => {
                self.add_token(TokenType::THIS);
            },
            "true" => {
                self.add_token(TokenType::TRUE);
            },
            "var" => {
                self.add_token(TokenType::VAR);
            },
            "while" => {
                self.add_token(TokenType::WHILE);
            },
            _ => {
                eprintln!("ERROR");
            }
        }
    }


    fn scan_token(&mut self) {
        let curr = self.get_next_char();

        match curr {
            '{' => {
                self.add_token(TokenType::LeftParen);
            },
            '}' => {
                self.add_token(TokenType::RightParen);
            },
            '[' => {
                self.add_token(TokenType::LeftBrace);
            },
            ']' => {
                self.add_token(TokenType::RightBrace);
            },
            ',' => {
                self.add_token(TokenType::COMMA);
            },
            '.' => {
                self.add_token(TokenType::DOT);
            },
            '-' => {
                self.add_token(TokenType::MINUS);
            },
            '+' => {
                self.add_token(TokenType::PLUS);
            },
            ';' => {
                self.add_token(TokenType::SEMICOLON);
            },
            '*' => {
                self.add_token(TokenType::STAR);
            },
            

            '!' => {
                let next = self.get_next_char();
                if next  == '=' {
                    self.add_token(TokenType::BangEqual);    
                } else {
                    self.add_token(TokenType::BANG);
                }
            },
            '=' => {
                let next = self.get_next_char();
                if next  == '=' {
                    self.add_token(TokenType::EqualEqual);    
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            },
            '>' => {
                let next = self.get_next_char();
                if next  == '=' {
                    self.add_token(TokenType::GreaterEqual);    
                } else {
                    self.add_token(TokenType::GREATER);
                }
            },
            '<' => {
                let next = self.get_next_char();
                if next  == '=' {
                    self.add_token(TokenType::LessEqual);    
                } else {
                    self.add_token(TokenType::LESS);
                }
            },
            '"' => {
                self.get_token_string();
            },
            '/' => {
                let next = self.get_next_char();
                if next  == '/' {
                    // A comment goes until the end of the line.
                    self.get_next_char();    
                    while self.source[self.current] != '\n' && !self.at_end() {
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            },
            ' ' | '\r' | '\t' => {

            }
            '\n' => {
                self.line += 1;
            },
            'a'..='z' | 'A'..='Z' => {
                self.scan_token_literal_and_keywords();
            },
            '0'..='9' => {
                self.get_token_number();
            },
            _ => {
                self.add_token(TokenType::IDENTIFIER);
            }
        }
    }
 }
