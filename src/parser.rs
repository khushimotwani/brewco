/*
 * ☕ Brewco Recursive Descent Parser ☕
 * 
 * @author: "Khushi Motwani" 💖
 * @khushi_parser_magic: "Turning coffee tokens into beautiful syntax trees!" ✨
 * @complexity_level: "Advanced coffee brewing techniques" ☕
 * 
 * Welcome to the most complex part of Brewco!
 * This parser transforms our coffee-themed tokens into executable AST nodes.
 * 
 * Built with precision, powered by endless coffee! ☕💖
 */

// src/parser.rs

use crate::ast::*;
use crate::lexer::Token;

pub struct ParseResult {
    pub statements: Vec<Statement>,
    pub errors: Vec<String>,
}

pub fn parse(tokens: &[Token]) -> ParseResult {
    let mut stmts = Vec::new();
    let mut errors = Vec::new();
    let mut i = 0;
    // Skip leading newlines
    while i < tokens.len() && tokens.get(i) == Some(&Token::Newline) {
        i += 1;
    }
    while i < tokens.len() {
        match parse_statement(tokens, i) {
            Some((st, ni)) => {
                stmts.push(st);
                i = ni;
                // Skip newlines after each statement
                while i < tokens.len() && tokens.get(i) == Some(&Token::Newline) {
                    i += 1;
                }
            },
            None => {
                let err_line = format!(
                    "This syntax is never ever getting back together with the parser at token {}. You need to calm down, but this line is causing a stir!",
                    i
                );
                errors.push(err_line);
                // Skip to next newline or end
                while i < tokens.len() && tokens.get(i) != Some(&Token::Newline) {
                    i += 1;
                }
                while i < tokens.len() && tokens.get(i) == Some(&Token::Newline) {
                    i += 1;
                }
            }
        }
    }
    ParseResult { statements: stmts, errors }
}

fn parse_statement(t: &[Token], mut i: usize) -> Option<(Statement, usize)> {
    use Token::*;
    // Skip newlines or semicolons
    while i < t.len() && (t.get(i) == Some(&Newline) || t.get(i) == Some(&Semicolon)) {
        i += 1;
    }

    if i >= t.len() { return None; }

    // Break / Continue
    if t.get(i) == Some(&Break) {
        return Some((Statement::Break, i + 1));
    }
    if t.get(i) == Some(&Continue) {
        return Some((Statement::Continue, i + 1));
    }

    // Return statement: serve [expr]
    if t.get(i) == Some(&Serve) {
        let mut j = i + 1;
        // Check if there's a return value
        if j < t.len() && t.get(j) != Some(&Newline) && t.get(j) != Some(&Semicolon) {
            let (expr, nj) = parse_expr(t, j)?;
            return Some((Statement::Return(Some(expr)), nj));
        } else {
            return Some((Statement::Return(None), j));
        }
    }

    // While loop: steep <cond> { body }
    if t.get(i) == Some(&Steep) {
        return parse_while(t, i);
    }

    // If statement: taste <cond> { then } [otherwise { else }]
    if t.get(i) == Some(&Taste) {
        return parse_if(t, i);
    }

    // For loop: pour <init>; <cond>; <incr> { body }
    if t.get(i) == Some(&Pour) {
        return parse_for(t, i);
    }

    // Switch / roast
    if t.get(i) == Some(&Roast) {
        return parse_roast(t, i);
    }

    // Try / catch
    if t.get(i) == Some(&TasteCarefully) {
        return parse_try_catch(t, i);
    }

    if t.get(i) == Some(&Bean) {
        return parse_bean_declaration(t, i);
    }

    // Coffee recipe (interface) declaration
    if t.get(i) == Some(&CoffeeRecipe) {
        return parse_coffee_recipe_declaration(t, i);
    }

    // Function declaration: brew <identifier>(<params>) { body }
    if t.get(i) == Some(&Brew) {
        return parse_brew_declaration(t, i);
    }

    // Variable declaration: beans <identifier> [: <type>] = <expr>
    if t.get(i) == Some(&Beans) {
        return parse_variable_declaration(t, i);
    }

    // Handle print statement: pourout <expr>[, <expr> ...]
    if let Some(Token::Identifier(id)) = t.get(i) {
        if id == "pourout" {
            let mut args = Vec::new();
            let mut j = i + 1;
            // Skip leading newlines
            while j < t.len() && t.get(j) == Some(&Newline) { j += 1; }
            // At least one argument expected
            let (first_arg, nj) = parse_expr(t, j)?;
            args.push(first_arg);
            j = nj;
            while j < t.len() && t.get(j) == Some(&Comma) {
                j += 1; // skip comma
                while j < t.len() && t.get(j) == Some(&Newline) { j += 1; }
                let (arg, nni) = parse_expr(t, j)?;
                args.push(arg);
                j = nni;
            }
            let expr = if args.len() == 1 {
                args.into_iter().next().unwrap()
            } else {
                Expr::ArrayLiteral(args)
            };
            return Some((Statement::Print(expr), j));
        }
    }

    let (expr, ni) = parse_expr(t, i)?;
    Some((Statement::ExprStmt(expr), ni))
}

fn parse_expr(t: &[Token], i: usize) -> Option<(Expr, usize)> {
    parse_assignment(t, i)
}

fn parse_assignment(t: &[Token], i: usize) -> Option<(Expr, usize)> {
    let (expr, ni) = parse_binary_op(t, i, 0)?;

    if ni < t.len() {
        // Handle various assignment operators
        match t.get(ni) {
            Some(Token::PourIn) | Some(Token::RefillWith) | Some(Token::Equals) => {
            let (value, nni) = parse_assignment(t, ni + 1)?;
            match expr {
                Expr::Identifier(_) | Expr::MemberAccess {..} | Expr::ArrayAccess {..} => {
                    return Some((Expr::Assignment {
                        target: Box::new(expr),
                        value: Box::new(value),
                    }, nni));
                }
                _ => return None, // Invalid assignment target
            }
            }
            _ => {}
        }
    }
    Some((expr, ni))
}

fn parse_binary_op(t: &[Token], mut i: usize, min_prec: u8) -> Option<(Expr, usize)> {
    let (mut lhs, ni) = parse_unary_op(t, i)?;
    i = ni;
    while i < t.len() {
        let op = match t.get(i) {
            Some(tok) => match op_prec(tok) {
                Some((prec, op)) if prec >= min_prec => op,
                _ => break,
            },
            None => break,
        };
        let prec = op_prec(t.get(i)?).unwrap().0;
        let next_min_prec = prec + 1;
        i += 1;
        let (rhs, nni) = parse_binary_op(t, i, next_min_prec)?;
        i = nni;
        lhs = Expr::BinaryOp {
            left: Box::new(lhs),
            op,
            right: Box::new(rhs),
        };
    }
    Some((lhs, i))
}

fn parse_unary_op(t: &[Token], i: usize) -> Option<(Expr, usize)> {
    let op = match t.get(i)? {
        Token::Sip => UnaryOperator::Negate,
        Token::NoFoam => UnaryOperator::Not,
        Token::Invert => UnaryOperator::BitNot,
        _ => return parse_call(t, i),
    };
    let (expr, ni) = parse_unary_op(t, i + 1)?;
    Some((Expr::UnaryOp { op, expr: Box::new(expr) }, ni))
}

fn parse_call(t: &[Token], i: usize) -> Option<(Expr, usize)> {
    let (mut expr, mut ni) = parse_primary(t, i)?;
    while ni < t.len() {
        if t.get(ni) == Some(&Token::LParen) {
            let (args, nni) = parse_args(t, ni + 1)?;
            expr = Expr::Call { callee: Box::new(expr), args };
            ni = nni;
        } else if t.get(ni) == Some(&Token::Dot) {
            if let Some(Token::Identifier(member)) = t.get(ni + 1) {
                expr = Expr::MemberAccess { object: Box::new(expr), member: member.clone() };
                ni += 2;
            } else {
                return None;
            }
        } else if t.get(ni) == Some(&Token::LBracket) {
            let (index, nni) = parse_expr(t, ni + 1)?;
            if t.get(nni) != Some(&Token::RBracket) { return None; }
            expr = Expr::ArrayAccess { array: Box::new(expr), index: Box::new(index) };
            ni = nni + 1;
        } else {
            break;
        }
    }
    Some((expr, ni))
}

fn parse_args(t: &[Token], mut i: usize) -> Option<(Vec<Expr>, usize)> {
    let mut args = Vec::new();
    if t.get(i) == Some(&Token::RParen) { return Some((args, i + 1)); }
    loop {
        let (arg, ni) = parse_expr(t, i)?;
        args.push(arg);
        i = ni;
        if t.get(i) == Some(&Token::Comma) {
            i += 1;
        } else if t.get(i) == Some(&Token::RParen) {
            return Some((args, i + 1));
        } else {
            return None;
        }
    }
}

fn parse_primary(t: &[Token], i: usize) -> Option<(Expr, usize)> {
    match t.get(i)? {
        Token::Number(n) => Some((Expr::Number(*n), i + 1)),
        Token::String(s) => Some((Expr::String(s.clone()), i + 1)),
        Token::Grind => {
            if let Some(Token::String(path)) = t.get(i + 1) {
                Some((Expr::Grind(path.clone()), i + 2))
            } else {
                None // Expected a string literal path after 'grind'
            }
        }
        Token::New => {
            // Parse 'new ClassName(args)'
            if let Some(Token::Identifier(class_name)) = t.get(i + 1) {
                let mut j = i + 2;
                if t.get(j) == Some(&Token::LParen) {
                    let (args, nj) = parse_args(t, j + 1)?;
                    Some((Expr::NewBean { name: class_name.clone(), args }, nj))
                } else {
                    // No parentheses, just 'new ClassName'
                    Some((Expr::NewBean { name: class_name.clone(), args: vec![] }, j))
                }
            } else {
                None
            }
        }
        Token::This => Some((Expr::This, i + 1)),
        Token::Super => Some((Expr::Super, i + 1)),
        Token::Identifier(id) => match id.as_str() {
            "true" => Some((Expr::Boolean(true), i + 1)),
            "false" => Some((Expr::Boolean(false), i + 1)),
            _ => Some((Expr::Identifier(id.clone()), i + 1)),
        },
        Token::LParen => {
            let (expr, ni) = parse_expr(t, i + 1)?;
            if t.get(ni) == Some(&Token::RParen) {
                Some((expr, ni + 1))
            } else {
                None
            }
        }
        Token::LBracket => parse_array_literal(t, i + 1),
        Token::LBrace => parse_object_literal(t, i + 1),
        _ => None,
    }
}

fn parse_array_literal(t: &[Token], mut i: usize) -> Option<(Expr, usize)> {
    use Token::*;
    let mut elements = Vec::new();
    // Skip leading newlines
    while i < t.len() && t.get(i) == Some(&Newline) { i += 1; }
    if t.get(i) == Some(&RBracket) {
        return Some((Expr::ArrayLiteral(elements), i + 1));
    }
    loop {
        while i < t.len() && t.get(i) == Some(&Newline) { i += 1; }
        let (elem, ni) = parse_expr(t, i)?;
        elements.push(elem);
        i = ni;
        while i < t.len() && t.get(i) == Some(&Newline) { i += 1; }
        if t.get(i) == Some(&Comma) {
            i += 1;
            continue;
        } else if t.get(i) == Some(&RBracket) {
            return Some((Expr::ArrayLiteral(elements), i + 1));
        } else {
            return None;
        }
    }
}

fn parse_object_literal(t: &[Token], mut i: usize) -> Option<(Expr, usize)> {
    use Token::*;
    let mut fields = Vec::new();
    // Skip leading newlines
    while i < t.len() && t.get(i) == Some(&Newline) { i += 1; }
    if t.get(i) == Some(&RBrace) {
        return Some((Expr::ObjectLiteral(fields), i + 1));
    }
    loop {
        while i < t.len() && t.get(i) == Some(&Newline) { i += 1; }
        
        // Key can be either a string literal or an identifier
        let key = match t.get(i) {
            Some(String(key)) => {
                i += 1;
                key.clone()
            }
            Some(Identifier(key)) => {
                i += 1;
            key.clone()
            }
            _ => return None,
        };
        
        while i < t.len() && t.get(i) == Some(&Newline) { i += 1; }
        if t.get(i) != Some(&Colon) { return None; }
        i += 1;
        while i < t.len() && t.get(i) == Some(&Newline) { i += 1; }
        let (value, ni) = parse_expr(t, i)?;
        i = ni;
        fields.push((key, value));
        while i < t.len() && t.get(i) == Some(&Newline) { i += 1; }
        if t.get(i) == Some(&Comma) {
            i += 1;
            continue;
        } else if t.get(i) == Some(&RBrace) {
            return Some((Expr::ObjectLiteral(fields), i + 1));
        } else {
            return None;
        }
    }
}

fn op_prec(tok: &Token) -> Option<(u8, BinaryOperator)> {
    use BinaryOperator::*;
    let (prec, op) = match tok {
        // Logical operators (lowest precedence)
        Token::Or => (1, Or),
        Token::With => (2, And),
        
        // Comparison operators
        Token::SameBlend => (3, Equal),
        Token::DifferentBlend => (3, NotEqual),
        Token::LessCaffeine => (4, Less),
        Token::MoreCaffeine => (4, Greater),
        Token::NotStronger => (4, LessEqual),
        Token::NotWeaker => (4, GreaterEqual),
        
        // Standard symbol operators (for compatibility)
        Token::Equal => (3, Equal),
        Token::NotEqual => (3, NotEqual),
        Token::Less => (4, Less),
        Token::Greater => (4, Greater),
        Token::LessEqual => (4, LessEqual),
        Token::GreaterEqual => (4, GreaterEqual),
        Token::And => (2, And),
        Token::OrSym => (1, Or),
        
        // Arithmetic operators
        Token::Add => (5, Add),
        Token::Sip => (5, Subtract),
        Token::Plus => (5, Add),
        Token::Minus => (5, Subtract),
        
        // Higher precedence arithmetic
        Token::BrewOp => (6, Multiply),
        Token::PourOp => (6, Divide),
        Token::Grounds => (6, Modulo),
        Token::Star => (6, Multiply),
        Token::Slash => (6, Divide),
        Token::Percent => (6, Modulo),
        
        // Bitwise operators
        Token::BlendWith => (7, BitAnd),
        Token::TopWith => (7, BitOr),
        Token::Spice => (7, BitXor),
        Token::DoubleShot => (8, Shl),
        Token::HalfCaf => (8, Shr),
        Token::BitAnd => (7, BitAnd),
        Token::BitOr => (7, BitOr),
        Token::BitXor => (7, BitXor),
        Token::Shl => (8, Shl),
        Token::Shr => (8, Shr),
        
        _ => return None,
    };
    Some((prec, op))
}

// ---------------------- Helper parsing routines ---------------------------

fn parse_block(t: &[Token], mut i: usize) -> Option<(Vec<Statement>, usize)> {
    use Token::*;
    if t.get(i) != Some(&LBrace) { return None; }
    i += 1;
    let mut stmts = Vec::new();
    while i < t.len() {
        if t.get(i) == Some(&RBrace) {
            return Some((stmts, i + 1));
        }
        match parse_statement(t, i) {
            Some((st, ni)) => {
                stmts.push(st);
                i = ni;
            }
            None => {
                // Skip problematic token to avoid infinite loop
                i += 1;
            }
        }
    }
    None
}

fn parse_if(t: &[Token], i: usize) -> Option<(Statement, usize)> {
    use Token::*;
    // Expect Taste
    if t.get(i) != Some(&Taste) { return None; }
    let (condition, mut j) = parse_expr(t, i + 1)?;
    // Parse then block
    let (then_branch, nj) = parse_block(t, j)?;
    j = nj;
    // Optional else/otherwise
    let (else_branch, j) = if t.get(j) == Some(&Otherwise) {
        parse_block(t, j + 1).map_or((Vec::new(), j), |(b, ni)| (b, ni))
    } else {
        (Vec::new(), j)
    };
    Some((Statement::If { condition, then_branch, else_branch }, j))
}

fn parse_while(t: &[Token], i: usize) -> Option<(Statement, usize)> {
    use Token::*;
    if t.get(i) != Some(&Steep) { return None; }
    let (condition, j) = parse_expr(t, i + 1)?;
    let (body, k) = parse_block(t, j)?;
    Some((Statement::While { condition, body }, k))
}

fn parse_for(t: &[Token], i: usize) -> Option<(Statement, usize)> {
    use Token::*;
    if t.get(i) != Some(&Pour) { return None; }
    let mut j = i + 1;

    // Check if this is a foreach loop: pour var in iterable
    if let Some(Token::Identifier(var_name)) = t.get(j) {
        if t.get(j + 1) == Some(&In) {
            return parse_foreach(t, i);
        }
    }

    // Traditional for loop: pour init; condition; increment { body }
    let init = if t.get(j) == Some(&Semicolon) {
        j += 1;
        None
    } else {
        let (stmt, ni) = parse_statement(t, j)?;
        j = ni;
        if t.get(j) != Some(&Semicolon) { return None; }
        j += 1;
        Some(Box::new(stmt))
    };

    let condition = if t.get(j) == Some(&Semicolon) {
        Expr::Boolean(true)
    } else {
        let (expr, ni) = parse_expr(t, j)?;
        j = ni;
        expr
    };
    if t.get(j) != Some(&Semicolon) { return None; }
    j += 1;

    let increment = if t.get(j) == Some(&LBrace) {
        None
    } else {
        let (expr, ni) = parse_expr(t, j)?;
        j = ni;
        Some(expr)
    };

    let (body, ni) = parse_block(t, j)?;

    Some((Statement::For { init, condition, increment, body }, ni))
}

fn parse_foreach(t: &[Token], i: usize) -> Option<(Statement, usize)> {
    use Token::*;
    if t.get(i) != Some(&Pour) { return None; }
    
    // pour var in iterable { body }
    let var = if let Some(Token::Identifier(name)) = t.get(i + 1) {
        name.clone()
    } else {
        return None;
    };

    if t.get(i + 2) != Some(&In) { return None; }

    let (iterable, ni) = parse_expr(t, i + 3)?;
    let (body, nni) = parse_block(t, ni)?;

    Some((Statement::Foreach { var, iterable, body }, nni))
}

fn parse_roast(t: &[Token], i: usize) -> Option<(Statement, usize)> {
    use Token::*;
    if t.get(i) != Some(&Roast) { return None; }
    // value expression after Roast
    let (value_expr, mut j) = parse_expr(t, i + 1)?;
    if t.get(j) != Some(&LBrace) { return None; }
    j += 1;

    let mut arms = Vec::new();
    let mut default_branch = Vec::new();

    while j < t.len() && t.get(j) != Some(&RBrace) {
        // Skip newlines
        while j < t.len() && t.get(j) == Some(&Newline) { j += 1; }
        if j >= t.len() || t.get(j) == Some(&RBrace) { break; }

        if t.get(j) == Some(&Otherwise) {
            // default arm
            if t.get(j + 1) != Some(&Colon) { return None; }
            let (body, nj) = parse_case_body(t, j + 2)?;
            default_branch = body;
            j = nj;
        } else {
            // case value
            let (case_expr, nj) = parse_expr(t, j)?;
            j = nj;
            if t.get(j) != Some(&Colon) { return None; }
            let (body, nj) = parse_case_body(t, j + 1)?;
            arms.push((case_expr, body));
            j = nj;
        }
    }
    if t.get(j) != Some(&RBrace) { return None; }
    Some((Statement::RoastSwitch { value: value_expr, arms, default: default_branch }, j + 1))
}

fn parse_case_body(t: &[Token], i: usize) -> Option<(Vec<Statement>, usize)> {
    let mut body = Vec::new();
    // Case body can be a block or a single statement
    if t.get(i) == Some(&Token::LBrace) {
        return parse_block(t, i);
    } else {
        // Single statement case
        let (stmt, ni) = parse_statement(t, i)?;
        body.push(stmt);
        return Some((body, ni));
    }
}

fn parse_try_catch(t: &[Token], mut i: usize) -> Option<(Statement, usize)> {
    // taste_carefully { try_body } if_spilled [(<err_var>)] { catch_body }
    if t.get(i) != Some(&Token::TasteCarefully) { return None; }
    i += 1;

    let (try_branch, ni) = parse_block(t, i)?;
    i = ni;

    if t.get(i) != Some(&Token::IfSpilled) { return None; }
    i += 1;

    let mut error_variable = None;
    if t.get(i) == Some(&Token::LParen) {
        if let Some(Token::Identifier(name)) = t.get(i + 1) {
            error_variable = Some(name.clone());
            i += 2;
            if t.get(i) != Some(&Token::RParen) { return None; }
            i += 1;
        } else {
            return None; // Expected identifier in catch
        }
    }

    let (catch_branch, ni) = parse_block(t, i)?;

    Some((Statement::TryCatch { try_branch, error_variable, catch_branch }, ni))
}

fn parse_variable_declaration(t: &[Token], i: usize) -> Option<(Statement, usize)> {
    // Expects 'beans' at t[i]
    if let Some(Token::Identifier(name)) = t.get(i + 1) {
        let mut j = i + 2;
        let mut type_ann = None;

        // Check for optional type annotation
        if t.get(j) == Some(&Token::Colon) {
            if let Some(Token::Identifier(type_name)) = t.get(j + 1) {
                type_ann = Some(type_name.clone());
                j += 2;
            } else {
                return None; // Expected type name after ':'
            }
        }

        if t.get(j) != Some(&Token::PourIn) {
            return None; // Expected '='
        }

        let (value, ni) = parse_expr(t, j + 1)?;
        Some((Statement::VarDecl { name: name.clone(), type_ann, value }, ni))
    } else {
        None
    }
}

fn parse_bean_declaration(t: &[Token], mut i: usize) -> Option<(Statement, usize)> {
    if t.get(i) != Some(&Token::Bean) { return None; }
    i += 1;

    let name = if let Some(Token::Identifier(name)) = t.get(i) {
        name.clone()
    } else {
        return None;
    };
    i += 1;

    let mut parent = None;
    if t.get(i) == Some(&Token::Blend) {
        i += 1;
        if let Some(Token::Identifier(p)) = t.get(i) {
            parent = Some(p.clone());
            i += 1;
        } else {
            return None; // Expected parent name after 'blend'
        }
    }

    if t.get(i) != Some(&Token::LBrace) { return None; }
    i += 1;

    let mut fields = Vec::new();
    let mut methods = Vec::new();

    while i < t.len() && t.get(i) != Some(&Token::RBrace) {
        // Skip newlines
        while i < t.len() && t.get(i) == Some(&Token::Newline) { i += 1; }

        if t.get(i) == Some(&Token::Brew) {
            if let Some((method, ni)) = parse_brew_declaration(t, i) {
                methods.push(method);
                i = ni;
            } else {
                return None; // Failed to parse method
            }
        } else if t.get(i) == Some(&Token::Beans) {
            i += 1; // consume 'beans'
            if let Some(Token::Identifier(name)) = t.get(i) {
                if t.get(i + 1) == Some(&Token::PourIn) {
                    let (value, ni) = parse_expr(t, i + 2)?;
                    fields.push(FieldDecl { name: name.clone(), value });
                    i = ni;
                    // Optional semicolon
                    if t.get(i) == Some(&Token::Semicolon) { i += 1; }
                } else {
                    return None; // Expected 'pour_in' for field
                }
            } else {
                return None; // Expected identifier for field name
            }
        } else if t.get(i) == Some(&Token::RBrace) {
            break;
        } else {
            return None; // Unexpected token in bean body
        }
    }

    if t.get(i) != Some(&Token::RBrace) { return None; }
    i += 1;

    Some((Statement::BeanDecl { name, parent, fields, methods }, i))
}

fn parse_brew_declaration(t: &[Token], mut i: usize) -> Option<(Statement, usize)> {
    if t.get(i) != Some(&Token::Brew) { return None; }
    i += 1;

    let name = if let Some(Token::Identifier(name)) = t.get(i) {
        name.clone()
    } else {
        return None;
    };
    i += 1;

    if t.get(i) != Some(&Token::LParen) { return None; }
    i += 1;

    let (params, ni) = parse_params(t, i)?;
    i = ni;

    if t.get(i) != Some(&Token::RParen) { return None; }
    i += 1;

    let mut return_type = None;
    if t.get(i) == Some(&Token::Colon) {
        i += 1;
        if let Some(Token::Identifier(type_name)) = t.get(i) {
            return_type = Some(type_name.clone());
            i += 1;
        } else {
            return None; // Expected return type
        }
    }

    let (body, ni) = parse_block(t, i)?;
    i = ni;

    Some((Statement::BrewDecl { name, params, body, return_type }, i))
}

fn parse_params(t: &[Token], mut i: usize) -> Option<(Vec<ParamDecl>, usize)> {
    let mut params = Vec::new();
    if t.get(i) == Some(&Token::RParen) {
        return Some((params, i));
    }

    loop {
        let name = if let Some(Token::Identifier(name)) = t.get(i) {
            name.clone()
        } else {
            return None; // Expected parameter name
        };
        i += 1;

        let mut type_name = "Any".to_string(); // Default type
        if t.get(i) == Some(&Token::Colon) {
            i += 1;
            if let Some(Token::Identifier(t_name)) = t.get(i) {
                type_name = t_name.clone();
                i += 1;
            } else {
                return None; // Expected parameter type
            }
        }

        params.push(ParamDecl { name, type_name });

        if t.get(i) == Some(&Token::Comma) {
            i += 1;
        } else {
            break;
        }
    }
    Some((params, i))
}

fn parse_coffee_recipe_declaration(t: &[Token], mut i: usize) -> Option<(Statement, usize)> {
    if t.get(i) != Some(&Token::CoffeeRecipe) { return None; }
    i += 1;

    let name = if let Some(Token::Identifier(name)) = t.get(i) {
        name.clone()
    } else {
        return None;
    };
    i += 1;

    if t.get(i) != Some(&Token::LBrace) { return None; }
    i += 1;

    let mut methods = Vec::new();

    while i < t.len() && t.get(i) != Some(&Token::RBrace) {
        // Skip newlines
        while i < t.len() && t.get(i) == Some(&Token::Newline) { i += 1; }

        if let Some(Token::Identifier(method_name)) = t.get(i) {
            i += 1;
            if t.get(i) != Some(&Token::LParen) { return None; }
            i += 1;

            let (params, ni) = parse_params(t, i)?;
            i = ni;

            if t.get(i) != Some(&Token::RParen) { return None; }
            i += 1;

            let mut return_type = "Any".to_string(); // Default return type
            if t.get(i) == Some(&Token::Arrow) {
                i += 1;
                if let Some(Token::Identifier(type_name)) = t.get(i) {
                    return_type = type_name.clone();
                    i += 1;
                } else {
                    return None; // Expected return type after arrow
                }
            }

            methods.push(MethodSignature {
                name: method_name.clone(),
                params,
                return_type,
            });

            // Optional semicolon or newline
            if t.get(i) == Some(&Token::Semicolon) { i += 1; }
            while i < t.len() && t.get(i) == Some(&Token::Newline) { i += 1; }
        } else if t.get(i) == Some(&Token::RBrace) {
            break;
        } else {
            return None; // Unexpected token in recipe body
        }
    }

    if t.get(i) != Some(&Token::RBrace) { return None; }
    i += 1;

    Some((Statement::CoffeeRecipeDecl { name, methods }, i))
}

/*
 * @khushi_parser_wisdom:
 * "A good parser is like a good barista - 
 *  it takes raw ingredients (tokens) and creates something beautiful (AST)"
 * 
 * This parser represents countless hours of love, debugging,
 * and probably way too much actual coffee consumption! ☕😅
 * 
 * Every parse error message was written with empathy for 
 * future Brewco developers! 💖
 * 
 * Keep parsing with passion! ✨
 * - Khushi Motwani 
 */