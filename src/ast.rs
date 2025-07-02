/*
 * 🌳 Brewco Abstract Syntax Tree 🌳
 * 
 * @author: "Khushi Motwani" 💖
 * @khushi_creation_note: "The structure that holds our coffee dreams together!"
 * @coffee_level: "Perfectly balanced, as all ASTs should be" ☕
 * 
 * This AST is the backbone of Brewco - where syntax becomes structure!
 * Every node represents a piece of our beautiful coffee-themed language.
 * 
 * Built with love, powered by coffee! ☕✨
 */

// src/ast.rs
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    ArrayLiteral(Vec<Expr>),
    ObjectLiteral(Vec<(String, Expr)>),
    BinaryOp { 
        left: Box<Expr>, 
        op: BinaryOperator, 
        right: Box<Expr> 
    },
    Assignment {
        target: Box<Expr>,
        value: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expr>
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>
    },
    MemberAccess { 
        object: Box<Expr>, 
        member: String 
    },
    ArrayAccess { 
        array: Box<Expr>, 
        index: Box<Expr> 
    },
    NewBean { 
        name: String, 
        args: Vec<Expr> 
    },
    Grind(String),
    This,
    Super,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,            // + or add
    Subtract,       // - or sip
    Multiply,       // * or brewop
    Divide,         // / or pourop
    Modulo,         // % or grounds
    Equal,          // == or same_blend
    NotEqual,       // != or different_blend
    Greater,        // > or more_caffeine
    Less,           // < or less_caffeine
    GreaterEqual,   // >= or not_weaker
    LessEqual,      // <= or not_stronger
    And,            // && or with
    Or,             // || or or
    BitAnd,         // & or blend_with
    BitOr,          // | or top_with
    BitXor,         // ^ or spice
    Shl,            // << or double_shot
    Shr,            // >> or half_caf
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,         // -
    Not,            // ! or no_foam
    BitNot,         // ~ or invert
}

#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl { 
        name: String, 
        type_ann: Option<String>,
        value: Expr 
    },
    ArrayDecl { 
        name: String, 
        elements: Vec<Expr> 
    },
    ObjectDecl {
        name: String,
        fields: Vec<(String, Expr)>
    },
    Print(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        else_branch: Vec<Statement>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Expr,
        increment: Option<Expr>,
        body: Vec<Statement>,
    },
    RoastDecl { 
        name: String, 
        body: Vec<Statement> 
    },
    BeanDecl {
        name: String,
        parent: Option<String>,
        fields: Vec<FieldDecl>,
        methods: Vec<Statement>,
    },
    CoffeeRecipeDecl {
        name: String,
        methods: Vec<MethodSignature>,
    },
    BrewDecl {
        name: String,
        params: Vec<ParamDecl>,
        body: Vec<Statement>,
        return_type: Option<String>,
    },
    BrewTime(Expr),
    Return(Option<Expr>),
    Break,
    Continue,
    ExprStmt(Expr),
    Foreach {
        var: String,
        iterable: Expr,
        body: Vec<Statement>,
    }, // foreach loop
    ConstructorDecl {
        params: Vec<ParamDecl>,
        body: Vec<Statement>,
    },
    RoastSwitch {
        value: Expr,
        arms: Vec<(Expr, Vec<Statement>)>,
        default: Vec<Statement>,
    },
    TryCatch {
        try_branch: Vec<Statement>,
        error_variable: Option<String>,
        catch_branch: Vec<Statement>,
    },
}

#[derive(Debug, Clone)]
pub struct FieldDecl {
    pub name: String,
    pub value: Expr,
}

#[derive(Debug, Clone)]
pub struct MethodSignature {
    pub name: String,
    pub params: Vec<ParamDecl>,
    pub return_type: String,
}

#[derive(Debug, Clone)]
pub struct ParamDecl {
    pub name: String,
    pub type_name: String,
}
