// src/lexer.rs

/*
 * 🔍 Brewco Lexical Analyzer ☕
 * 
 * @author: "Khushi Motwani" 💖
 * @khushi_scanner_magic: "Where text becomes meaningful tokens!" ✨
 * @tokenization_love: "Every character matters in our coffee brew!" ☕
 * 
 * This lexer transforms raw Brewco source code into beautiful tokens!
 * From coffee beans to brewing instructions - every symbol has meaning.
 * 
 * Crafted with precision, powered by coffee love! ☕💖
 */

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Beans,
    Bean,           // class
    Brew,           // function
    Blend,          // extends
    Taste,          // if
    Otherwise,      // else
    Steep,          // while
    Pour,           // for
    Roast,          // switch
    Serve,          // return
    Break,
    Continue,
    This,
    Super,
    Private,        // for private fields/methods
    Public,         // for public fields/methods
    CoffeeRecipe,   // interface
    New,            // new keyword for instantiation
    BrewTime,       // sleep/delay
    Foreach,        // foreach loop
    TasteCarefully, // try
    IfSpilled,      // catch
    RefillWith,     // for array element assignment
    Grind,          // import/load module
    In,             // in (for foreach loops)

    // Themed Operators
    Add,            // add (arithmetic +)
    Sip,            // sip (arithmetic -)
    BrewOp,         // brewop (arithmetic *)
    PourOp,         // pourop (arithmetic /)
    Grounds,        // grounds (arithmetic %)
    SameBlend,      // same_blend (==)
    DifferentBlend, // different_blend (!=)
    LessCaffeine,   // less_caffeine (<)
    MoreCaffeine,   // more_caffeine (>)
    NotStronger,    // not_stronger (<=)
    NotWeaker,      // not_weaker (>=)
    With,           // with (&&)
    Or,             // or (||)
    NoFoam,         // no_foam (!)
    BlendWith,      // blend_with (&)
    TopWith,        // top_with (|)
    Spice,          // spice (^)
    Invert,         // invert (~)
    DoubleShot,     // double_shot (<<)
    HalfCaf,        // half_caf (>>)
    PourIn,         // pour_in (=)
    ServeBack,      // serve_back (return)

    // Data types
    Cup,            // array
    CoffeeChain,    // linked list
    CoffeeMenu,     // hash map
    
    // Literals
    Identifier(String),
    Number(f64),
    String(String),
    
    // Operators
    Equals,         // =
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Colon,          // :
    Semicolon,      // ;
    Arrow,          // ->
    Greater,        // >
    Less,           // <
    GreaterEqual,   // >=
    LessEqual,      // <=
    Equal,          // ==
    NotEqual,       // !=
    And,            // &&
    OrSym,          // ||
    Not,            // !
    BitAnd,         // &
    BitOr,          // |
    BitXor,         // ^
    BitNot,         // ~
    Shl,            // <<
    Shr,            // >>
    
    // Delimiters
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    LBracket,       // [
    RBracket,       // ]
    Comma,          // ,
    Dot,            // .
    Newline,        // \n
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        // Skip lines that start with optional whitespace and then 🎀
        if c == '\u{1F380}' || c == ' ' || c == '\t' {
            // Clone iterator to check ahead
            let mut clone = chars.clone();
            // Skip whitespace
            while let Some(&wc) = clone.peek() {
                if wc == ' ' || wc == '\t' { clone.next(); } else { break; }
            }
            if let Some(&wc) = clone.peek() {
                if wc == '\u{1F380}' {
                    // Advance the main iterator to the comment start
                    while let Some(&wc) = chars.peek() {
                        if wc == '\u{1F380}' { chars.next(); break; }
                        if wc == '\n' { break; }
                        chars.next();
                    }
                    // Skip until newline
                    while let Some(&ch) = chars.peek() {
                        if ch == '\n' { break; }
                        chars.next();
                    }
                    continue;
                }
            }
        }
        match c {
            ' ' | '\t' | '\r' => { chars.next(); }
            '\n' => { chars.next(); tokens.push(Token::Newline); }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::SameBlend);
                } else {
                    tokens.push(Token::PourIn);
                }
            }
            '!' => {
                chars.next();
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::DifferentBlend);
                } else {
                    tokens.push(Token::NoFoam);
                }
            }
            '>' => {
                chars.next();
                if let Some(&'>') = chars.peek() {
                    chars.next();
                    tokens.push(Token::HalfCaf);
                } else if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::NotWeaker);
                } else {
                    tokens.push(Token::MoreCaffeine);
                }
            }
            '<' => {
                chars.next();
                if let Some(&'<') = chars.peek() {
                    chars.next();
                    tokens.push(Token::DoubleShot);
                } else if let Some(&'=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::NotStronger);
                } else {
                    tokens.push(Token::LessCaffeine);
                }
            }
            '+' => { tokens.push(Token::Add); chars.next(); }
            '-' => { 
                chars.next();
                if let Some(&'>') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Sip);
                }
            }
            '*' => { tokens.push(Token::BrewOp); chars.next(); }
            '/' => {
                chars.next();
                if let Some(&'/') = chars.peek() {
                    // It's a comment, consume until newline
                    while let Some(&ch) = chars.peek() {
                        if ch == '\n' { break; }
                        chars.next();
                    }
                } else {
                    tokens.push(Token::PourOp);
                }
            }
            '%' => { tokens.push(Token::Grounds); chars.next(); }
            ':' => { tokens.push(Token::Colon); chars.next(); }
            ';' => { tokens.push(Token::Semicolon); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '{' => { tokens.push(Token::LBrace); chars.next(); }
            '}' => { tokens.push(Token::RBrace); chars.next(); }
            '[' => { tokens.push(Token::LBracket); chars.next(); }
            ']' => { tokens.push(Token::RBracket); chars.next(); }
            ',' => { tokens.push(Token::Comma); chars.next(); }
            '.' => { tokens.push(Token::Dot); chars.next(); }
            '&' => {
                chars.next();
                if let Some(&'&') = chars.peek() {
                    chars.next();
                    tokens.push(Token::With);
                } else {
                    tokens.push(Token::BlendWith);
                }
            }
            '|' => {
                chars.next();
                if let Some(&'|') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Or);
                } else {
                    tokens.push(Token::TopWith);
                }
            }
            '^' => { tokens.push(Token::Spice); chars.next(); }
            '~' => { tokens.push(Token::Invert); chars.next(); }
            '"' => {
                chars.next();
                let mut s = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '"' { break; }
                    s.push(ch); chars.next();
                }
                chars.next();
                tokens.push(Token::String(s));
            }
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) || ch == '.' {
                        num_str.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num_str.parse().unwrap()));
            }
            _ if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(match ident.as_str() {
                    "beans" => Token::Beans,
                    "bean" => Token::Bean,
                    "brew" => Token::Brew,
                    "blend" => Token::Blend,
                    "taste" => Token::Taste,
                    "otherwise" => Token::Otherwise,
                    "steep" => Token::Steep,
                    "pour" => Token::Pour,
                    "roast" => Token::Roast,
                    "serve" => Token::Serve,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    "this" => Token::This,
                    "super" => Token::Super,
                    "private" => Token::Private,
                    "public" => Token::Public,
                    "recipe" => Token::CoffeeRecipe,
                    "new" => Token::New,
                    "brew_time" => Token::BrewTime,
                    "foreach" => Token::Foreach,
                    "taste_carefully" => Token::TasteCarefully,
                    "if_spilled" => Token::IfSpilled,
                    "refill_with" => Token::RefillWith,
                    "grind" => Token::Grind,
                    "in" => Token::In,
                    "add" => Token::Add,
                    "sip" => Token::Sip,
                    "brew_op" => Token::BrewOp,
                    "pour_op" => Token::PourOp,
                    "grounds" => Token::Grounds,
                    "same_blend" => Token::SameBlend,
                    "different_blend" => Token::DifferentBlend,
                    "less_caffeine" => Token::LessCaffeine,
                    "more_caffeine" => Token::MoreCaffeine,
                    "not_stronger" => Token::NotStronger,
                    "not_weaker" => Token::NotWeaker,
                    "with" => Token::With,
                    "or" => Token::Or,
                    "no_foam" => Token::NoFoam,
                    "blend_with" => Token::BlendWith,
                    "top_with" => Token::TopWith,
                    "spice" => Token::Spice,
                    "invert" => Token::Invert,
                    "double_shot" => Token::DoubleShot,
                    "half_caf" => Token::HalfCaf,
                    "pour_in" => Token::PourIn,
                    "serve_back" => Token::ServeBack,
                    "true" => Token::Identifier("true".to_string()),
                    "false" => Token::Identifier("false".to_string()),
                    _ => Token::Identifier(ident),
                });
            }
            _ => { chars.next(); }
        }
    }
    tokens
}

/*
 * Coffee-Themed Token System 
 * @designer: Khushi Motwani
 * @mood: Absolutely delighted ☕✨
 * 
 * Each token name was chosen with so much love!
 * "beans" for variables? Genius! (if I say so myself 😅)
 * "pourout" for print? Adorable! 
 * "brew" for functions? Perfect! ☕
 * 
 * @khushi_confession: I spent way too much time naming these 
 * but regret NOTHING! 💖
 */

/*
 * Emoji Comment Support (🎀)
 * @innovation: Khushi Motwani's special touch
 * @why: "Because why not make comments prettier?" ✨
 * 
 * Yes, I made emoji comments a thing in my language!
 * 🎀 looks so cute and makes code documentation beautiful
 * 
 * This was probably my favorite feature to implement! ☕💖
 * - Khushi
 */

/*
 * @khushi_final_thoughts:
 * The lexer is like the first impression of a language
 * I wanted Brewco's first impression to be warm, 
 * welcoming, and full of coffee love! ☕
 * 
 * Hope every developer who uses this feels the joy 
 * I felt creating it! ✨
 * 
 * Keep tokenizing with love! 💖
 * - Khushi Motwani 
 */
