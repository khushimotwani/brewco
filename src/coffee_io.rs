/*
 * ☕ Coffee File Brewery - I/O System ☕
 * 
 * @file_wizard: Khushi Motwani 
 * @naming_inspiration: "What if file operations were delicious?" ☕
 * @proudest_moment: "sip_file() and pour_to_file() are poetry!" ✨
 * 
 * Welcome to the Coffee File Brewery - where file operations 
 * are as smooth as your morning espresso! 
 * 
 * I designed every function name to make file I/O feel 
 * warm and welcoming instead of scary and technical! 💖
 * 
 * @khushi_favorite: "scan_pantry() for directory listing is *chef's kiss*"
 * - Khushi 💖
 */

// src/coffee_io.rs - The Coffee Import/Export System ☕

use std::fs;
use std::io::Write;
use std::path::Path;
use crate::espresso_errors::{CoffeeSpillReport, SpillType};

// Coffee-themed file operations
pub struct CoffeeFileBrewery;

impl CoffeeFileBrewery {
    /// Read the entire contents of a coffee recipe file
    pub fn sip_entire_recipe(recipe_path: &str) -> Result<String, CoffeeSpillReport> {
        match fs::read_to_string(recipe_path) {
            Ok(contents) => Ok(contents),
            Err(e) => {
                let spill = CoffeeSpillReport::new_brewing_disaster(
                    SpillType::BeanNotFound,
                    0, 0,
                    &format!("Failed to sip from recipe '{}': {}", recipe_path, e)
                );
                Err(spill)
            }
        }
    }
    
    /// Write coffee recipe to a file
    pub fn pour_recipe_to_file(recipe_path: &str, coffee_contents: &str) -> Result<(), CoffeeSpillReport> {
        match fs::write(recipe_path, coffee_contents) {
            Ok(_) => Ok(()),
            Err(e) => {
                let spill = CoffeeSpillReport::new_brewing_disaster(
                    SpillType::OverExtraction,
                    0, 0,
                    &format!("Failed to pour recipe to '{}': {}", recipe_path, e)
                );
                Err(spill)
            }
        }
    }
    
    /// Append to a coffee log file
    pub fn add_to_coffee_log(log_path: &str, coffee_entry: &str) -> Result<(), CoffeeSpillReport> {
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path) 
        {
            Ok(mut file) => {
                match writeln!(file, "{}", coffee_entry) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        let spill = CoffeeSpillReport::new_brewing_disaster(
                            SpillType::OverExtraction,
                            0, 0,
                            &format!("Failed to add entry to coffee log '{}': {}", log_path, e)
                        );
                        Err(spill)
                    }
                }
            }
            Err(e) => {
                let spill = CoffeeSpillReport::new_brewing_disaster(
                    SpillType::BeanNotFound,
                    0, 0,
                    &format!("Failed to open coffee log '{}': {}", log_path, e)
                );
                Err(spill)
            }
        }
    }
    
    /// Check if a coffee recipe exists
    pub fn recipe_exists(recipe_path: &str) -> bool {
        Path::new(recipe_path).exists()
    }
    
    /// List all coffee recipes in a directory
    pub fn scan_coffee_pantry(pantry_path: &str) -> Result<Vec<String>, CoffeeSpillReport> {
        match fs::read_dir(pantry_path) {
            Ok(entries) => {
                let mut coffee_recipes = Vec::new();
                
                for entry in entries {
                    match entry {
                        Ok(dir_entry) => {
                            let path = dir_entry.path();
                            if let Some(extension) = path.extension() {
                                if extension == "brewco" || extension == "coffee" {
                                    if let Some(filename) = path.file_name() {
                                        if let Some(name_str) = filename.to_str() {
                                            coffee_recipes.push(name_str.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            let spill = CoffeeSpillReport::new_brewing_disaster(
                                SpillType::UnderExtraction,
                                0, 0,
                                &format!("Error scanning coffee pantry entry: {}", e)
                            );
                            return Err(spill);
                        }
                    }
                }
                
                Ok(coffee_recipes)
            }
            Err(e) => {
                let spill = CoffeeSpillReport::new_brewing_disaster(
                    SpillType::BeanNotFound,
                    0, 0,
                    &format!("Failed to scan coffee pantry '{}': {}", pantry_path, e)
                );
                Err(spill)
            }
        }
    }
    
    /// Create a new coffee recipe file with template
    pub fn brew_new_recipe(recipe_name: &str, recipe_type: &str) -> Result<(), CoffeeSpillReport> {
        let template = match recipe_type {
            "basic" => generate_basic_coffee_template(),
            "bean" => generate_bean_coffee_template(),
            "advanced" => generate_advanced_coffee_template(),
            _ => generate_basic_coffee_template(),
        };
        
        let filename = if recipe_name.ends_with(".brewco") {
            recipe_name.to_string()
        } else {
            format!("{}.brewco", recipe_name)
        };
        
        Self::pour_recipe_to_file(&filename, &template)
    }
}

// Coffee recipe templates
fn generate_basic_coffee_template() -> String {
    r#"🎀 Basic Brewco Recipe ☕
🎀 Generated by the Coffee File Brewery

beans greeting pour_in "Hello, Coffee World!"
beans coffee_strength pour_in 9
beans is_delicious pour_in true

pourout greeting
pourout "Coffee strength:", coffee_strength
pourout "Is it delicious?", is_delicious

🎀 Try some basic math
beans total_cups pour_in coffee_strength add 3
pourout "Total cups needed:", total_cups

🎀 Happy brewing! ☕✨
"#.to_string()
}

fn generate_bean_coffee_template() -> String {
    r#"🎀 Coffee Bean Class Template ☕
🎀 Generated by the Coffee File Brewery

bean CoffeeShop {
    beans shop_name pour_in "The Perfect Brew"
    beans daily_special pour_in "Vanilla Latte"
    
    brew welcome_customer() {
        pourout "Welcome to", this.shop_name
        pourout "Today's special is:", this.daily_special
    }
    
    brew make_coffee(coffee_type: String, size: String) {
        pourout "Brewing a", size, coffee_type
        pourout "Your coffee is ready! ☕"
    }
}

🎀 Create and use the coffee shop
beans my_shop pour_in new CoffeeShop()
my_shop.welcome_customer()
my_shop.make_coffee("Espresso", "Large")

🎀 Happy brewing! ☕✨
"#.to_string()
}

fn generate_advanced_coffee_template() -> String {
    r#"🎀 Advanced Brewco Recipe ☕
🎀 Generated by the Coffee File Brewery

🎀 Coffee Recipe Interface
coffee_recipe BrewingMethod {
    brew_coffee(beans: Number, water: Number) -> String
    get_brew_time() -> Number
}

🎀 Espresso Bean Class
bean EspressoMachine blend BrewingMethod {
    beans machine_name pour_in "Professional Espresso"
    beans pressure pour_in 9
    
    brew brew_coffee(beans: Number, water: Number): String {
        beans ratio pour_in beans divide water
        taste ratio more_caffeine 0.5 {
            return "Perfect espresso! ☕"
        } otherwise {
            return "Needs more coffee beans! 🫘"
        }
    }
    
    brew get_brew_time(): Number {
        return 25  🎀 seconds
    }
}

🎀 Use the advanced features
beans my_machine pour_in new EspressoMachine()
beans result pour_in my_machine.brew_coffee(18, 36)
pourout result

🎀 Happy advanced brewing! ☕✨
"#.to_string()
}

// Native functions for Brewco file operations
pub fn native_sip_file(args: Vec<crate::interpreter::Value>) -> Result<crate::interpreter::Value, crate::interpreter::ControlFlow> {
    if args.len() != 1 {
        return Err(crate::interpreter::ControlFlow::RuntimeError(
            "sip_file() expects 1 argument (file path)".to_string()
        ));
    }
    
    match &args[0] {
        crate::interpreter::Value::String(path) => {
            match CoffeeFileBrewery::sip_entire_recipe(path) {
                Ok(contents) => Ok(crate::interpreter::Value::String(contents)),
                Err(spill) => Err(crate::interpreter::ControlFlow::RuntimeError(
                    format!("File reading spill: {}", spill.bitter_message)
                ))
            }
        }
        _ => Err(crate::interpreter::ControlFlow::RuntimeError(
            "sip_file() expects a string file path".to_string()
        ))
    }
}

pub fn native_pour_to_file(args: Vec<crate::interpreter::Value>) -> Result<crate::interpreter::Value, crate::interpreter::ControlFlow> {
    if args.len() != 2 {
        return Err(crate::interpreter::ControlFlow::RuntimeError(
            "pour_to_file() expects 2 arguments (file path, content)".to_string()
        ));
    }
    
    match (&args[0], &args[1]) {
        (crate::interpreter::Value::String(path), crate::interpreter::Value::String(content)) => {
            match CoffeeFileBrewery::pour_recipe_to_file(path, content) {
                Ok(_) => Ok(crate::interpreter::Value::Boolean(true)),
                Err(spill) => Err(crate::interpreter::ControlFlow::RuntimeError(
                    format!("File writing spill: {}", spill.bitter_message)
                ))
            }
        }
        _ => Err(crate::interpreter::ControlFlow::RuntimeError(
            "pour_to_file() expects string arguments".to_string()
        ))
    }
}

pub fn native_recipe_exists(args: Vec<crate::interpreter::Value>) -> Result<crate::interpreter::Value, crate::interpreter::ControlFlow> {
    if args.len() != 1 {
        return Err(crate::interpreter::ControlFlow::RuntimeError(
            "recipe_exists() expects 1 argument (file path)".to_string()
        ));
    }
    
    match &args[0] {
        crate::interpreter::Value::String(path) => {
            Ok(crate::interpreter::Value::Boolean(CoffeeFileBrewery::recipe_exists(path)))
        }
        _ => Err(crate::interpreter::ControlFlow::RuntimeError(
            "recipe_exists() expects a string file path".to_string()
        ))
    }
}

pub fn native_scan_pantry(args: Vec<crate::interpreter::Value>) -> Result<crate::interpreter::Value, crate::interpreter::ControlFlow> {
    if args.len() != 1 {
        return Err(crate::interpreter::ControlFlow::RuntimeError(
            "scan_pantry() expects 1 argument (directory path)".to_string()
        ));
    }
    
    match &args[0] {
        crate::interpreter::Value::String(path) => {
            match CoffeeFileBrewery::scan_coffee_pantry(path) {
                Ok(recipes) => {
                    let values: Vec<crate::interpreter::Value> = recipes
                        .into_iter()
                        .map(|r| crate::interpreter::Value::String(r))
                        .collect();
                    Ok(crate::interpreter::Value::Array(values))
                }
                Err(spill) => Err(crate::interpreter::ControlFlow::RuntimeError(
                    format!("Pantry scanning spill: {}", spill.bitter_message)
                ))
            }
        }
        _ => Err(crate::interpreter::ControlFlow::RuntimeError(
            "scan_pantry() expects a string directory path".to_string()
        ))
    }
} 