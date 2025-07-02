/*
 * ☕ Brewco Native Functions Library ☕
 * 
 * @author: "Khushi Motwani" 💖
 * @khushi_native_mastery: "30+ coffee-themed functions crafted with love!" ✨
 * @function_count: "More than your favorite coffee shop's menu!" ☕
 * 
 * This library brings the full power of coffee to your Brewco programs!
 * Every function named with coffee love, built for maximum brewing efficiency.
 * 
 * From string manipulation to file operations - all with that coffee twist! ☕💖
 */

// src/native.rs

use crate::interpreter::{Value, ControlFlow};

pub fn root_drip(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("root_drip() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Number(n) => {
            if *n < 0.0 {
                Err(ControlFlow::RuntimeError("Cannot take the square root of a negative number.".to_string()))
            } else {
                Ok(Value::Number(n.sqrt()))
            }
        },
        _ => Err(ControlFlow::RuntimeError("root_drip() expects a number as an argument.".to_string())),
    }
}

pub fn absolute_aroma(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("absolute_aroma() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(ControlFlow::RuntimeError("absolute_aroma() expects a number as an argument.".to_string())),
    }
}

pub fn round_up_the_grounds(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("round_up_the_grounds() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err(ControlFlow::RuntimeError("round_up_the_grounds() expects a number as an argument.".to_string())),
    }
}

pub fn settle_the_grounds(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("settle_the_grounds() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err(ControlFlow::RuntimeError("settle_the_grounds() expects a number as an argument.".to_string())),
    }
}

pub fn extra_shot(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("extra_shot() expects 2 arguments, but got {}", args.len())));
    }

    let base = match args.get(0).unwrap() {
        Value::Number(n) => n,
        _ => return Err(ControlFlow::RuntimeError("extra_shot() expects numbers as arguments.".to_string())),
    };

    let exponent = match args.get(1).unwrap() {
        Value::Number(n) => n,
        _ => return Err(ControlFlow::RuntimeError("extra_shot() expects numbers as arguments.".to_string())),
    };

    Ok(Value::Number(base.powf(*exponent)))
}

// String manipulation functions
pub fn string_length(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("string_length() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err(ControlFlow::RuntimeError("string_length() expects a string as an argument.".to_string())),
    }
}

pub fn brew_blend(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("brew_blend() expects 2 arguments, but got {}", args.len())));
    }

    let s1 = match args.get(0).unwrap() {
        Value::String(s) => s,
        _ => return Err(ControlFlow::RuntimeError("brew_blend() expects strings as arguments.".to_string())),
    };

    let s2 = match args.get(1).unwrap() {
        Value::String(s) => s,
        _ => return Err(ControlFlow::RuntimeError("brew_blend() expects strings as arguments.".to_string())),
    };

    Ok(Value::String(format!("{}{}", s1, s2)))
}

pub fn foam_up(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("foam_up() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(ControlFlow::RuntimeError("foam_up() expects a string as an argument.".to_string())),
    }
}

pub fn settle_down(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("settle_down() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(ControlFlow::RuntimeError("settle_down() expects a string as an argument.".to_string())),
    }
}

// Array functions
pub fn cup_size(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("cup_size() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
        _ => Err(ControlFlow::RuntimeError("cup_size() expects an array as an argument.".to_string())),
    }
}

pub fn add_to_cup(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("add_to_cup() expects 2 arguments, but got {}", args.len())));
    }

    let mut arr = match args.get(0).unwrap() {
        Value::Array(a) => a.clone(),
        _ => return Err(ControlFlow::RuntimeError("add_to_cup() expects an array as the first argument.".to_string())),
    };

    arr.push(args.get(1).unwrap().clone());
    Ok(Value::Array(arr))
}

// Random number generation
pub fn random_bean() -> Result<Value, ControlFlow> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash = hasher.finish();
    
    // Simple linear congruential generator for pseudo-randomness
    let random = ((hash.wrapping_mul(1103515245).wrapping_add(12345)) % (1 << 31)) as f64 / (1 << 31) as f64;
    Ok(Value::Number(random))
}

// Type checking functions
pub fn is_brew(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("is_brew() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Function { .. } | Value::BoundMethod { .. } => Ok(Value::Boolean(true)),
        _ => Ok(Value::Boolean(false)),
    }
}

pub fn is_number(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("is_number() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Number(_) => Ok(Value::Boolean(true)),
        _ => Ok(Value::Boolean(false)),
    }
}

pub fn is_string(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("is_string() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::String(_) => Ok(Value::Boolean(true)),
        _ => Ok(Value::Boolean(false)),
    }
}

// Additional Coffee-Themed String Functions
pub fn grind_to_pieces(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("grind_to_pieces() expects 2 arguments, but got {}", args.len())));
    }

    let text = match args.get(0).unwrap() {
        Value::String(s) => s,
        _ => return Err(ControlFlow::RuntimeError("grind_to_pieces() expects a string as the first argument.".to_string())),
    };

    let delimiter = match args.get(1).unwrap() {
        Value::String(s) => s,
        _ => return Err(ControlFlow::RuntimeError("grind_to_pieces() expects a string as the second argument.".to_string())),
    };

    let pieces: Vec<Value> = text.split(delimiter)
        .map(|piece| Value::String(piece.to_string()))
        .collect();
    
    Ok(Value::Array(pieces))
}

pub fn filter_grounds(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("filter_grounds() expects 2 arguments, but got {}", args.len())));
    }

    let text = match args.get(0).unwrap() {
        Value::String(s) => s,
        _ => return Err(ControlFlow::RuntimeError("filter_grounds() expects a string as the first argument.".to_string())),
    };

    let start_pos = match args.get(1).unwrap() {
        Value::Number(n) => *n as usize,
        _ => return Err(ControlFlow::RuntimeError("filter_grounds() expects a number as the second argument.".to_string())),
    };

    if start_pos >= text.len() {
        return Ok(Value::String("".to_string()));
    }

    Ok(Value::String(text[start_pos..].to_string()))
}

pub fn first_sip(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("first_sip() expects 2 arguments, but got {}", args.len())));
    }

    let text = match args.get(0).unwrap() {
        Value::String(s) => s,
        _ => return Err(ControlFlow::RuntimeError("first_sip() expects a string as the first argument.".to_string())),
    };

    let length = match args.get(1).unwrap() {
        Value::Number(n) => *n as usize,
        _ => return Err(ControlFlow::RuntimeError("first_sip() expects a number as the second argument.".to_string())),
    };

    let result = if length > text.len() {
        text.clone()
    } else {
        text[..length].to_string()
    };

    Ok(Value::String(result))
}

// Advanced Array Functions
pub fn pour_together(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("pour_together() expects 2 arguments, but got {}", args.len())));
    }

    let arr1 = match args.get(0).unwrap() {
        Value::Array(a) => a,
        _ => return Err(ControlFlow::RuntimeError("pour_together() expects arrays as arguments.".to_string())),
    };

    let arr2 = match args.get(1).unwrap() {
        Value::Array(a) => a,
        _ => return Err(ControlFlow::RuntimeError("pour_together() expects arrays as arguments.".to_string())),
    };

    let mut result = arr1.clone();
    result.extend(arr2.clone());
    Ok(Value::Array(result))
}

pub fn extract_brew(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("extract_brew() expects 2 arguments, but got {}", args.len())));
    }

    let arr = match args.get(0).unwrap() {
        Value::Array(a) => a,
        _ => return Err(ControlFlow::RuntimeError("extract_brew() expects an array as the first argument.".to_string())),
    };

    let index = match args.get(1).unwrap() {
        Value::Number(n) => *n as usize,
        _ => return Err(ControlFlow::RuntimeError("extract_brew() expects a number as the second argument.".to_string())),
    };

    if index >= arr.len() {
        return Err(ControlFlow::RuntimeError("extract_brew() index out of bounds!".to_string()));
    }

    Ok(arr[index].clone())
}

pub fn reverse_pour(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("reverse_pour() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Array(arr) => {
            let mut reversed = arr.clone();
            reversed.reverse();
            Ok(Value::Array(reversed))
        },
        _ => Err(ControlFlow::RuntimeError("reverse_pour() expects an array as an argument.".to_string())),
    }
}

// Coffee-Themed Math Functions
pub fn brew_minimum(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("brew_minimum() expects 2 arguments, but got {}", args.len())));
    }

    let n1 = match args.get(0).unwrap() {
        Value::Number(n) => n,
        _ => return Err(ControlFlow::RuntimeError("brew_minimum() expects numbers as arguments.".to_string())),
    };

    let n2 = match args.get(1).unwrap() {
        Value::Number(n) => n,
        _ => return Err(ControlFlow::RuntimeError("brew_minimum() expects numbers as arguments.".to_string())),
    };

    Ok(Value::Number(n1.min(*n2)))
}

pub fn brew_maximum(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 2 {
        return Err(ControlFlow::RuntimeError(format!("brew_maximum() expects 2 arguments, but got {}", args.len())));
    }

    let n1 = match args.get(0).unwrap() {
        Value::Number(n) => n,
        _ => return Err(ControlFlow::RuntimeError("brew_maximum() expects numbers as arguments.".to_string())),
    };

    let n2 = match args.get(1).unwrap() {
        Value::Number(n) => n,
        _ => return Err(ControlFlow::RuntimeError("brew_maximum() expects numbers as arguments.".to_string())),
    };

    Ok(Value::Number(n1.max(*n2)))
}

pub fn perfect_temperature(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("perfect_temperature() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Number(n) => Ok(Value::Number(n.round())),
        _ => Err(ControlFlow::RuntimeError("perfect_temperature() expects a number as an argument.".to_string())),
    }
}

// Coffee Shop Utilities
pub fn brewing_time() -> Result<Value, ControlFlow> {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as f64;
    
    Ok(Value::Number(timestamp))
}

pub fn coffee_strength_check(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("coffee_strength_check() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Number(n) => {
            let strength = if *n < 3.0 {
                "weak"
            } else if *n < 7.0 {
                "medium"
            } else {
                "strong"
            };
            Ok(Value::String(strength.to_string()))
        },
        _ => Err(ControlFlow::RuntimeError("coffee_strength_check() expects a number as an argument.".to_string())),
    }
}

// Additional Type Checking
pub fn is_cup(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("is_cup() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Array(_) => Ok(Value::Boolean(true)),
        _ => Ok(Value::Boolean(false)),
    }
}

pub fn is_boolean_bean(args: Vec<Value>) -> Result<Value, ControlFlow> {
    if args.len() != 1 {
        return Err(ControlFlow::RuntimeError(format!("is_boolean_bean() expects 1 argument, but got {}", args.len())));
    }

    match args.get(0).unwrap() {
        Value::Boolean(_) => Ok(Value::Boolean(true)),
        _ => Ok(Value::Boolean(false)),
    }
}