/*
 * ☕ Brewco Main Entry Point ☕
 * 
 * @author: "Khushi Motwani" 💖
 * @khushi_signature: "This is where the magic begins! ✨"
 * @creation_note: "The gateway to the Brewco universe!"
 * @khushi_feeling: "Excited to share this with the world!" 🌍
 * 
 * Welcome to Brewco - where code meets coffee culture! ☕
 * This main.rs is the beating heart of our coffee-themed programming experience.
 */

// src/main.rs

mod ast;
mod lexer;
mod parser;
mod interpreter;
mod native;
mod type_checker;
mod espresso_errors; // The Barista's Wisdom System ☕
mod coffee_io;       // The Coffee Import/Export System ☕
mod coffee_bean_roastery;    // The Coffee Bean Import & Roastery System ☕
mod coffee_package_roastery; // The Coffee Bean Package Roastery Supply Chain ☕
mod barista_language_server; // The Barista Language Server & Coffee Shop Assistant ☕
mod turbo_espresso_compiler; // The Turbo Espresso Brewing Engine ☕
mod gourmet_coffee_features; // The Gourmet Coffee Blending System ☕

use std::env;
use std::io::{self, Write};
use espresso_errors::CoffeeSpillReport;

fn print_cli_help() {
    println!("☕ Brewco CLI - Your Personal Coffee Shop Compiler ☕");
    println!("Usage:");
    println!("  brew <filename.brewco>   Brew a Brewco program");
    println!("  brew repl              Start interactive coffee shop");
    println!("  brew --help             Show this help message");
    println!("  brew --version          Show version information");
    println!("If no file is given, defaults to 'hello.brewco'.");
    println!("\n💡 Pro tip: Use .brewco extension for your coffee recipes!");
}

fn start_repl() {
    println!("☕ Welcome to the Interactive Brewco Coffee Shop! ☕");
    println!("🏪 Where every line of code is brewed to perfection!");
    println!("Type 'exit', 'quit', or 'enough_caffeine' to leave");
    println!("Type 'help' or 'barista_help' for brewing commands");
    println!("================================================");
    
    let mut coffee_interpreter = interpreter::Interpreter::new();
    let mut brewing_session = 1;
    
    loop {
        print!("☕ Coffee Shop #{} > ", brewing_session);
        io::stdout().flush().unwrap();
        
        let mut coffee_input = String::new();
        match io::stdin().read_line(&mut coffee_input) {
            Ok(_) => {
                let brewing_command = coffee_input.trim();
                
                // Special REPL commands with coffee flair
                match brewing_command {
                    "exit" | "quit" | "enough_caffeine" => {
                        println!("☕ Thanks for visiting our Coffee Shop!");
                        println!("🌟 May your code be bug-free and your coffee strong!");
                        break;
                    },
                    "help" | "barista_help" => {
                        println!("☕ Coffee Shop Commands:");
                        println!("  beans var = value     Declare a new coffee bean variable");
                        println!("  pourout expression    Display the aroma of an expression");
                        println!("  clear_counter         Clear the coffee shop counter");
                        println!("  show_pantry          Show all declared coffee beans");
                        println!("  brewing_history      Show recent brewing commands");
                        continue;
                    },
                    "clear_counter" => {
                        coffee_interpreter = interpreter::Interpreter::new();
                        println!("☕ Coffee shop counter cleared! Fresh start brewing...");
                        continue;
                    },
                    "show_pantry" => {
                        println!("☕ Current Coffee Bean Pantry:");
                        println!("   📦 Variables are stored in the coffee interpreter's private pantry!");
                        println!("   💡 Try declaring some: beans my_var pour_in 42");
                        continue;
                    },
                    "" => continue, // Empty input
                    _ => {} // Process as Brewco code
                }
                
                // Tokenize and parse the coffee input
                let coffee_tokens = lexer::lex(brewing_command);
                let brewing_result = parser::parse(&coffee_tokens);
                
                // Handle any coffee spills (errors)
                if !brewing_result.errors.is_empty() {
                    for brewing_error in &brewing_result.errors {
                        let spill_report = CoffeeSpillReport::new_brewing_disaster(
                            espresso_errors::SpillType::IncompleteRecipe,
                            1, 1, // REPL line numbers
                            brewing_error
                        );
                        println!("{}", spill_report);
                    }
                } else {
                    // Execute the brewing instructions
                    coffee_interpreter.run(&brewing_result.statements);
                }
                
                brewing_session += 1;
            },
            Err(brewing_error) => {
                let input_spill = CoffeeSpillReport::new_brewing_disaster(
                    espresso_errors::SpillType::UnderExtraction,
                    brewing_session, 1,
                    &format!("Failed to read coffee input: {}", brewing_error)
                );
                println!("{}", input_spill);
                break;
            }
        }
    }
}

fn run_file(filename: &str) {
    let code = match std::fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            println!("[ERROR] Could not read file: {}", filename);
            std::process::exit(1);
        }
    };
    let tokens = lexer::lex(&code);
    // For debugging:
    // println!("[DEBUG] Tokens: {:#?}", tokens);

    let stmts = parser::parse(&tokens);
    if !stmts.errors.is_empty() {
        println!("☕ Oops! Looks like your coffee script hit a sour note:");
        for err in stmts.errors {
            println!("  - {}", err);
        }
        println!("                       Like a latte left out in the rain, this won't brew. Fix the errors and shake it off!");
        std::process::exit(1);
    }

    // For debugging:
    // println!("[DEBUG] AST: {:#?}", stmts.statements);

    let mut type_checker = type_checker::TypeChecker::new();
    if let Err(errors) = type_checker.check(&stmts.statements) {
        println!("☕ Your coffee isn't fresh! The Freshness Checker found these issues:");
        for err in errors {
            println!("  - {}", err);
        }
        std::process::exit(1);
    }

    let mut interpreter = interpreter::Interpreter::new();
    interpreter.run(&stmts.statements);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        // No arguments - start REPL
        start_repl();
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "help" => {
            print_cli_help();
            return;
        }
        "repl" => {
            start_repl();
            return;
        }
        _ => {
            // Treat as filename
            run_file(command);
        }
    }
}

/*
 * @khushi_note: This is my beautiful Brewco interpreter!
 * Every line of code here represents my passion for both 
 * programming languages and coffee culture. ☕
 * 
 * The REPL (Read-Eval-Print Loop) creates an interactive
 * environment where developers can experiment with Brewco
 * syntax in real-time, just like brewing the perfect cup! ✨
 * 
 * I hope this brings joy to every developer who uses it! 💖
 * 
 * Thank you for using Brewco - hope it brings you as much
 * joy as writing it brought me! 😊
 * 
 * - Khushi Motwani, Creator of Brewco ☕💖
 */