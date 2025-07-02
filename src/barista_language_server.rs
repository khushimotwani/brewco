// src/barista_language_server.rs - The Barista Language Server & Coffee Shop Assistant ☕

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::espresso_errors::{CoffeeSpillReport, SpillType};
use crate::{lexer, parser, ast};

/// The Barista Language Server - provides intelligent coffee brewing assistance
pub struct BaristaLanguageServer {
    coffee_workspace: CoffeeWorkspace,
    brewing_diagnostics: Vec<CoffeeBrewingDiagnostic>,
    autocomplete_suggestions: HashMap<String, Vec<CoffeeSuggestion>>,
    hover_info_cache: HashMap<String, CoffeeHoverInfo>,
    coffee_shop_settings: CoffeeShopSettings,
}

/// Coffee Workspace - represents the current coffee project
#[derive(Clone)]
pub struct CoffeeWorkspace {
    pub roastery_root: String,
    pub open_coffee_files: HashMap<String, OpenCoffeeFile>,
    pub brewing_configuration: BrewingConfiguration,
}

/// An open coffee file in the editor
#[derive(Clone)]
pub struct OpenCoffeeFile {
    pub file_path: String,
    pub coffee_content: String,
    pub brewing_version: u64,
    pub parsed_coffee_ast: Option<Vec<ast::Statement>>,
    pub brewing_errors: Vec<CoffeeBrewingDiagnostic>,
    pub last_sip_time: std::time::SystemTime, // last modification time
}

/// Brewing configuration for the coffee project
#[derive(Clone)]
pub struct BrewingConfiguration {
    pub auto_brew_on_save: bool,
    pub strict_coffee_type_checking: bool,
    pub preferred_brewing_style: BrewingStyle,
    pub coffee_linting_rules: CoffeeLintingRules,
}

/// Coffee brewing styles
#[derive(Clone)]
pub enum BrewingStyle {
    EspressoShot,    // Fast and concentrated
    SlowBrew,        // Careful and thorough
    ColdBrew,        // Relaxed style
    CustomBlend,     // User-defined style
}

/// Coffee linting rules
#[derive(Clone)]
pub struct CoffeeLintingRules {
    pub enforce_coffee_naming: bool,     // Variables must be coffee-themed
    pub require_coffee_comments: bool,   // Comments must use 🎀
    pub max_brewing_complexity: u32,     // Max function complexity
    pub preferred_bean_style: String,    // Naming convention
}

/// Coffee Shop Settings - IDE preferences
#[derive(Clone)]
pub struct CoffeeShopSettings {
    pub show_coffee_emoji_hints: bool,
    pub auto_complete_coffee_terms: bool,
    pub highlight_coffee_syntax: bool,
    pub barista_wisdom_level: BaristaWisdomLevel,
}

/// Barista wisdom levels
#[derive(Clone)]
pub enum BaristaWisdomLevel {
    NoviceBrewer,      // Basic suggestions
    ExperiencedBarista, // Advanced suggestions  
    CoffeeMaster,      // Expert suggestions
    LegendaryRoaster,  // Guru-level suggestions
}

/// Coffee brewing diagnostic (like LSP diagnostic)
#[derive(Clone, Serialize, Deserialize)]
pub struct CoffeeBrewingDiagnostic {
    pub brewing_range: CoffeeRange,
    pub severity: BrewingSeverity,
    pub spill_message: String,
    pub barista_suggestion: Option<String>,
    pub brewing_code: Option<String>,
    pub related_information: Vec<CoffeeRelatedInfo>,
}

/// Coffee range in the file
#[derive(Clone, Serialize, Deserialize)]
pub struct CoffeeRange {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

/// Brewing severity levels
#[derive(Clone, Serialize, Deserialize)]
pub enum BrewingSeverity {
    CoffeeSpill,     // Error
    BitterTaste,     // Warning
    WeakBrew,        // Info
    PerfectBrew,     // Hint
}

/// Related coffee information
#[derive(Clone, Serialize, Deserialize)]
pub struct CoffeeRelatedInfo {
    pub related_coffee_file: String,
    pub related_range: CoffeeRange,
    pub relation_message: String,
}

/// Coffee suggestion for autocomplete
#[derive(Clone)]
pub struct CoffeeSuggestion {
    pub suggestion_text: String,
    pub brewing_kind: CoffeeSuggestionKind,
    pub detailed_info: String,
    pub coffee_snippet: Option<String>,
    pub barista_rating: f32, // 0.0 to 5.0 stars
}

/// Coffee suggestion kinds
#[derive(Clone)]
pub enum CoffeeSuggestionKind {
    CoffeeKeyword,      // Language keywords
    BeanVariable,       // Variables
    BrewingFunction,    // Functions
    CoffeeClass,        // Classes
    CoffeeInterface,    // Interfaces
    CoffeeModule,       // Modules
    CoffeeSnippet,      // Code snippets
    CoffeeEmoji,        // Coffee emojis
}

/// Coffee hover information
#[derive(Clone)]
pub struct CoffeeHoverInfo {
    pub hover_content: String,
    pub coffee_type_info: Option<String>,
    pub brewing_examples: Vec<String>,
    pub barista_tips: Vec<String>,
}

impl BaristaLanguageServer {
    pub fn new_coffee_shop_server() -> Self {
        let default_settings = CoffeeShopSettings {
            show_coffee_emoji_hints: true,
            auto_complete_coffee_terms: true,
            highlight_coffee_syntax: true,
            barista_wisdom_level: BaristaWisdomLevel::ExperiencedBarista,
        };
        
        let brewing_config = BrewingConfiguration {
            auto_brew_on_save: true,
            strict_coffee_type_checking: true,
            preferred_brewing_style: BrewingStyle::EspressoShot,
            coffee_linting_rules: CoffeeLintingRules {
                enforce_coffee_naming: true,
                require_coffee_comments: false,
                max_brewing_complexity: 10,
                preferred_bean_style: "snake_case".to_string(),
            },
        };
        
        BaristaLanguageServer {
            coffee_workspace: CoffeeWorkspace {
                roastery_root: ".".to_string(),
                open_coffee_files: HashMap::new(),
                brewing_configuration: brewing_config,
            },
            brewing_diagnostics: Vec::new(),
            autocomplete_suggestions: HashMap::new(),
            hover_info_cache: HashMap::new(),
            coffee_shop_settings: default_settings,
        }
    }
    
    /// Open a coffee file in the workspace
    pub fn open_coffee_file(&mut self, file_path: &str, content: &str) -> Result<(), CoffeeSpillReport> {
        println!("☕ Opening coffee file: {}", file_path);
        
        let coffee_file = OpenCoffeeFile {
            file_path: file_path.to_string(),
            coffee_content: content.to_string(),
            brewing_version: 1,
            parsed_coffee_ast: None,
            brewing_errors: Vec::new(),
            last_sip_time: std::time::SystemTime::now(),
        };
        
        self.coffee_workspace.open_coffee_files.insert(file_path.to_string(), coffee_file);
        self.brew_file_analysis(file_path)?;
        
        Ok(())
    }
    
    /// Analyze a coffee file and provide diagnostics
    pub fn brew_file_analysis(&mut self, file_path: &str) -> Result<(), CoffeeSpillReport> {
        // First, get the content and parse it
        let (coffee_content, parsed_ast) = {
            let coffee_file = self.coffee_workspace.open_coffee_files.get(file_path).ok_or_else(|| {
                CoffeeSpillReport::new_brewing_disaster(
                    SpillType::BeanNotFound,
                    0, 0,
                    &format!("Coffee file '{}' not found in workspace", file_path)
                )
            })?;
            
            let coffee_tokens = lexer::lex(&coffee_file.coffee_content);
            let brewing_result = parser::parse(&coffee_tokens);
            
            (coffee_file.coffee_content.clone(), brewing_result)
        };
        
        // Generate diagnostics with suggestions before getting mutable borrow
        let mut diagnostics = Vec::new();
        for (i, error) in parsed_ast.errors.iter().enumerate() {
            let suggestion = self.generate_barista_suggestion(error);
            let diagnostic = CoffeeBrewingDiagnostic {
                brewing_range: CoffeeRange {
                    start_line: i as u32,
                    start_column: 0,
                    end_line: i as u32,
                    end_column: 100,
                },
                severity: BrewingSeverity::CoffeeSpill,
                spill_message: error.clone(),
                barista_suggestion: Some(suggestion),
                brewing_code: Some(format!("COFFEE_PARSE_ERROR_{}", i)),
                related_information: Vec::new(),
            };
            diagnostics.push(diagnostic);
        }
        
        // Now get mutable access to update the file
        let coffee_file = self.coffee_workspace.open_coffee_files.get_mut(file_path).unwrap();
        
        // Clear previous errors and add new diagnostics
        coffee_file.brewing_errors.clear();
        coffee_file.brewing_errors.extend(diagnostics);
        
        // Store the AST if parsing succeeded
        if parsed_ast.errors.is_empty() {
            coffee_file.parsed_coffee_ast = Some(parsed_ast.statements);
            
            // Perform additional analysis with the content copy
            self.analyze_coffee_style_content(&coffee_content, file_path)?;
            self.analyze_coffee_complexity_content(&coffee_content, file_path)?;
        }
        
        Ok(())
    }
    
    /// Generate barista suggestions for errors
    fn generate_barista_suggestion(&self, error: &str) -> String {
        if error.contains("syntax") {
            "☕ Try checking your coffee syntax! Make sure you're using proper Brewco keywords like 'beans', 'pour_in', 'taste', etc.".to_string()
        } else if error.contains("unexpected") {
            "☕ This ingredient doesn't belong in this recipe! Check the Brewco documentation for proper syntax.".to_string()
        } else {
            "☕ Take a sip of coffee and review your code. The barista believes in you!".to_string()
        }
    }
    
    /// Analyze coffee coding style
    fn analyze_coffee_style_content(&mut self, content: &str, file_path: &str) -> Result<(), CoffeeSpillReport> {
        if !self.coffee_workspace.brewing_configuration.coffee_linting_rules.enforce_coffee_naming {
            return Ok(());
        }
        
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            // Check for non-coffee variable names
            if line.contains("beans ") && !self.is_coffee_themed_name(line) {
                let diagnostic = CoffeeBrewingDiagnostic {
                    brewing_range: CoffeeRange {
                        start_line: line_num as u32,
                        start_column: 0,
                        end_line: line_num as u32,
                        end_column: line.len() as u32,
                    },
                    severity: BrewingSeverity::WeakBrew,
                    spill_message: "Consider using coffee-themed variable names for better flavor!".to_string(),
                    barista_suggestion: Some("☕ Try names like 'coffee_strength', 'bean_count', 'brewing_time', etc.".to_string()),
                    brewing_code: Some("COFFEE_STYLE_NAMING".to_string()),
                    related_information: Vec::new(),
                };
                
                if let Some(coffee_file) = self.coffee_workspace.open_coffee_files.get_mut(file_path) {
                    coffee_file.brewing_errors.push(diagnostic);
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if a variable name is coffee-themed
    fn is_coffee_themed_name(&self, line: &str) -> bool {
        let coffee_keywords = [
            "coffee", "bean", "brew", "espresso", "latte", "mocha", "cappuccino",
            "roast", "grind", "steam", "milk", "foam", "cup", "mug", "barista",
            "cafe", "shop", "aroma", "flavor", "blend", "strength", "caffeine"
        ];
        
        coffee_keywords.iter().any(|keyword| line.to_lowercase().contains(keyword))
    }
    
    /// Analyze coffee code complexity
    fn analyze_coffee_complexity_content(&mut self, content: &str, file_path: &str) -> Result<(), CoffeeSpillReport> {
        let max_complexity = self.coffee_workspace.brewing_configuration.coffee_linting_rules.max_brewing_complexity;
        
        // Simple complexity calculation - count nested blocks
        let mut complexity = 0u32;
        let mut nesting_level = 0u32;
        
        for line in content.lines() {
            if line.trim().contains('{') {
                nesting_level += 1;
                complexity += nesting_level;
            }
            if line.trim().contains('}') {
                nesting_level = nesting_level.saturating_sub(1);
            }
        }
        
        if complexity > max_complexity {
            let diagnostic = CoffeeBrewingDiagnostic {
                brewing_range: CoffeeRange {
                    start_line: 0,
                    start_column: 0,
                    end_line: 0,
                    end_column: 0,
                },
                severity: BrewingSeverity::BitterTaste,
                spill_message: format!("This coffee recipe is too complex! Complexity: {}, Max: {}", complexity, max_complexity),
                barista_suggestion: Some("☕ Consider breaking this into smaller brewing functions for better taste!".to_string()),
                brewing_code: Some("COFFEE_COMPLEXITY_WARNING".to_string()),
                related_information: Vec::new(),
            };
            
            if let Some(coffee_file) = self.coffee_workspace.open_coffee_files.get_mut(file_path) {
                coffee_file.brewing_errors.push(diagnostic);
            }
        }
        
        Ok(())
    }
    
    /// Provide autocomplete suggestions
    pub fn get_coffee_completions(&mut self, file_path: &str, line: u32, column: u32) -> Vec<CoffeeSuggestion> {
        let mut suggestions = Vec::new();
        
        // Coffee keywords
        let coffee_keywords = vec![
            ("beans", "Declare a coffee bean variable"),
            ("brew", "Define a brewing function"),
            ("taste", "Conditional brewing (if statement)"),
            ("otherwise", "Alternative brewing (else statement)"),
            ("steep", "Brewing loop (while statement)"),
            ("pour", "Pouring loop (for statement)"),
            ("pourout", "Display coffee output"),
            ("bean", "Define a coffee bean class"),
            ("coffee_recipe", "Define a coffee recipe interface"),
            ("new", "Create a new coffee bean instance"),
            ("this", "Reference to current coffee bean"),
            ("super", "Reference to parent coffee bean"),
        ];
        
        for (keyword, description) in coffee_keywords {
            suggestions.push(CoffeeSuggestion {
                suggestion_text: keyword.to_string(),
                brewing_kind: CoffeeSuggestionKind::CoffeeKeyword,
                detailed_info: description.to_string(),
                coffee_snippet: None,
                barista_rating: 5.0,
            });
        }
        
        // Coffee operators
        let coffee_operators = vec![
            ("pour_in", "Assignment operator (=)"),
            ("add", "Addition operator (+)"),
            ("sip", "Subtraction operator (-)"),
            ("same_blend", "Equality comparison (==)"),
            ("different_blend", "Inequality comparison (!=)"),
            ("more_caffeine", "Greater than (>)"),
            ("less_caffeine", "Less than (<)"),
        ];
        
        for (operator, description) in coffee_operators {
            suggestions.push(CoffeeSuggestion {
                suggestion_text: operator.to_string(),
                brewing_kind: CoffeeSuggestionKind::CoffeeKeyword,
                detailed_info: description.to_string(),
                coffee_snippet: None,
                barista_rating: 4.5,
            });
        }
        
        // Coffee snippets
        let coffee_snippets = vec![
            ("coffee_class", "bean ${1:CoffeeClass} {\n    beans ${2:property} pour_in ${3:value}\n    \n    brew ${4:method}() {\n        ${5:// brewing logic}\n    }\n}"),
            ("coffee_function", "brew ${1:function_name}(${2:parameters}) {\n    ${3:// brewing logic}\n    return ${4:result}\n}"),
            ("coffee_loop", "steep ${1:condition} {\n    ${2:// brewing process}\n}"),
            ("coffee_if", "taste ${1:condition} {\n    ${2:// perfect brew}\n} otherwise {\n    ${3:// needs more work}\n}"),
        ];
        
        for (name, snippet) in coffee_snippets {
            suggestions.push(CoffeeSuggestion {
                suggestion_text: name.to_string(),
                brewing_kind: CoffeeSuggestionKind::CoffeeSnippet,
                detailed_info: format!("Coffee code snippet: {}", name),
                coffee_snippet: Some(snippet.to_string()),
                barista_rating: 4.8,
            });
        }
        
        suggestions
    }
    
    /// Get hover information for coffee elements
    pub fn get_coffee_hover_info(&mut self, file_path: &str, line: u32, column: u32) -> Option<CoffeeHoverInfo> {
        // Mock implementation - in real implementation, this would analyze the AST
        Some(CoffeeHoverInfo {
            hover_content: "☕ **Coffee Bean Variable**\n\nA delicious coffee bean that holds a value.".to_string(),
            coffee_type_info: Some("Type: CoffeeBean<String>".to_string()),
            brewing_examples: vec![
                "beans my_coffee pour_in \"Espresso\"".to_string(),
                "beans coffee_strength pour_in 9".to_string(),
            ],
            barista_tips: vec![
                "☕ Use descriptive coffee-themed names for better code flavor!".to_string(),
                "☕ Consider the strength of your coffee when choosing values.".to_string(),
            ],
        })
    }
    
    /// Get all diagnostics for a file
    pub fn get_coffee_diagnostics(&self, file_path: &str) -> Vec<CoffeeBrewingDiagnostic> {
        if let Some(coffee_file) = self.coffee_workspace.open_coffee_files.get(file_path) {
            coffee_file.brewing_errors.clone()
        } else {
            Vec::new()
        }
    }
    
    /// Update coffee shop settings
    pub fn update_coffee_shop_settings(&mut self, settings: CoffeeShopSettings) {
        self.coffee_shop_settings = settings;
        println!("☕ Coffee shop settings updated! Your barista is now more helpful!");
    }
} 