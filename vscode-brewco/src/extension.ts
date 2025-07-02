/*
 * ☕ Brewco VS Code Extension ☕
 * 
 * @author: "Khushi Motwani" 💖
 * @purpose: "Making Brewco development as smooth as espresso" ☕
 * @signature: "Made with love by Khushi" ✨
 * 
 * This extension brings the full Brewco experience to VS Code!
 * Every feature crafted with coffee-loving care! ☕💖
 */

import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    console.log('☕ Brewco extension is brewing up! ☕');
    
    // Register all Brewco commands
    const commands = [
        registerRunInterpreter(),
        registerRunREPL(),
        registerCreateNewFile()
    ];
    
    commands.forEach(command => context.subscriptions.push(command));
    
    // Register providers
    registerCompletionProvider(context);
    registerHoverProvider(context);
    
    console.log('✨ Brewco extension is ready to brew amazing code! ✨');
}

// Run Brewco file
function registerRunInterpreter() {
    return vscode.commands.registerCommand('brewco.runInterpreter', () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('☕ No coffee recipe open! Please open a .brewco file first.');
            return;
        }
        
        const filePath = editor.document.fileName;
        if (!filePath.endsWith('.brewco')) {
            vscode.window.showWarningMessage('☕ This doesn\'t look like a coffee recipe! (.brewco file required)');
            return;
        }
        
        const terminal = vscode.window.createTerminal('☕ Brewco Brewing Terminal');
        terminal.show();
        terminal.sendText(`cargo run "${filePath}"`);
        
        vscode.window.showInformationMessage(`☕ Brewing your delicious "${filePath.split('/').pop()}" recipe! ☕`);
    });
}

// Open Brewco REPL
function registerRunREPL() {
    return vscode.commands.registerCommand('brewco.runREPL', () => {
        const terminal = vscode.window.createTerminal('☕ Brewco Coffee Shop (REPL)');
        terminal.show();
        terminal.sendText('cargo run repl');
        
        vscode.window.showInformationMessage('☕ Welcome to the Brewco Coffee Shop! Start brewing! ☕');
    });
}

// Create new Brewco file
function registerCreateNewFile() {
    return vscode.commands.registerCommand('brewco.createNewFile', async () => {
        const fileName = await vscode.window.showInputBox({
            prompt: '☕ What should we name your new coffee recipe?',
            placeHolder: 'my_amazing_recipe',
            validateInput: (value: string) => {
                if (!value || value.trim() === '') {
                    return 'Please enter a filename for your coffee recipe ☕';
                }
                if (!/^[a-zA-Z0-9_-]+$/.test(value)) {
                    return 'Filename should only contain letters, numbers, underscores, and hyphens ☕';
                }
                return null;
            }
        });
        
        if (!fileName) return;
        
        const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
        if (!workspaceFolder) {
            vscode.window.showErrorMessage('☕ No workspace open! Please open a folder first.');
            return;
        }
        
        const newFile = vscode.Uri.joinPath(workspaceFolder.uri, `${fileName}.brewco`);
        
        try {
            const encoder = new TextEncoder();
            await vscode.workspace.fs.writeFile(newFile, encoder.encode(generateBrewcoTemplate(fileName)));
            const doc = await vscode.workspace.openTextDocument(newFile);
            await vscode.window.showTextDocument(doc);
            
            vscode.window.showInformationMessage(`☕ New coffee recipe "${fileName}.brewco" is ready for brewing!`);
        } catch (error) {
            vscode.window.showErrorMessage(`☕ Oops! Couldn't create your recipe: ${error}`);
        }
    });
}

// Register completion provider for Brewco
function registerCompletionProvider(context: vscode.ExtensionContext) {
    const provider = vscode.languages.registerCompletionItemProvider(
        'brewco',
        {
            provideCompletionItems(document: vscode.TextDocument, position: vscode.Position) {
                const completions: vscode.CompletionItem[] = [];
                
                // Keywords
                const keywords = [
                    { label: 'beans', detail: 'Variable declaration', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'brew', detail: 'Function declaration', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'roast', detail: 'Class declaration', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'taste', detail: 'If statement', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'otherwise', detail: 'Else clause', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'steep', detail: 'For-each loop', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'pour', detail: 'While loop', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'serve', detail: 'Return statement', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'pourout', detail: 'Print output', kind: vscode.CompletionItemKind.Function },
                    { label: 'grind', detail: 'Import module', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'new', detail: 'Create new instance', kind: vscode.CompletionItemKind.Keyword },
                    { label: 'blend', detail: 'Inheritance', kind: vscode.CompletionItemKind.Keyword }
                ];
                
                // Operators
                const operators = [
                    { label: 'pour_in', detail: 'Assignment (=)', kind: vscode.CompletionItemKind.Operator },
                    { label: 'same_blend', detail: 'Equality (==)', kind: vscode.CompletionItemKind.Operator },
                    { label: 'different_blend', detail: 'Not equal (!=)', kind: vscode.CompletionItemKind.Operator },
                    { label: 'more_caffeine', detail: 'Greater than (>)', kind: vscode.CompletionItemKind.Operator },
                    { label: 'less_caffeine', detail: 'Less than (<)', kind: vscode.CompletionItemKind.Operator },
                    { label: 'with', detail: 'Logical AND (&&)', kind: vscode.CompletionItemKind.Operator },
                    { label: 'or', detail: 'Logical OR (||)', kind: vscode.CompletionItemKind.Operator },
                    { label: 'add', detail: 'Addition (+)', kind: vscode.CompletionItemKind.Operator },
                    { label: 'sip', detail: 'Subtraction (-)', kind: vscode.CompletionItemKind.Operator }
                ];
                
                // Native functions with detailed signatures
                const nativeFunctions = [
                    // String functions
                    { label: 'string_length', detail: 'string_length(str: String) -> Number', kind: vscode.CompletionItemKind.Function },
                    { label: 'brew_blend', detail: 'brew_blend(str1: String, str2: String) -> String', kind: vscode.CompletionItemKind.Function },
                    { label: 'foam_up', detail: 'foam_up(str: String) -> String', kind: vscode.CompletionItemKind.Function },
                    { label: 'settle_down', detail: 'settle_down(str: String) -> String', kind: vscode.CompletionItemKind.Function },
                    
                    // Array functions
                    { label: 'cup_size', detail: 'cup_size(array: Array) -> Number', kind: vscode.CompletionItemKind.Function },
                    { label: 'add_to_cup', detail: 'add_to_cup(array: Array, element: Any) -> Array', kind: vscode.CompletionItemKind.Function },
                    { label: 'pour_together', detail: 'pour_together(arr1: Array, arr2: Array) -> Array', kind: vscode.CompletionItemKind.Function },
                    
                    // Math functions
                    { label: 'brew_minimum', detail: 'brew_minimum(a: Number, b: Number) -> Number', kind: vscode.CompletionItemKind.Function },
                    { label: 'brew_maximum', detail: 'brew_maximum(a: Number, b: Number) -> Number', kind: vscode.CompletionItemKind.Function },
                    { label: 'perfect_temperature', detail: 'perfect_temperature(num: Number) -> Number', kind: vscode.CompletionItemKind.Function },
                    { label: 'extra_shot', detail: 'extra_shot(num: Number) -> Number', kind: vscode.CompletionItemKind.Function },
                    
                    // IO functions
                    { label: 'sip_file', detail: 'sip_file(path: String) -> String', kind: vscode.CompletionItemKind.Function },
                    { label: 'pour_to_file', detail: 'pour_to_file(path: String, content: String) -> Boolean', kind: vscode.CompletionItemKind.Function },
                    { label: 'recipe_exists', detail: 'recipe_exists(path: String) -> Boolean', kind: vscode.CompletionItemKind.Function },
                    
                    // Type functions
                    { label: 'type_check', detail: 'type_check(value: Any) -> String', kind: vscode.CompletionItemKind.Function },
                    { label: 'is_coffee_number', detail: 'is_coffee_number(value: Any) -> Boolean', kind: vscode.CompletionItemKind.Function },
                    { label: 'is_coffee_string', detail: 'is_coffee_string(value: Any) -> Boolean', kind: vscode.CompletionItemKind.Function },
                    
                    // Utility functions
                    { label: 'coffee_debug', detail: 'coffee_debug(value: Any) -> String', kind: vscode.CompletionItemKind.Function },
                    { label: 'barista_help', detail: 'barista_help() -> String', kind: vscode.CompletionItemKind.Function }
                ];
                
                return [...keywords, ...operators, ...nativeFunctions];
            }
        }
    );
    
    context.subscriptions.push(provider);
}

// Register hover provider for Brewco
function registerHoverProvider(context: vscode.ExtensionContext) {
    const provider = vscode.languages.registerHoverProvider('brewco', {
        provideHover(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            const word = document.getText(range);
            
            // Find function info
            const func = getNativeFunctionInfo(word);
            if (func) {
                const markdown = new vscode.MarkdownString();
                markdown.appendMarkdown(`**${func.name}** - ${func.description}\n\n`);
                markdown.appendCodeblock(func.signature, 'brewco');
                markdown.appendMarkdown(`\n*Category: ${func.category}*`);
                return new vscode.Hover(markdown);
            }
            
            return null;
        }
    });
    
    context.subscriptions.push(provider);
}

// Get native function information
function getNativeFunctionInfo(functionName: string) {
    const functions: any = {
        // String functions
        'string_length': { name: 'string_length', signature: 'string_length(str: String) -> Number', description: 'Get the length of a string', category: 'String' },
        'brew_blend': { name: 'brew_blend', signature: 'brew_blend(str1: String, str2: String) -> String', description: 'Concatenate two strings', category: 'String' },
        'foam_up': { name: 'foam_up', signature: 'foam_up(str: String) -> String', description: 'Convert string to uppercase', category: 'String' },
        'settle_down': { name: 'settle_down', signature: 'settle_down(str: String) -> String', description: 'Convert string to lowercase', category: 'String' },
        
        // Array functions
        'cup_size': { name: 'cup_size', signature: 'cup_size(array: Array) -> Number', description: 'Get the size of an array', category: 'Array' },
        'add_to_cup': { name: 'add_to_cup', signature: 'add_to_cup(array: Array, element: Any) -> Array', description: 'Add element to array', category: 'Array' },
        'pour_together': { name: 'pour_together', signature: 'pour_together(arr1: Array, arr2: Array) -> Array', description: 'Concatenate two arrays', category: 'Array' },
        
        // Math functions
        'brew_minimum': { name: 'brew_minimum', signature: 'brew_minimum(a: Number, b: Number) -> Number', description: 'Get minimum of two numbers', category: 'Math' },
        'brew_maximum': { name: 'brew_maximum', signature: 'brew_maximum(a: Number, b: Number) -> Number', description: 'Get maximum of two numbers', category: 'Math' },
        'perfect_temperature': { name: 'perfect_temperature', signature: 'perfect_temperature(num: Number) -> Number', description: 'Get square root of number', category: 'Math' },
        
        // IO functions
        'sip_file': { name: 'sip_file', signature: 'sip_file(path: String) -> String', description: 'Read file contents', category: 'IO' },
        'pour_to_file': { name: 'pour_to_file', signature: 'pour_to_file(path: String, content: String) -> Boolean', description: 'Write content to file', category: 'IO' },
        'recipe_exists': { name: 'recipe_exists', signature: 'recipe_exists(path: String) -> Boolean', description: 'Check if file exists', category: 'IO' },
        
        // Type functions
        'type_check': { name: 'type_check', signature: 'type_check(value: Any) -> String', description: 'Get type of value', category: 'Type' },
        'is_coffee_number': { name: 'is_coffee_number', signature: 'is_coffee_number(value: Any) -> Boolean', description: 'Check if value is number', category: 'Type' },
        'is_coffee_string': { name: 'is_coffee_string', signature: 'is_coffee_string(value: Any) -> Boolean', description: 'Check if value is string', category: 'Type' }
    };
    
    return functions[functionName] || null;
}

// Generate Brewco file template
function generateBrewcoTemplate(fileName: string): string {
    return `🎀 ${fileName} - A delicious Brewco recipe ☕
🎀 Created with love using the Brewco VS Code extension!
🎀 Author: Khushi Motwani 💖

beans greeting pour_in "Hello, Brewco World!"
beans coffee_strength pour_in 10

pourout(greeting)
pourout("Coffee strength:", coffee_strength)

🎀 Add your amazing coffee code here! ☕✨

brew make_perfect_coffee(strength) {
    taste (strength more_caffeine 8) {
        serve "Perfect espresso! ☕"
    } otherwise {
        serve "Needs more coffee beans! 🫘"
    }
}

beans perfect_brew pour_in make_perfect_coffee(coffee_strength)
pourout(perfect_brew)

🎀 Happy brewing with Brewco! ☕💖`;
}

export function deactivate() {
    console.log('☕ Brewco extension is taking a coffee break! See you soon! ☕');
} 