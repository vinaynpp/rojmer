
// A simple prsonal finance management - interactive console application in Rust
// that reads user input and responds accordingly.
use std::env;
use std::io::{self, Write};





// HashMap to store cash flow data
use std::collections::HashMap;
// Function to initialize the application
fn init_app() {
    // Initialize any necessary data structures or configurations here
    println!("Initializing the application...");
}
// Function to handle user input and process commands
fn handle_input(input: &str) {
    // Process the user input and respond accordingly
    match input.trim().to_lowercase().as_str() {
        "help" => println!("Available commands: help, exit"),
        "exit" => println!("Exiting the application..."),
        _ => println!("Unknown command: {}", input),
    }
}

fn handle_cash_flow(input: &str, cash_flow: &mut HashMap<String, Vec<String>>) {
    // Handle cash flow commands
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 2 {
        println!("Usage: cashflow <category> <tag>");
        return;
    }

    let category = parts[1].to_string();
    let tag = parts[2].to_string();

    let entry = cash_flow.entry(category.clone())
        .or_insert_with(Vec::new);
    entry.push(tag.clone());
    println!("Added cash flow entry: {} -> {}", category, tag);
    println!("Current cash flow for {}: {:?}", category, cash_flow.get(&category).unwrap());
}

fn print_cash_flow(cash_flow: &HashMap<String, Vec<String>>) {
    // Print the current cash flow data
    if cash_flow.is_empty() {
        println!("No cash flow data available.");
    } else {
        println!("Current cash flow data:");
        for (category, tags) in cash_flow {
            println!("{}: {:?}", category, tags);
        }
    }
}

fn handle_transaction(input: &str, txn: &mut HashMap<String, Vec<String>>) {
    // Handle transaction commands
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 2 {
        println!("Usage: transaction <category> <tag>");
        return;
    }

    let category = parts[1].to_string();
    let tag = parts[2].to_string();
    let amount = if parts.len() > 3 {
        parts[3].to_string()
    } else {
        "0".to_string() // Default amount if not provided
    };

    let entry = cash_flow.entry(category.clone())
        .or_insert_with(Vec::new);
    entry.push(format!("{}: {}", tag, amount));
    println!("Added transaction: {} -> {} ({})", category, tag, amount);
    println!("Current transactions for {}: {:?}", category, cash_flow.get(&category).unwrap());
}



fn main() {
    // Set the current working directory to the directory of the executable
    if let Some(exe_path) = env::current_exe().ok() {
        if let Some(dir) = exe_path.parent() {
            env::set_current_dir(dir).expect("Failed to set current directory");
        }
    }

    // Start the interactive console application
    run_console();
}
// Function to run the interactive console application
fn run_console() {
    use std::io::{self, Write};

    let mut cash_flow: HashMap<String, Vec<String>> = HashMap::new();
    // Print a welcome message
    println!("Welcome to the interactive console application!");
    println!("Type 'exit' to quit.");
    println!("Type 'help' for available commands.");
    println!("Type 'cf <category> <tag>' to add a cash flow entry.");
    println!("Type 'pcf' to display current cash flow data.");
    
    // Loop to read user input
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt is printed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let trimmed_input = input.trim();
        // Handle special commands
        if trimmed_input.eq_ignore_ascii_case("help") {
            println!("Available commands: help, exit, cf <category> <tag>, pcf");
            continue;
        } else if trimmed_input.eq_ignore_ascii_case("pcf") {
            // let cash_flow: HashMap<String, Vec<String>> = HashMap::new();
            print_cash_flow(&cash_flow);
            continue;
        } else if trimmed_input.starts_with("cf ") {
            handle_cash_flow(trimmed_input, &mut cash_flow);
            continue;
        }
     

    }
}
