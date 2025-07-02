/*
 * ☕ Coffee Spill Reports - Enhanced Error System ☕
 * 
 * @error_whisperer: Khushi Motwani 
 * @philosophy: "Errors should be helpful, not scary" ☕💖
 * @innovation: "Made error messages that actually help developers" ✨
 * 
 * This is my favorite kind of engineering - turning frustrating 
 * compiler errors into gentle, helpful guidance! 
 * 
 * Every error message here was crafted with empathy and 
 * sprinkled with coffee-themed personality! 
 * 
 * @khushi_belief: "Good error messages are love letters to future debuggers"
 * - Khushi 💖
 */

// src/espresso_errors.rs - The Barista's Wisdom for Better Error Messages ☕

use std::fmt;

#[derive(Debug, Clone)]
pub struct CoffeeSpillReport {
    pub spill_type: SpillType,
    pub coffee_line: usize,
    pub brewing_column: usize,
    pub bitter_message: String,
    pub barista_wisdom: Vec<String>,
    pub coffee_context: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpillType {
    // Parser spills - when syntax goes wrong
    UnexpectedIngredient,      // Unexpected token
    MissingBean,              // Missing identifier
    WrongBrewingMethod,       // Wrong operator
    IncompleteRecipe,         // Incomplete statement
    
    // Runtime spills - when execution goes wrong  
    BeanNotFound,             // Variable not found
    WrongCupType,             // Type mismatch
    OverExtraction,           // Stack overflow
    UnderExtraction,          // Missing return value
    
    // Semantic spills - when meaning is unclear
    ConflictingFlavors,       // Conflicting types
    MissingAroma,             // Missing method
    TooManyShots,             // Too many arguments
    NotEnoughCaffeine,        // Missing required feature
}

impl CoffeeSpillReport {
    pub fn new_brewing_disaster(
        spill_type: SpillType, 
        line: usize, 
        column: usize, 
        message: &str
    ) -> Self {
        let barista_wisdom = generate_barista_wisdom(&spill_type, message);
        
        CoffeeSpillReport {
            spill_type,
            coffee_line: line,
            brewing_column: column,
            bitter_message: message.to_string(),
            barista_wisdom,
            coffee_context: None,
        }
    }
    
    pub fn add_coffee_context(&mut self, context: &str) {
        self.coffee_context = Some(context.to_string());
    }
    
    pub fn brew_detailed_report(&self) -> String {
        let mut report = String::new();
        
        // Header with coffee emoji and error type
        report.push_str(&format!(
            "☕ COFFEE SPILL ALERT! {} at line {}, column {}\n", 
            spill_description(&self.spill_type),
            self.coffee_line,
            self.brewing_column
        ));
        
        // Main error message
        report.push_str(&format!("🚨 What happened: {}\n", self.bitter_message));
        
        // Context if available
        if let Some(ref context) = self.coffee_context {
            report.push_str(&format!("📍 In this brewing context:\n   {}\n", context));
        }
        
        // Barista wisdom (suggestions)
        if !self.barista_wisdom.is_empty() {
            report.push_str("\n☕ The Barista's Wisdom:\n");
            for (i, wisdom) in self.barista_wisdom.iter().enumerate() {
                report.push_str(&format!("   {}. {}\n", i + 1, wisdom));
            }
        }
        
        // Footer with encouragement
        report.push_str("\n💪 Don't let this spill ruin your brew! Every barista makes mistakes. ");
        report.push_str("Clean it up and keep brewing amazing code! ☕✨\n");
        
        report
    }
}

fn spill_description(spill_type: &SpillType) -> &'static str {
    match spill_type {
        SpillType::UnexpectedIngredient => "Unexpected Ingredient Found",
        SpillType::MissingBean => "Missing Coffee Bean",
        SpillType::WrongBrewingMethod => "Wrong Brewing Method",
        SpillType::IncompleteRecipe => "Incomplete Recipe",
        SpillType::BeanNotFound => "Coffee Bean Not Found",
        SpillType::WrongCupType => "Wrong Cup Type",
        SpillType::OverExtraction => "Over-Extraction Error",
        SpillType::UnderExtraction => "Under-Extraction Error",
        SpillType::ConflictingFlavors => "Conflicting Flavors",
        SpillType::MissingAroma => "Missing Aroma",
        SpillType::TooManyShots => "Too Many Espresso Shots",
        SpillType::NotEnoughCaffeine => "Not Enough Caffeine",
    }
}

fn generate_barista_wisdom(spill_type: &SpillType, message: &str) -> Vec<String> {
    let mut wisdom = Vec::new();
    
    match spill_type {
        SpillType::UnexpectedIngredient => {
            wisdom.push("Check if you're using the right coffee syntax".to_string());
            wisdom.push("Maybe you meant to use a different operator like 'add' instead of '+'?".to_string());
            wisdom.push("Look for missing semicolons or brackets that might be causing confusion".to_string());
        }
        
        SpillType::MissingBean => {
            wisdom.push("Don't forget to declare your beans with 'beans variable_name = value'".to_string());
            wisdom.push("Check for typos in your variable names - Brewco is case-sensitive".to_string());
            wisdom.push("Make sure the variable is in scope where you're trying to use it".to_string());
        }
        
        SpillType::WrongBrewingMethod => {
            wisdom.push("Brewco uses coffee-themed operators: 'add', 'sip', 'more_caffeine', etc.".to_string());
            wisdom.push("Try 'same_blend' for equality comparison instead of '=='".to_string());
            wisdom.push("Use 'pour_in' for assignment instead of '='".to_string());
        }
        
        SpillType::IncompleteRecipe => {
            wisdom.push("Every recipe needs all its ingredients - check for missing parts".to_string());
            wisdom.push("Make sure your blocks are properly closed with '}'".to_string());
            wisdom.push("Check that function calls have matching parentheses".to_string());
        }
        
        SpillType::BeanNotFound => {
            wisdom.push("This coffee bean hasn't been planted yet - declare it first".to_string());
            wisdom.push("Check the spelling of your variable name".to_string());
            wisdom.push("Make sure the variable is declared in the current scope".to_string());
        }
        
        SpillType::WrongCupType => {
            wisdom.push("You're trying to pour coffee into the wrong cup type".to_string());
            wisdom.push("Check if you're mixing numbers with strings incorrectly".to_string());
            wisdom.push("Use type conversion functions if needed".to_string());
        }
        
        SpillType::TooManyShots => {
            wisdom.push("This function is getting more espresso shots than it can handle".to_string());
            wisdom.push("Check the function signature to see how many parameters it expects".to_string());
            wisdom.push("Remove extra arguments or add parameters to the function definition".to_string());
        }
        
        _ => {
            wisdom.push("Take a sip of coffee and review the code carefully".to_string());
            wisdom.push("Check the Brewco documentation for syntax examples".to_string());
            wisdom.push("Try breaking the problem into smaller brewing steps".to_string());
        }
    }
    
    // Add context-specific wisdom based on the message content
    if message.contains("parse") || message.contains("syntax") {
        wisdom.push("Use the Brewco REPL to test small snippets: 'cargo run repl'".to_string());
    }
    
    if message.contains("type") {
        wisdom.push("Remember: Brewco is strongly typed like a perfectly calibrated espresso machine".to_string());
    }
    
    wisdom
}

// Helper functions for creating specific error types
pub fn unexpected_token_spill(line: usize, column: usize, found: &str, expected: &str) -> CoffeeSpillReport {
    let message = format!(
        "Found '{}' when brewing, but expected '{}'. It's like adding salt instead of sugar!",
        found, expected
    );
    CoffeeSpillReport::new_brewing_disaster(SpillType::UnexpectedIngredient, line, column, &message)
}

pub fn missing_bean_spill(line: usize, column: usize, bean_name: &str) -> CoffeeSpillReport {
    let message = format!(
        "The coffee bean '{}' is missing from your pantry. Did you forget to declare it?",
        bean_name
    );
    CoffeeSpillReport::new_brewing_disaster(SpillType::BeanNotFound, line, column, &message)
}

pub fn type_mismatch_spill(line: usize, column: usize, expected: &str, found: &str) -> CoffeeSpillReport {
    let message = format!(
        "Type mismatch: expected a {} but got a {}. It's like ordering decaf when you wanted espresso!",
        expected, found
    );
    CoffeeSpillReport::new_brewing_disaster(SpillType::WrongCupType, line, column, &message)
}

pub fn incomplete_recipe_spill(line: usize, column: usize, what_missing: &str) -> CoffeeSpillReport {
    let message = format!(
        "Your coffee recipe is incomplete - missing {}. Every good brew needs all its ingredients!",
        what_missing
    );
    CoffeeSpillReport::new_brewing_disaster(SpillType::IncompleteRecipe, line, column, &message)
}

impl fmt::Display for CoffeeSpillReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.brew_detailed_report())
    }
} 