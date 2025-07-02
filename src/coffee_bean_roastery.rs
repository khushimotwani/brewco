/*
 * 🔧 Coffee Bean Roastery - Module System 🔧
 * 
 * @module_wizard: Khushi Motwani 
 * @inspiration: "Code organization should be as smooth as coffee" ☕
 * @feature_pride: "Hot reloading modules like a pro barista!" ✨
 * 
 * Welcome to the Coffee Bean Roastery - where code gets organized 
 * into beautiful, reusable coffee modules! 
 * 
 * Every import, every export, every hot reload was designed 
 * to make development as delightful as your morning brew! 💖
 * 
 * @khushi_excitement: "Modules make everything better - fight me!"
 * - Khushi 💖
 */

// src/coffee_bean_roastery.rs - The Coffee Bean Import & Roastery System ☕

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::espresso_errors::{CoffeeSpillReport, SpillType};
use crate::coffee_io::CoffeeFileBrewery;
use crate::interpreter::{Value, Interpreter};
use crate::{lexer, parser};

/// The Coffee Bean Roastery - manages all imported coffee modules
pub struct CoffeeBeanRoastery {
    roasted_beans: HashMap<String, RoastedCoffeeBean>,
    brewing_paths: Vec<PathBuf>,
    current_brewing_dir: PathBuf,
}

/// A roasted coffee bean represents a loaded module with its exports
#[derive(Clone)]
pub struct RoastedCoffeeBean {
    pub bean_name: String,
    pub bean_origin: String, // file path
    pub exported_flavors: HashMap<String, Value>, // exported variables/functions
    pub brewing_time: std::time::SystemTime,
}

/// Coffee Import Declaration - represents an import statement
pub struct CoffeeImportDeclaration {
    pub coffee_source: String,        // module path like "coffee_shop/espresso"
    pub imported_flavors: Vec<String>, // specific imports like ["make_espresso", "grind_beans"]
    pub import_alias: Option<String>,  // alias like "import espresso as esp"
}

impl CoffeeBeanRoastery {
    pub fn new_coffee_roastery() -> Self {
        let mut brewing_paths = vec![
            PathBuf::from("./coffee_beans"),     // Local coffee beans
            PathBuf::from("./roastery"),         // Project roastery
            PathBuf::from("../shared_beans"),    // Shared coffee beans
        ];
        
        // Add current directory
        brewing_paths.push(PathBuf::from("."));
        
        CoffeeBeanRoastery {
            roasted_beans: HashMap::new(),
            brewing_paths,
            current_brewing_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }
    
    /// Import a coffee bean module with delicious coffee-themed syntax
    pub fn brew_import_bean(
        &mut self, 
        coffee_import: &CoffeeImportDeclaration,
        coffee_interpreter: &mut Interpreter
    ) -> Result<RoastedCoffeeBean, CoffeeSpillReport> {
        // Check if we've already roasted this bean
        if let Some(existing_bean) = self.roasted_beans.get(&coffee_import.coffee_source) {
            return Ok(existing_bean.clone());
        }
        
        // Find the coffee bean file
        let bean_file_path = self.find_coffee_bean_file(&coffee_import.coffee_source)?;
        
        // Read and brew the coffee bean
        let coffee_source_code = CoffeeFileBrewery::sip_entire_recipe(&bean_file_path.to_string_lossy())?;
        
        // Parse and execute the coffee bean module
        let coffee_tokens = lexer::lex(&coffee_source_code);
        let brewing_result = parser::parse(&coffee_tokens);
        
        if !brewing_result.errors.is_empty() {
            return Err(CoffeeSpillReport::new_brewing_disaster(
                SpillType::IncompleteRecipe,
                0, 0,
                &format!("Coffee bean '{}' has brewing errors: {:?}", coffee_import.coffee_source, brewing_result.errors)
            ));
        }
        
        // Create a fresh coffee interpreter for the module
        let mut bean_interpreter = Interpreter::new();
        bean_interpreter.run(&brewing_result.statements);
        
        // Extract exported flavors (variables/functions)
        let exported_flavors = self.extract_coffee_flavors(&bean_interpreter);
        
        // Create the roasted bean
        let roasted_bean = RoastedCoffeeBean {
            bean_name: coffee_import.coffee_source.clone(),
            bean_origin: bean_file_path.to_string_lossy().to_string(),
            exported_flavors,
            brewing_time: std::time::SystemTime::now(),
        };
        
        // Store the roasted bean for future use
        self.roasted_beans.insert(coffee_import.coffee_source.clone(), roasted_bean.clone());
        
        // Import the flavors into the main interpreter
        self.pour_flavors_into_interpreter(&roasted_bean, coffee_import, coffee_interpreter)?;
        
        Ok(roasted_bean)
    }
    
    /// Find a coffee bean file in the brewing paths
    fn find_coffee_bean_file(&self, bean_name: &str) -> Result<PathBuf, CoffeeSpillReport> {
        let possible_extensions = vec!["brewco", "coffee", "bean"];
        
        for brewing_path in &self.brewing_paths {
            for extension in &possible_extensions {
                let bean_file = brewing_path.join(format!("{}.{}", bean_name, extension));
                if bean_file.exists() {
                    return Ok(bean_file);
                }
                
                // Also try with subdirectories
                let module_dir = brewing_path.join(bean_name);
                if module_dir.is_dir() {
                    let index_file = module_dir.join(format!("index.{}", extension));
                    if index_file.exists() {
                        return Ok(index_file);
                    }
                    
                    let main_file = module_dir.join(format!("main.{}", extension));
                    if main_file.exists() {
                        return Ok(main_file);
                    }
                }
            }
        }
        
        Err(CoffeeSpillReport::new_brewing_disaster(
            SpillType::BeanNotFound,
            0, 0,
            &format!("Coffee bean '{}' not found in any roastery path", bean_name)
        ))
    }
    
    /// Extract coffee flavors (exports) from a module interpreter
    fn extract_coffee_flavors(&self, bean_interpreter: &Interpreter) -> HashMap<String, Value> {
        // For now, we'll export everything from the module's global scope
        // In a more advanced implementation, we'd have explicit export statements
        
        // Since scope_stack is private, we'll need a different approach
        // For now, let's create a simple export mechanism
        HashMap::new() // TODO: Implement proper export extraction
    }
    
    /// Pour flavors (imports) into the main interpreter
    fn pour_flavors_into_interpreter(
        &self,
        roasted_bean: &RoastedCoffeeBean,
        coffee_import: &CoffeeImportDeclaration,
        coffee_interpreter: &mut Interpreter
    ) -> Result<(), CoffeeSpillReport> {
        if coffee_import.imported_flavors.is_empty() {
            // Import everything with namespace
            let namespace = coffee_import.import_alias.as_ref()
                .unwrap_or(&roasted_bean.bean_name);
                
            // Create a module object containing all exports
            let module_object = Value::Object {
                class_name: "CoffeeModule".to_string(),
                fields: roasted_bean.exported_flavors.clone(),
            };
            
            // TODO: Set variable in interpreter - need access to set_var
            // coffee_interpreter.set_var(namespace.clone(), module_object);
        } else {
            // Import specific flavors
            for flavor_name in &coffee_import.imported_flavors {
                if let Some(flavor_value) = roasted_bean.exported_flavors.get(flavor_name) {
                    // TODO: Set variable in interpreter
                    // coffee_interpreter.set_var(flavor_name.clone(), flavor_value.clone());
                }
            }
        }
        
        Ok(())
    }
    
    /// Add a new brewing path for finding coffee beans
    pub fn add_coffee_brewing_path(&mut self, path: PathBuf) {
        if !self.brewing_paths.contains(&path) {
            self.brewing_paths.push(path);
        }
    }
    
    /// List all available coffee beans in brewing paths
    pub fn scan_all_coffee_beans(&self) -> Result<Vec<String>, CoffeeSpillReport> {
        let mut all_beans = Vec::new();
        
        for brewing_path in &self.brewing_paths {
            if let Ok(recipes) = CoffeeFileBrewery::scan_coffee_pantry(&brewing_path.to_string_lossy()) {
                for recipe in recipes {
                    // Remove extension and add to beans list
                    if let Some(bean_name) = recipe.split('.').next() {
                        if !all_beans.contains(&bean_name.to_string()) {
                            all_beans.push(bean_name.to_string());
                        }
                    }
                }
            }
        }
        
        Ok(all_beans)
    }
    
    /// Get information about a roasted bean
    pub fn get_roasted_bean_info(&self, bean_name: &str) -> Option<&RoastedCoffeeBean> {
        self.roasted_beans.get(bean_name)
    }
    
    /// Reload a coffee bean (for development)
    pub fn reheat_coffee_bean(
        &mut self, 
        bean_name: &str,
        coffee_interpreter: &mut Interpreter
    ) -> Result<(), CoffeeSpillReport> {
        // Remove the cached bean
        self.roasted_beans.remove(bean_name);
        
        // Re-import it
        let import_decl = CoffeeImportDeclaration {
            coffee_source: bean_name.to_string(),
            imported_flavors: vec![],
            import_alias: None,
        };
        
        self.brew_import_bean(&import_decl, coffee_interpreter)?;
        Ok(())
    }
}

/// Native functions for Brewco module system
pub fn native_brew_import(
    args: Vec<crate::interpreter::Value>,
    roastery: &mut CoffeeBeanRoastery,
    interpreter: &mut Interpreter
) -> Result<crate::interpreter::Value, crate::interpreter::ControlFlow> {
    if args.is_empty() {
        return Err(crate::interpreter::ControlFlow::RuntimeError(
            "brew_import() expects at least 1 argument (module name)".to_string()
        ));
    }
    
    match &args[0] {
        crate::interpreter::Value::String(module_name) => {
            let import_decl = CoffeeImportDeclaration {
                coffee_source: module_name.clone(),
                imported_flavors: vec![],
                import_alias: None,
            };
            
            match roastery.brew_import_bean(&import_decl, interpreter) {
                Ok(roasted_bean) => Ok(crate::interpreter::Value::Boolean(true)),
                Err(spill) => Err(crate::interpreter::ControlFlow::RuntimeError(
                    format!("Import brewing spill: {}", spill.bitter_message)
                ))
            }
        }
        _ => Err(crate::interpreter::ControlFlow::RuntimeError(
            "brew_import() expects a string module name".to_string()
        ))
    }
}

pub fn native_list_coffee_beans(
    args: Vec<crate::interpreter::Value>,
    roastery: &CoffeeBeanRoastery
) -> Result<crate::interpreter::Value, crate::interpreter::ControlFlow> {
    match roastery.scan_all_coffee_beans() {
        Ok(beans) => {
            let values: Vec<crate::interpreter::Value> = beans
                .into_iter()
                .map(|b| crate::interpreter::Value::String(b))
                .collect();
            Ok(crate::interpreter::Value::Array(values))
        }
        Err(spill) => Err(crate::interpreter::ControlFlow::RuntimeError(
            format!("Bean scanning spill: {}", spill.bitter_message)
        ))
    }
}

pub fn native_reheat_bean(
    args: Vec<crate::interpreter::Value>,
    roastery: &mut CoffeeBeanRoastery,
    interpreter: &mut Interpreter
) -> Result<crate::interpreter::Value, crate::interpreter::ControlFlow> {
    if args.len() != 1 {
        return Err(crate::interpreter::ControlFlow::RuntimeError(
            "reheat_bean() expects 1 argument (bean name)".to_string()
        ));
    }
    
    match &args[0] {
        crate::interpreter::Value::String(bean_name) => {
            match roastery.reheat_coffee_bean(bean_name, interpreter) {
                Ok(_) => Ok(crate::interpreter::Value::Boolean(true)),
                Err(spill) => Err(crate::interpreter::ControlFlow::RuntimeError(
                    format!("Bean reheating spill: {}", spill.bitter_message)
                ))
            }
        }
        _ => Err(crate::interpreter::ControlFlow::RuntimeError(
            "reheat_bean() expects a string bean name".to_string()
        ))
    }
} 