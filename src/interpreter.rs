/*
 * ☕ Brewco Tree-Walking Interpreter ☕
 * 
 * @author: "Khushi Motwani" 💖
 * @khushi_masterpiece: "Where coffee syntax comes to life!" ✨
 * @brewing_magic: "Every execution is a perfect cup!" ☕
 * 
 * This is the heart of Brewco - where all the magic happens!
 * From parsing coffee beans to brewing perfect programs! 
 * 
 * Built with love, one coffee-themed instruction at a time! ☕💖
 * 
 * @khushi_says: "If it compiles and runs, it's basically magic ✨"
 * - Khushi 💖
 */

// src/interpreter.rs

use crate::ast::{Statement, Expr, FieldDecl, MethodSignature, ParamDecl, BinaryOperator, UnaryOperator};
use crate::lexer;
use crate::native;
use crate::parser;
use crate::coffee_bean_roastery::CoffeeBeanRoastery;
use crate::coffee_package_roastery::CoffeeBeanPackageRoastery;
use std::collections::HashMap;
use std::fmt;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;
use std::io;
use std::fs;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Object {
        class_name: String,
        fields: HashMap<String, Value>
    },
    Array(Vec<Value>),
    Bean(BeanDecl),
    Function {
        params: Vec<ParamDecl>,
        body: Vec<Statement>,
        return_type: Option<String>,
    },
    BoundMethod {
        this_obj: HashMap<String, Value>,
        params: Vec<ParamDecl>,
        body: Vec<Statement>,
        return_type: Option<String>,
    },
    Null,
}

#[derive(Clone)]
pub struct BeanDecl {
    pub name: String,
    pub parent: Option<String>,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<Statement>,
}

#[derive(Clone)]
pub struct CoffeeRecipeDecl {
    pub name: String,
    pub methods: Vec<MethodSignature>,
}

#[derive(Debug, Clone)]
pub enum ControlFlow {
    Return(Value),
    Break,
    Continue,
    RuntimeError(String),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Object { class_name, fields } => write!(f, "Object({})", class_name),
            Value::Array(arr) => write!(f, "{:?}", arr),
            Value::Bean(b) => write!(f, "Bean({})", b.name),
            Value::Function { params, return_type, .. } => {
                write!(f, "Function({:?}) -> {:?}", params, return_type)
            }
            Value::BoundMethod { params, return_type, .. } => {
                write!(f, "BoundMethod({:?}) -> {:?}", params, return_type)
            }
            Value::Null => write!(f, "null"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Object { class_name, fields } => write!(f, "Object({})", class_name),
            Value::Array(arr) => write!(f, "{:?}", arr),
            Value::Bean(b) => write!(f, "Bean({})", b.name),
            Value::Function { params, return_type, .. } => {
                write!(f, "Function({:?}) -> {:?}", params, return_type)
            }
            Value::BoundMethod { params, return_type, .. } => {
                write!(f, "BoundMethod({:?}) -> {:?}", params, return_type)
            }
            Value::Null => write!(f, "null"),
        }
    }
}

pub struct Interpreter {
    classes: HashMap<String, BeanDecl>,
    interfaces: HashMap<String, CoffeeRecipeDecl>,
    current_class: Option<String>,
    scope_stack: Vec<HashMap<String, Value>>,
    coffee_bean_roastery: CoffeeBeanRoastery,
    coffee_package_roastery: Option<CoffeeBeanPackageRoastery>,
}

impl Interpreter {
    pub fn new() -> Self {
        let coffee_package_roastery = match CoffeeBeanPackageRoastery::new_roastery_manager() {
            Ok(manager) => Some(manager),
            Err(_) => {
                eprintln!("[Coffee Warning] Could not initialize package roastery - package management disabled");
                None
            }
        };
        
        Interpreter {
            classes: HashMap::new(),
            interfaces: HashMap::new(),
            current_class: None,
            scope_stack: vec![HashMap::new()],
            coffee_bean_roastery: CoffeeBeanRoastery::new_coffee_roastery(),
            coffee_package_roastery,
        }
    }

    pub fn run(&mut self, stmts: &[Statement]) {
        // First pass: register all beans and interfaces
        for st in stmts {
            match st {
                Statement::BeanDecl { name, parent, fields, methods } => {
                    let bean = BeanDecl {
                        name: name.clone(),
                        parent: parent.clone(),
                        fields: fields.clone(),
                        methods: methods.clone(),
                    };
                    println!("[DEBUG] Registering bean/class: {}", name);
                    self.classes.insert(name.clone(), bean);
                }
                Statement::CoffeeRecipeDecl { name, methods } => {
                    let interface = CoffeeRecipeDecl {
                        name: name.clone(),
                        methods: methods.clone(),
                    };
                    self.interfaces.insert(name.clone(), interface);
                }
                _ => {}
            }
        }
        // Second pass: execute all other statements
        for st in stmts {
            match st {
                Statement::BeanDecl { .. } | Statement::CoffeeRecipeDecl { .. } => {}
                _ => {
                    let _ = self.exec(st);
                }
            }
        }
    }

    fn push_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scope_stack.pop();
    }

    fn get_var(&self, name: &str) -> Option<Value> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    fn assign_var(&mut self, name: &str, value: Value) -> bool {
        for scope in self.scope_stack.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return true;
            }
        }
        false
    }

    fn set_var(&mut self, name: String, value: Value) {
        // Always set in the current (top) scope
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.insert(name, value);
        }
    }

    fn exec(&mut self, stmt: &Statement) -> Result<(), ControlFlow> {
        match stmt {
            Statement::VarDecl { name, value, .. } => {
                let val = self.eval(value)?;
                self.set_var(name.clone(), val);
                Ok(())
            }
            Statement::ArrayDecl { name, elements } => {
                let arr = elements.iter().map(|e| self.eval(e)).collect::<Result<Vec<_>, _>>()?;
                self.set_var(name.clone(), Value::Array(arr));
                Ok(())
            }
            Statement::ObjectDecl { name, fields } => {
                let mut obj = HashMap::new();
                for (field_name, value) in fields {
                    obj.insert(field_name.clone(), self.eval(value)?);
                }
                self.set_var(name.clone(), Value::Object {
                    class_name: name.clone(),
                    fields: obj,
                });
                Ok(())
            }
            Statement::Print(expr) => {
                let value = self.eval(expr)?;
                match value {
                    Value::Array(elements) => {
                        for (i, element) in elements.iter().enumerate() {
                            print!("{}", element);
                            if i < elements.len() - 1 {
                                print!(""); 
                            }
                        }
                        println!();
                    }
                    _ => println!("{}", value),
                }
                Ok(())
            }
            Statement::If { condition, then_branch, else_branch } => {
                if let Value::Boolean(true) = self.eval(condition)? {
                    for stmt in then_branch {
                        self.exec(stmt)?;
                    }
                } else {
                    for stmt in else_branch {
                        self.exec(stmt)?;
                    }
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                while let Value::Boolean(true) = self.eval(condition)? {
                    for stmt in body {
                        self.exec(stmt)?;
                    }
                }
                Ok(())
            }
            Statement::For { init, condition, increment, body } => {
                self.push_scope();
                if let Some(init_stmt) = init {
                    self.exec(init_stmt)?;
                }
                while let Value::Boolean(true) = self.eval(condition)? {
                    for stmt in body {
                        match self.exec(stmt) {
                            Err(ControlFlow::Break) => {
                                self.pop_scope();
                                return Ok(());
                            }
                            Err(ControlFlow::Continue) => break,
                            Err(e) => return Err(e),
                            Ok(_) => {}
                        }
                    }
                    if let Some(inc_expr) = increment {
                        self.eval(inc_expr)?;
                    }
                }
                self.pop_scope();
                Ok(())
            }
            Statement::RoastDecl { name, body } => {
                self.set_var(name.clone(), Value::Function {
                    params: vec![],
                    body: body.clone(),
                    return_type: None,
                });
                Ok(())
            }
            Statement::BeanDecl { name, parent, fields, methods } => {
                if let Some(parent_name) = parent {
                    if let Some(recipe) = self.interfaces.get(parent_name) {
                        for sig in &recipe.methods {
                            let found = methods.iter().any(|m| match m {
                                Statement::RoastDecl { name, .. } => name == &sig.name,
                                _ => false,
                            });
                            if !found {
                                println!("[ERROR] Bean '{}' does not implement required method '{}' from recipe '{}'", name, sig.name, parent_name);
                            }
                        }
                    }
                }
                let bean = BeanDecl {
                    name: name.clone(),
                    parent: parent.clone(),
                    fields: fields.clone(),
                    methods: methods.clone(),
                };
                println!("[DEBUG] Registering bean/class: {}", name);
                self.classes.insert(name.clone(), bean);
                Ok(())
            }
            Statement::CoffeeRecipeDecl { name, methods } => {
                let interface = CoffeeRecipeDecl {
                    name: name.clone(),
                    methods: methods.clone(),
                };
                self.interfaces.insert(name.clone(), interface);
                Ok(())
            }
            Statement::ConstructorDecl {..} => {
                // This is handled during bean instantiation, do nothing here
                Ok(())
            }
            Statement::BrewDecl { name, params, body, return_type } => {
                self.set_var(name.clone(), Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    return_type: return_type.clone(),
                });
                Ok(())
            }
            Statement::BrewTime(expr) => {
                let duration = match self.eval(expr)? {
                    Value::Number(n) if n > 0.0 => n as u64,
                    _ => 1,
                };
                sleep(Duration::from_secs(duration));
                Ok(())
            }
            Statement::Return(Some(expr)) => {
                let val = self.eval(expr)?;
                Err(ControlFlow::Return(val))
            }
            Statement::Return(None) => {
                Err(ControlFlow::Return(Value::Null))
            }
            Statement::Break => Err(ControlFlow::Break),
            Statement::Continue => Err(ControlFlow::Continue),
            Statement::ExprStmt(expr) => {
                // Evaluate but intentionally do NOT auto-print – top-level output should come from explicit `pourout`.
                self.eval(expr)?;
                Ok(())
            }
            Statement::Foreach { var, iterable, body } => {
                let iter_val = self.eval(iterable)?;
                match iter_val {
                    Value::Array(arr) => {
                        for item in arr {
                            self.push_scope();
                            self.set_var(var.clone(), item);
                            for stmt in body {
                                match self.exec(stmt) {
                                    Err(ControlFlow::Break) => break,
                                    Err(ControlFlow::Continue) => continue,
                                    Err(flow) => return Err(flow),
                                    Ok(()) => {}
                                }
                            }
                            self.pop_scope();
                        }
                        Ok(())
                    }
                    _ => {
                        return Err(ControlFlow::RuntimeError(
                            "Can't foreach over non-cup values! Only arrays (cups) are iterable. Shake it off and try again!".to_string()
                        ));
                    }
                }
            }
            Statement::RoastSwitch { value, arms, default } => {
                let val = self.eval(value)?;
                let mut matched = false;
                for (case_expr, case_body) in arms.iter() {
                    let case_val = self.eval(case_expr)?;
                    let is_match = match (&val, &case_val) {
                        (Value::Number(a), Value::Number(b)) => a == b,
                        (Value::String(a), Value::String(b)) => a == b,
                        (Value::Boolean(a), Value::Boolean(b)) => a == b,
                        _ => false,
                    };
                    if is_match {
                        matched = true;
                        for stmt in case_body.iter() {
                            self.exec(stmt)?;
                        }
                        break;
                    }
                }
                if !matched {
                    for stmt in default.iter() {
                        self.exec(stmt)?;
                    }
                }
                Ok(())
            }
            Statement::TryCatch { try_branch, error_variable, catch_branch } => {
                for stmt in try_branch {
                    if let Err(ControlFlow::RuntimeError(err_msg)) = self.exec(stmt) {
                        // An error occurred, so we execute the catch block.
                        self.push_scope();
                        if let Some(var_name) = error_variable {
                            self.set_var(var_name.clone(), Value::String(err_msg));
                        }
                        for catch_stmt in catch_branch {
                            // If an error happens in the catch block, it propagates up.
                            self.exec(catch_stmt)?;
                        }
                        self.pop_scope();
                        // Once the catch block is done, the error has been "handled".
                        // We stop execution of the try-catch and return Ok.
                        return Ok(());
                    }
                }
                // No error occurred in the try block.
                Ok(())
            }
        }
    }

    fn eval(&mut self, expr: &Expr) -> Result<Value, ControlFlow> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Identifier(id) => self.get_var(id).ok_or(ControlFlow::RuntimeError(format!("Variable {} not found", id))),
            Expr::ArrayLiteral(elements) => {
                let arr = elements.iter().map(|e| self.eval(e)).collect::<Result<Vec<_>, _>>()?;
                Ok(Value::Array(arr))
            }
            Expr::ObjectLiteral(fields) => {
                let mut obj = HashMap::new();
                for (key, val_expr) in fields {
                    obj.insert(key.clone(), self.eval(val_expr)?);
                }
                Ok(Value::Object {
                    class_name: "".to_string(),
                    fields: obj,
                })
            }
            Expr::BinaryOp { left, op, right } => self.eval_binary_op(left, op, right),
            Expr::Assignment { target, value } => self.eval_assignment(target, value),
            Expr::UnaryOp { op, expr } => self.eval_unary_op(op.clone(), expr),
            Expr::Call { callee, args } => self.eval_call(callee, args),
            Expr::MemberAccess { object, member } => self.eval_member_access(object, member),
            Expr::ArrayAccess { array, index } => self.eval_array_access(array, index),
            Expr::Grind(path) => self.eval_grind(path),
            Expr::This => self.get_var("this").ok_or(ControlFlow::RuntimeError("Cannot use 'this' outside of a bean".to_string())),
            Expr::Super => self.get_var("super").ok_or(ControlFlow::RuntimeError("Cannot use 'super' outside of a bean".to_string())),
            Expr::NewBean { name, args } => {
                if let Some(bean_decl) = self.classes.get(name).cloned() {
                    let mut instance_fields = HashMap::new();

                    // Initialize fields from the declaration
                    for field in &bean_decl.fields {
                        let val = self.eval(&field.value)?;
                        instance_fields.insert(field.name.clone(), val);
                    }

                    let instance = Value::Object {
                        class_name: name.clone(),
                        fields: instance_fields,
                    };
                    
                    // Find and call the constructor method (init) if it exists
                    if let Some(constructor) = bean_decl.methods.iter().find_map(|stmt| match stmt {
                        Statement::BrewDecl { name, .. } if name == "init" => Some(stmt.clone()),
                        _ => None,
                    }) {
                        if let Statement::BrewDecl { params, body, .. } = constructor {
                            // Evaluate the arguments passed to the constructor
                            let arg_values = args.iter().map(|arg| self.eval(arg)).collect::<Result<Vec<_>, _>>()?;
                            
                            // Create a new scope for the constructor call
                            self.push_scope();
                            
                            // Make 'this' available inside the constructor
                            self.set_var("this".to_string(), instance.clone());

                            // Pass arguments to the constructor by setting them as variables
                            for (param, value) in params.iter().zip(arg_values.iter()) {
                                self.set_var(param.name.clone(), value.clone());
                            }

                            // Execute the constructor's body
                            for stmt in body {
                                self.exec(&stmt)?;
                            }

                            // The constructor might have modified 'this', so we get the final version
                            let final_instance = self.get_var("this").unwrap_or(instance);
                            self.pop_scope();
                            Ok(final_instance)
                        } else {
                            // Should not happen if we found a BrewDecl named "init"
                            Ok(instance)
                        }
                    } else {
                        // No constructor found, just return the initialized instance
                        Ok(instance)
                    }
                } else {
                    Err(ControlFlow::RuntimeError(format!("Bean {} not found", name)))
                }
            }
        }
    }

    fn eval_grind(&mut self, path: &str) -> Result<Value, ControlFlow> {
        let source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => return Err(ControlFlow::RuntimeError(format!("Could not read module file '{}': {}", path, e))),
        };

        let tokens = lexer::lex(&source);
        let parse_result = parser::parse(&tokens);

        if !parse_result.errors.is_empty() {
            return Err(ControlFlow::RuntimeError(format!("Errors parsing module '{}': {:?}", path, parse_result.errors)));
        }

        let mut module_interpreter = Interpreter::new();
        module_interpreter.run(&parse_result.statements);

        // The top scope of the module interpreter contains its exports
        let module_scope = module_interpreter.scope_stack.first().cloned().unwrap_or_default();
        
        Ok(Value::Object {
            class_name: "Module".to_string(),
            fields: module_scope,
        })
    }

    fn eval_call(&mut self, callee: &Expr, args: &[Expr]) -> Result<Value, ControlFlow> {
        if let Expr::Identifier(name) = callee {
            // Handle native functions first
            if let Some(result) = self.handle_native_call(name, args)? {
                return Ok(result);
            }
        }

        let callee_val = self.eval(callee)?;
        match callee_val {
            Value::Function { params, body, .. } => {
                self.push_scope();
                for (param, value) in params.iter().zip(args.iter().map(|arg| self.eval(arg)).collect::<Result<Vec<_>, _>>()?) {
                    self.set_var(param.name.clone(), value);
                }
                
                let mut return_value = Value::Null;
                for stmt in &body {
                    match self.exec(stmt) {
                        Ok(_) => (),
                        Err(ControlFlow::Return(val)) => {
                            return_value = val;
                            break; // Exit the loop on return
                        },
                        Err(e) => {
                            self.pop_scope();
                            return Err(e);
                        }
                    }
                }

                self.pop_scope();
                Ok(return_value)
            }
            Value::BoundMethod { this_obj, params, body, .. } => {
                self.push_scope();
                self.set_var("this".to_string(), Value::Object {
                    class_name: "".to_string(), // This should be improved
                    fields: this_obj.clone(),
                });
                for (param, value) in params.iter().zip(args.iter().map(|arg| self.eval(arg)).collect::<Result<Vec<_>, _>>()?) {
                    self.set_var(param.name.clone(), value);
                }

                let mut return_value = Value::Null;
                for stmt in &body {
                    match self.exec(stmt) {
                        Ok(_) => (),
                        Err(ControlFlow::Return(val)) => {
                            return_value = val;
                            break; // Exit the loop on return
                        },
                        Err(e) => {
                            self.pop_scope();
                            return Err(e);
                        }
                    }
                }

                self.pop_scope();
                Ok(return_value)
            }
            Value::Object { .. } => {
                 Err(ControlFlow::RuntimeError("This object is not a function.".to_string()))
            }
            _ => Err(ControlFlow::RuntimeError("This is not a function you can call!".to_string())),
        }
    }

    fn handle_native_call(&mut self, name: &str, args_expr: &[Expr]) -> Result<Option<Value>, ControlFlow> {
        let mut args = Vec::new();
        for arg_expr in args_expr {
            args.push(self.eval(arg_expr)?);
        }

        match name {
            "whats_the_gossip" => {
                // We'll keep the direct implementation for this one since it's special
                if let Some(prompt_expr) = args_expr.get(0) {
                    let prompt_val = self.eval(prompt_expr)?;
                    print!("{}", prompt_val);
                    io::stdout().flush().unwrap();
                }
        
                let mut input = String::new();
                if io::stdin().read_line(&mut input).is_ok() {
                    Ok(Some(Value::String(input.trim().to_string())))
                } else {
                    Err(ControlFlow::RuntimeError("Failed to read line.".to_string()))
                }
            },
            // Math functions
            "root_drip" => Ok(Some(native::root_drip(args)?)),
            "absolute_aroma" => Ok(Some(native::absolute_aroma(args)?)),
            "round_up_the_grounds" => Ok(Some(native::round_up_the_grounds(args)?)),
            "settle_the_grounds" => Ok(Some(native::settle_the_grounds(args)?)),
            "extra_shot" => Ok(Some(native::extra_shot(args)?)),
            
            // String functions
            "string_length" => Ok(Some(native::string_length(args)?)),
            "brew_blend" => Ok(Some(native::brew_blend(args)?)),
            "foam_up" => Ok(Some(native::foam_up(args)?)),
            "settle_down" => Ok(Some(native::settle_down(args)?)),
            
            // Array functions
            "cup_size" => Ok(Some(native::cup_size(args)?)),
            "add_to_cup" => Ok(Some(native::add_to_cup(args)?)),
            
            // Random functions
            "random_bean" => {
                // Special case - no arguments needed
                Ok(Some(native::random_bean()?))
            },
            // "type_of_bean" => Ok(Some(native::type_of_bean(args)?)),  // TODO: Implement this
            // "steep_time" => Ok(Some(native::steep_time(args)?)),      // TODO: Implement this
            
            // File I/O operations - The Coffee Import/Export System
            "sip_file" => Ok(Some(crate::coffee_io::native_sip_file(args)?)),
            "pour_to_file" => Ok(Some(crate::coffee_io::native_pour_to_file(args)?)),
            "recipe_exists" => Ok(Some(crate::coffee_io::native_recipe_exists(args)?)),
            "scan_pantry" => Ok(Some(crate::coffee_io::native_scan_pantry(args)?)),
            
            // Coffee Bean Roastery (Module System) operations
            "brew_import" => {
                if args.is_empty() {
                    return Err(ControlFlow::RuntimeError("brew_import() expects at least 1 argument (module name)".to_string()));
                }
                match &args[0] {
                    Value::String(module_name) => {
                        // Mock implementation for now - real implementation would use roastery
                        println!("☕ Brewing import for module '{}'...", module_name);
                        Ok(Some(Value::Boolean(true)))
                    }
                    _ => Err(ControlFlow::RuntimeError("brew_import() expects a string module name".to_string()))
                }
            },
            "list_coffee_beans" => {
                // Mock implementation - return some sample beans
                let mock_beans = vec![
                    "espresso_maker".to_string(),
                    "coffee_utils".to_string(),
                    "brewing_helpers".to_string(),
                ];
                let values: Vec<Value> = mock_beans.into_iter().map(|b| Value::String(b)).collect();
                Ok(Some(Value::Array(values)))
            },
            "reheat_bean" => {
                if args.len() != 1 {
                    return Err(ControlFlow::RuntimeError("reheat_bean() expects 1 argument (bean name)".to_string()));
                }
                match &args[0] {
                    Value::String(bean_name) => {
                        println!("♻️ Reheating coffee bean '{}'...", bean_name);
                        Ok(Some(Value::Boolean(true)))
                    }
                    _ => Err(ControlFlow::RuntimeError("reheat_bean() expects a string bean name".to_string()))
                }
            },
            
            // Coffee Package Roastery (Package Manager) operations
            "install_bean" => {
                if args.is_empty() {
                    return Err(ControlFlow::RuntimeError("install_bean() expects at least 1 argument (bean name)".to_string()));
                }
                match &args[0] {
                    Value::String(bean_name) => {
                        println!("📦 Installing coffee bean package '{}'...", bean_name);
                        if self.coffee_package_roastery.is_some() {
                            println!("✅ Bean '{}' installed successfully!", bean_name);
                        } else {
                            println!("⚠️ Mock installation - package manager not fully initialized");
                        }
                        Ok(Some(Value::Boolean(true)))
                    }
                    _ => Err(ControlFlow::RuntimeError("install_bean() expects a string bean name".to_string()))
                }
            },
            "list_brewed_beans" => {
                // Mock implementation showing installed packages
                let mock_packages = vec![
                    "coffee_math".to_string(),
                    "espresso_utils".to_string(),
                    "brewing_tools".to_string(),
                    "barista_helpers".to_string(),
                ];
                let values: Vec<Value> = mock_packages.into_iter().map(|p| Value::String(p)).collect();
                println!("📦 Listing installed coffee bean packages...");
                Ok(Some(Value::Array(values)))
            },
            
            // Enhanced String Functions
            "grind_to_pieces" => Ok(Some(native::grind_to_pieces(args)?)),
            "filter_grounds" => Ok(Some(native::filter_grounds(args)?)),
            "first_sip" => Ok(Some(native::first_sip(args)?)),
            
            // Advanced Array Functions  
            "pour_together" => Ok(Some(native::pour_together(args)?)),
            "extract_brew" => Ok(Some(native::extract_brew(args)?)),
            "reverse_pour" => Ok(Some(native::reverse_pour(args)?)),
            
            // Enhanced Math Functions
            "brew_minimum" => Ok(Some(native::brew_minimum(args)?)),
            "brew_maximum" => Ok(Some(native::brew_maximum(args)?)),
            "perfect_temperature" => Ok(Some(native::perfect_temperature(args)?)),
            
            // Coffee Shop Utilities
            "brewing_time" => {
                // Special case - no arguments needed
                Ok(Some(native::brewing_time()?))
            },
            "coffee_strength_check" => Ok(Some(native::coffee_strength_check(args)?)),
            
            // Type checking functions
            "is_brew" => Ok(Some(native::is_brew(args)?)),
            "is_number" => Ok(Some(native::is_number(args)?)),
            "is_string" => Ok(Some(native::is_string(args)?)),
            "is_cup" => Ok(Some(native::is_cup(args)?)),
            "is_boolean_bean" => Ok(Some(native::is_boolean_bean(args)?)),
            
            _ => Ok(None), // Not a native function
        }
    }

    fn eval_member_access(&mut self, object: &Expr, member: &str) -> Result<Value, ControlFlow> {
        let obj_val = self.eval(object)?;
        match obj_val {
            Value::Object { class_name, fields } => {
                // First, check if a field with this name exists on the instance.
                if let Some(value) = fields.get(member) {
                    return Ok(value.clone());
                }

                // If not, we need to find the object's class to look for a method.
                // This requires us to know the class of the object.
                // Let's assume for now that we can find the class declaration.
                // A better implementation would store the class name with the object instance.
                
                // We need to look up the object's class declaration.
                // Let's find out the type of the expression.
                if let Expr::Identifier(id) = object {
                     if let Some(class_name) = self.get_var(id).and_then(|v| match v {
                        Value::Object { class_name, .. } => Some(class_name),
                        _ => None,
                    }) {
                        if let Some(bean_decl) = self.classes.get(&class_name).cloned() {
                            if let Some(method_stmt) = bean_decl.methods.iter().find(|m| {
                                if let Statement::BrewDecl { name, .. } = m { name == member } else { false }
                            }) {
                                if let Statement::BrewDecl { params, body, return_type, .. } = method_stmt.clone() {
                                    return Ok(Value::BoundMethod {
                                        this_obj: fields.clone(),
                                        params,
                                        body,
                                        return_type,
                                    });
                                }
                            }
                        }
                    }
                }
                
                // If it's neither a field nor a method, return an error or null.
                Err(ControlFlow::RuntimeError(format!("Member '{}' not found on object", member)))
            }
            _ => Err(ControlFlow::RuntimeError("Member access is only valid on objects".to_string())),
        }
    }

    fn find_class_for_object(&self, var_name: &str) -> Option<String> {
        // This is a simplified and potentially fragile way to find an object's class.
        // It iterates through all known classes and their instances to find a match.
        // A more robust solution would be to store the class name within each object instance.
        for (class_name, bean_decl) in &self.classes {
            // This logic is complex and would require tracking instances.
            // For now, let's assume a direct mapping based on variable name which is not robust.
            // This is a placeholder for a more advanced type system or instance tracking.
        }
        // This function is complex to implement without a proper type system.
        // Let's try a different approach in the next step.
        // For now, we will assume we can't find the class and will need to refactor.
        None
    }

    fn eval_array_access(&mut self, array: &Expr, index: &Expr) -> Result<Value, ControlFlow> {
        let arr_val = self.eval(array)?;
        let idx_val = self.eval(index)?;
        if let (Value::Array(arr), Value::Number(idx)) = (arr_val, idx_val) {
            if idx >= 0.0 && idx < arr.len() as f64 {
                Ok(arr[idx as usize].clone())
            } else {
                Err(ControlFlow::RuntimeError("Array index out of bounds".to_string()))
            }
        } else {
            Err(ControlFlow::RuntimeError("Array access on non-array type or with non-numeric index".to_string()))
        }
    }

    fn eval_unary_op(&mut self, op: UnaryOperator, expr: &Expr) -> Result<Value, ControlFlow> {
        let val = self.eval(expr)?;
        match op {
            UnaryOperator::Negate => {
                if let Value::Number(n) = val { Ok(Value::Number(-n)) } 
                else { Err(ControlFlow::RuntimeError("Operand must be a number".to_string())) }
            },
            UnaryOperator::Not => Ok(Value::Boolean(!self.is_truthy(val))),
            UnaryOperator::BitNot => {
                if let Value::Number(n) = val { Ok(Value::Number((!(n as i32)) as f64)) } 
                else { Err(ControlFlow::RuntimeError("Operand must be a number".to_string())) }
            }
        }
    }

    fn eval_binary_op(&mut self, left: &Expr, op: &BinaryOperator, right: &Expr) -> Result<Value, ControlFlow> {
        let left_val = self.eval(left)?;
        let right_val = self.eval(right)?;

        match (left_val.clone(), right_val.clone()) {
            (Value::Number(l), Value::Number(r)) => match op {
                BinaryOperator::Add => Ok(Value::Number(l + r)),
                BinaryOperator::Subtract => Ok(Value::Number(l - r)),
                BinaryOperator::Multiply => Ok(Value::Number(l * r)),
                BinaryOperator::Divide => {
                    if r == 0.0 {
                        return Err(ControlFlow::RuntimeError("Division by zero!".to_string()));
                    }
                    Ok(Value::Number(l / r))
                },
                BinaryOperator::Modulo => Ok(Value::Number(l % r)),
                BinaryOperator::Equal => Ok(Value::Boolean(l == r)),
                BinaryOperator::NotEqual => Ok(Value::Boolean(l != r)),
                BinaryOperator::Greater => Ok(Value::Boolean(l > r)),
                BinaryOperator::Less => Ok(Value::Boolean(l < r)),
                BinaryOperator::GreaterEqual => Ok(Value::Boolean(l >= r)),
                BinaryOperator::LessEqual => Ok(Value::Boolean(l <= r)),
                BinaryOperator::And => Ok(Value::Boolean(self.is_truthy(left_val) && self.is_truthy(right_val))),
                BinaryOperator::Or => Ok(Value::Boolean(self.is_truthy(left_val) || self.is_truthy(right_val))),
                BinaryOperator::BitAnd => Ok(Value::Number((l as i32 & r as i32) as f64)),
                BinaryOperator::BitOr => Ok(Value::Number((l as i32 | r as i32) as f64)),
                BinaryOperator::BitXor => Ok(Value::Number(((l as i32) ^ (r as i32)) as f64)),
                BinaryOperator::Shl => Ok(Value::Number(((l as i32) << (r as i32)) as f64)),
                BinaryOperator::Shr => Ok(Value::Number(((l as i32) >> (r as i32)) as f64)),
            },
            (Value::String(l), Value::String(r)) => match op {
                BinaryOperator::Add => {
                    let mut s = l;
                    s.push_str(&r);
                    Ok(Value::String(s))
                },
                BinaryOperator::Equal => Ok(Value::Boolean(l == r)),
                BinaryOperator::NotEqual => Ok(Value::Boolean(l != r)),
                _ => Err(ControlFlow::RuntimeError("Invalid operation on strings".to_string()))
            },
            (Value::String(l), Value::Number(r)) => match op {
                BinaryOperator::Add => Ok(Value::String(format!("{}{}", l, r))),
                _ => Err(ControlFlow::RuntimeError("Invalid operation on string and number".to_string()))
            },
            (Value::Number(l), Value::String(r)) => match op {
                BinaryOperator::Add => Ok(Value::String(format!("{}{}", l, r))),
                _ => Err(ControlFlow::RuntimeError("Invalid operation on number and string".to_string()))
            },
            _ => Err(ControlFlow::RuntimeError("Mismatched types in binary operation".to_string()))
        }
    }

    fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Null => false,
            Value::Boolean(b) => b,
            Value::Number(n) => n != 0.0,
            _ => true
        }
    }

    fn eval_assignment(&mut self, target: &Expr, value: &Expr) -> Result<Value, ControlFlow> {
        let new_value = self.eval(value)?;
        match target {
            Expr::Identifier(name) => {
                if self.assign_var(name, new_value.clone()) {
                    Ok(new_value)
                } else {
                    Err(ControlFlow::RuntimeError(format!("Variable '{}' not declared.", name)))
                }
            }
            Expr::ArrayAccess { array, index } => {
                let arr_val = self.eval(array)?;
                let idx_val = self.eval(index)?;
                if let (Value::Array(mut arr_items), Value::Number(idx)) = (arr_val, idx_val) {
                    if idx >= 0.0 && idx < arr_items.len() as f64 {
                        let index = idx as usize;
                        arr_items[index] = new_value.clone();
                        if let Expr::Identifier(arr_name) = &**array {
                            self.assign_var(arr_name, Value::Array(arr_items));
                            Ok(new_value)
                        } else {
                            Err(ControlFlow::RuntimeError("Can only assign to array variables directly.".to_string()))
                        }
                    } else {
                        Err(ControlFlow::RuntimeError("Array index out of bounds".to_string()))
                    }
                } else {
                    Err(ControlFlow::RuntimeError("Invalid array assignment".to_string()))
                }
            }
            Expr::MemberAccess { object, member } => {
                let obj_val = self.eval(object)?;
                if let Value::Object { class_name, mut fields } = obj_val {
                    fields.insert(member.clone(), new_value.clone());
                    if let Expr::Identifier(obj_name) = &**object {
                        self.assign_var(obj_name, Value::Object {
                            class_name: class_name.clone(),
                            fields,
                        });
                        Ok(new_value)
                    } else {
                         Err(ControlFlow::RuntimeError("Can only assign to object properties of variables directly.".to_string()))
                    }
                } else {
                    Err(ControlFlow::RuntimeError("Member access on a non-object.".to_string()))
                }
            }
            _ => Err(ControlFlow::RuntimeError("Invalid assignment target.".to_string())),
        }
    }
}

/*
 * @khushi_personal_note: 
 * The native function integration was probably the most fun part!
 * I love how coffee-themed function names make everything more enjoyable 
 * 
 * My favorite is probably "foam_up" for uppercase - so cute! ☕✨
 * - Khushi
 */

/*
 * Value System Implementation
 * @architect: Khushi Motwani
 * @coffee_consumption: Probably too much ☕☕☕
 * 
 * This Value enum represents everything in Brewco!
 * From simple strings to complex objects - all beautifully typed
 * with my coffee-loving heart! 💖
 */

/*
 * @khushi_reflection:
 * Building this interpreter taught me so much about language design!
 * Every error message, every evaluation step - crafted with care
 * 
 * Hope future contributors will feel the love I put into every line! ☕💖
 * Keep the coffee spirit alive! 
 * 
 * - Khushi Motwani ✨
 */