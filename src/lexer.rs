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
    LeftSqrBrace,
    RightSqrBrace,
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
        match self {
            Self::LeftParen => write!(f, "LeftParen"),
            Self::RightParen => write!(f, "RightParen"),
            Self::LeftSqrBrace => write!(f, "LeftSqrBrace"),
            Self::RightSqrBrace => write!(f, "RightSqrBrace"),
            Self::LeftBrace => write!(f, "LeftBrace"),
            Self::RightBrace => write!(f, "RightBrace"),
            Self::COMMA => write!(f, "COMMA"),
            Self::DOT => write!(f, "DOT"),
            Self::MINUS => write!(f, "MINUS"),
            Self::PLUS => write!(f, "PLUS"),
            Self::SEMICOLON => write!(f, "SEMICOLON"),
            Self::SLASH => write!(f, "SLASH"),
            Self::STAR => write!(f, "STAR"),

            Self::BANG => write!(f, "BANG"),
            Self::BangEqual => write!(f, "BangEqual"),
            Self::EQUAL => write!(f, "EQUAL"),
            Self::EqualEqual => write!(f, "EqualEqual"),
            Self::GREATER => write!(f, "GREATER"),
            Self::GreaterEqual => write!(f, "GreaterEqual"),
            Self::LESS => write!(f, "LESS"),
            Self::LessEqual => write!(f, "LessEqual"),

            Self::IDENTIFIER => write!(f, "IDENTIFIER"),
            Self::STRING => write!(f, "STRING"),
            Self::NUMBER => write!(f, "NUMBER"),

            Self::AND => write!(f, "AND"),
            Self::CLASS => write!(f, "CLASS"),
            Self::ELSE => write!(f, "ELSE"),
            Self::FALSE => write!(f, "FALSE"),
            Self::FUN => write!(f, "FUN"),
            Self::FOR => write!(f, "FOR"),
            Self::IF => write!(f, "IF"),
            Self::NIL => write!(f, "NIL"),
            Self::OR => write!(f, "OR"),
            Self::PRINT => write!(f, "PRINT"),
            Self::RETURN => write!(f, "RETURN"),
            Self::SUPER => write!(f, "SUPER"),
            Self::THIS => write!(f, "THIS"),
            Self::TRUE => write!(f, "TRUE"),
            Self::VAR => write!(f, "VAR"),
            Self::WHILE => write!(f, "WHILE"),

            Self::EOF => write!(f, "EOF"),
        }
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
        write!(f, "TOKEN: {:?} - Line {}", self.token_type, self.line)?; // Added Line for clarity and ? for error handling

        // You might want to also include the literal if it exists
        if let Some(ref lit) = self.literal {
            match lit {
                Literal::String(s) => write!(f, " - Literal: \"{}\"", s)?,
                Literal::Number(i) => write!(f, " - Literal: {}", i)?,
            }
        }
        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
 }


 impl Scanner {
    pub fn new(src: String) -> Self {
        Scanner {
            source: src.chars().collect(),
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
        }
    }


    pub fn scan_tokens(&mut self) -> &Vec<Token> {

        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::EOF);
        &self.tokens
    }


    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }


    fn get_next_char(&mut self) -> char {
        
        if self.at_end() {
            return '\0';
        }
        
        self.current += 1;
        let c = self.peek();
        c
    }


    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }


    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }
        self.source[self.current]
    }


    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }


    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }


    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        println!("Token Added: {}", token_type); 
        self.tokens.push(Token::new(token_type, literal, self.line));  
    }


    fn get_token_number(&mut self) {
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let num_str: String = self.source[self.start..self.current].iter().collect();
        // println!("\tget_token_number: {}", num_str);
        
        match num_str.parse::<f64>() {
            Ok(num_value) => {
                let processed_num = num_value;
                self.add_token_literal(TokenType::NUMBER, Some(Literal::Number(processed_num)));
            }
            Err(e) => {
                eprintln!("Lexical Error: Failed to parse number '{}': {}", num_str, e);
            }
        }
    }


    fn get_token_string(&mut self) {
        while self.peek() != '"' && !self.at_end()
        {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }


        if self.at_end() {
            eprintln!("Lexical Error: Unterminated string on line {}.", self.line);
            return;
        }
        self.advance(); // Consume closing '"'

        let curr_string: String = self.source[self.start..self.current]
            .iter()
            .collect();
        
        self.add_token_literal(TokenType::STRING, Some(Literal::String(curr_string)));
    }


    fn scan_token_literal_and_keywords(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }


        let lt: String = self.source[self.start..self.current]
            .iter()
            .collect();

        println!("scan_token_literal_and_keywords - {}", lt);

        
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
                self.add_token_literal(TokenType::IDENTIFIER, Some(Literal::String(lt)));
            }
        }
    }


    fn scan_token(&mut self) {
        let curr = self.advance();

        match curr {
            '{' => {
                self.add_token(TokenType::LeftBrace);
            },
            '}' => {
                self.add_token(TokenType::RightBrace);
            },
            '(' => {
                self.add_token(TokenType::LeftParen);
            },
            ')' => {
                self.add_token(TokenType::RightParen);
            },
            '[' => {
                self.add_token(TokenType::LeftSqrBrace);
            },
            ']' => {
                self.add_token(TokenType::RightSqrBrace);
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
                let next = self.peek_next(); 
                
                if next  == '=' {
                    self.add_token(TokenType::BangEqual);    
                } else {
                    self.add_token(TokenType::BANG);
                }
            },
            '=' => {
                let next = self.peek_next();
                
                if next  == '=' {
                    self.add_token(TokenType::EqualEqual);    
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            },
            '>' => {
                let next = self.peek_next();
                
                if next  == '=' {
                    self.add_token(TokenType::GreaterEqual);    
                } else {
                    self.add_token(TokenType::GREATER);
                }
            },
            '<' => {
                let next = self.peek_next();
                
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
                let next = self.peek_next();
                
                if next  == '/' {
                    self.get_next_char();    
                    while self.peek() != '\n' && !self.at_end() {
                        let _ = self.get_next_char();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            },
            ' ' | '\r' | '\t' => {

            }
            '\\' => {
                let next = self.peek_next();
                
                if next  == 'n' {
                    self.line += 1;
                }
            },
            'a'..='z' | 'A'..='Z' => {
                self.scan_token_literal_and_keywords();
            },
            '0'..='9' => {
                self.get_token_number();
            },
            '\0' => {
                self.add_token(TokenType::EOF);
            },
            _ => {
                eprintln!("scan_token : ERROR");
            }
        }
    }
 }
