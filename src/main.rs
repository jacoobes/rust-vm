use core::panic;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
#[logos(subpattern decimal = r"[0-9][_0-9]*")]
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("var")]
    Var,
    #[token(".")]
    Period,
    #[token(":")]
    Semi,
    #[token("=")]
    Equal,
    #[token("(")]
    ParenL,
    #[token(")")]
    ParenR,
    #[token("{")]
    BrackL,
    #[token("}")]
    BrackR,
    #[token("*")]
    Star,
    #[token("start")]
    Start,

    #[token("fn")]
    Function,

    #[regex(r"[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Double(f64),

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Symbol,
}

#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    Ident(String),
    Call(String, Vec<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}
 
#[derive(Debug, Clone)]
enum Stmt {
    VarDecl { name: String, value: Expr },
    Expr(Expr),
}
 
#[derive(Debug, Clone)]
enum Item {
    VarDecl { name: String, value: Expr },
    Start(Vec<Stmt>),
}
 
#[derive(Debug, Clone)]
struct Program {
    items: Vec<Item>,
}




fn main() {
    let program = "
        var song = violin2 * sin(0.5)

        start {
            song()    
        }

    ";
    for result in Token::lexer(program) {
        match result {
            Ok(token) => println!("{:#?}", token),
            Err(e) => panic!("some error occurred: {:?}", e),
        }
    }
    
}
