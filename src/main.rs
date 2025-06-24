// use std::env;

#[derive(Clone)]
struct Bank {
    name: String,
    accountnumber: String,
    balance: f64,
}

#[derive(Clone)]
struct Bucket {
    name: String,
    balance: f64,
    bank: Bank,
}

#[derive(Clone)]
struct Tag {
    name: String,
    description: String,
    bucket: Bucket,
}

enum TransactionType {
    Income,
    Expense,
}

// timestamp for transactions
use std::time::{SystemTime, UNIX_EPOCH};
// Transaction struct to hold transaction details
struct Transaction {
    id: u64,
    transaction_type: TransactionType,
    amount: f64,
    timestamp: u64,
    tags: Vec<Tag>,
    description: String,
}

enum TimePeriodType {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

struct TimePeriod {
    type_: TimePeriodType,
    start: u64,
    end: u64,
}


struct Budget {
    name: String,
    parent: String, // Name of the parent bank or bucket
    parent_type: String, // "bank" or "bucket"
    time_period: TimePeriod,
    amount: f64,
    description: String,
}



struct UserData {
    bank: Vec<Bank>,
    bucket: Vec<Bucket>,
    tag: Vec<Tag>,
    transaction: Vec<Transaction>,
    budget: Vec<Budget>,
}

fn handle_transaction(input: &str, user_data: &mut UserData) {
    // Handle transaction commands
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 3 {
        println!("Usage: txn <type> <amount> [tags] [description]");
        return;
    }

    let transaction_type = match parts[1].to_lowercase().as_str() {
        "i" => TransactionType::Income,
        "e" => TransactionType::Expense,
        _ => {
            println!("Invalid transaction type. Use 'income' or 'expense'.");
            return;
        }
    };

    let amount: f64 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount. Please enter a valid number.");
            return;
        }
    };

    let description = if parts.len() > 4 {
        parts[4..].join(" ")
    } else {
        String::new()
    };

    // create tag if not already exists
    let mut tags = Vec::new();
    if parts.len() > 3 {
        for tag_name in parts[3].split(',') {
            let tag_name = tag_name.trim();
            if let Some(tag) = user_data.tag.iter().find(|t| t.name == tag_name) {
                tags.push(tag.clone());
            } else {
                println!("Tag '{}' does not exist. Please create it first.", tag_name);
                return;
            }
        }
    }

    // Create a new transaction
    let transaction_id = user_data.transaction.len() as u64 + 1;

    let transaction = Transaction {
        id: transaction_id,
        transaction_type,
        amount,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
        description,
        tags,
    };

    // Store the transaction in user data
    user_data.transaction.push(transaction);
}

fn print_transactions(user_data: &UserData) {
    // Print all transactions
    if user_data.transaction.is_empty() {
        println!("No transactions available.");
    } else {
        for transaction in &user_data.transaction {
            println!("Transaction ID: {}", transaction.id);
            match transaction.transaction_type {
                TransactionType::Income => println!("Type: Income"),
                TransactionType::Expense => println!("Type: Expense"),
            }
            println!("Amount: {:.2}", transaction.amount);
            println!("Timestamp: {}", transaction.timestamp);
            println!("Description: {}", transaction.description);
            if !transaction.tags.is_empty() {
                println!("Tags:");
                for tag in &transaction.tags {
                    println!("- {}", tag.name);
                    println!("  Description: {}", tag.description);
                    println!("  Bucket: {}", tag.bucket.name);
                    println!("  Bucket Balance: {:.2}", tag.bucket.balance);
                    println!("  Bank: {}", tag.bucket.bank.name);
                    println!("  Account Number: {}", tag.bucket.bank.accountnumber);
                    println!("  Bank Balance: {:.2}", tag.bucket.bank.balance);
                }
            } else {
                println!("No tags associated with this transaction.");
            }
            println!("--------------------");
        }
    }
}

fn handle_bank(input: &str, user_data: &mut UserData) {
    // Handle bank commands
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 3 {
        println!("Usage: bank <name> <accountnumber> <balance>");
        return;
    }

    let name = parts[1].to_string();
    let accountnumber = parts[2].to_string();
    let balance: f64 = match parts[3].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid balance. Please enter a valid number.");
            return;
        }
    };

    let bank = Bank {
        name,
        accountnumber,
        balance,
    };
    // Check if the bank already exists
    if user_data
        .bank
        .iter()
        .any(|b| b.name == bank.name && b.accountnumber == bank.accountnumber)
    {
        println!(
            "Bank with name '{}' and account number '{}' already exists.",
            bank.name, bank.accountnumber
        );
        return;
    }

    // Store the bank in user data
    user_data.bank.push(bank);
}

fn print_banks(user_data: &UserData) {
    // Print all banks
    if user_data.bank.is_empty() {
        println!("No banks available.");
    } else {
        for bank in &user_data.bank {
            println!("Bank Name: {}", bank.name);
            println!("Account Number: {}", bank.accountnumber);
            println!("Balance: {:.2}", bank.balance);
            println!("--------------------");
        }
    }
}

fn handle_bucket(input: &str, user_data: &mut UserData) {
    // Handle bucket commands
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 4 {
        println!("Usage: bucket <name> <balance> [bank_name] [account_number]");
        return;
    }

    let name = parts[1].to_string();
    let balance: f64 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid balance. Please enter a valid number.");
            return;
        }
    };
    let bank_name = if parts.len() > 3 {
        parts[3].to_string()
    } else {
        String::new() // Default bank name if not provided
    };
    let account_number = if parts.len() > 4 {
        parts[4].to_string()
    } else {
        String::new() // Default account number if not provided
    };

    // Find existing bank or create a new one
    let bank = if let Some(existing_bank) = user_data
        .bank
        .iter()
        .find(|b| b.name == bank_name && b.accountnumber == account_number)
    {
        existing_bank.clone()
    } else {
        let newbank = Bank {
            name: bank_name.clone(),
            accountnumber: account_number.clone(),
            balance: 0.0,
        };
        user_data.bank.push(newbank.clone());
        newbank
    };

    let bucket = Bucket {
        name,
        balance,
        bank,
    };

    // Store the bucket in user data only if it doesn't already exist
    if user_data.bucket.iter().any(|b| b.name == bucket.name) {
        println!("Bucket with name '{}' already exists.", bucket.name);
        return;
    }

    user_data.bucket.push(bucket);
}

fn print_buckets(user_data: &UserData) {
    // Print all buckets
    if user_data.bucket.is_empty() {
        println!("No buckets available.");
    } else {
        for bucket in &user_data.bucket {
            println!("Bucket Name: {}", bucket.name);
            println!("Balance: {:.2}", bucket.balance);
            println!("Bank: {}", bucket.bank.name);
            println!("Account Number: {}", bucket.bank.accountnumber);
            println!("Bank Balance: {:.2}", bucket.bank.balance);
            println!("--------------------");
        }
    }
}

fn handle_tag(input: &str, user_data: &mut UserData) {
    // Handle tag commands
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() < 3 {
        println!("Usage: tag <name> <bucket_name> <description> ");
        return;
    }

    let name = parts[1].to_string();
    let bucket_name = parts[2].to_string();
    let description = if parts.len() > 3 {
        parts[3].to_string()
    } else {
        String::new() // Default bucket name if not provided
    };

    // Find existing bucket or create a new one
    let bucket =
        if let Some(existing_bucket) = user_data.bucket.iter().find(|b| b.name == bucket_name) {
            existing_bucket.clone()
        } else {
            println!("Bucket with name '{}' does not exist.", bucket_name);
            return;
        };

    let tag = Tag {
        name,
        description,
        bucket,
    };

    // Store the tag in user data only if it doesn't already exist
    if user_data.tag.iter().any(|t| t.name == tag.name) {
        println!("Tag with name '{}' already exists.", tag.name);
        return;
    }

    user_data.tag.push(tag);
}

fn print_tags(user_data: &UserData) {
    // Print all tags
    if user_data.tag.is_empty() {
        println!("No tags available.");
    } else {
        for tag in &user_data.tag {
            println!("Tag Name: {}", tag.name);
            println!("Description: {}", tag.description);
            println!("Bucket: {}", tag.bucket.name);
            println!("--------------------");
        }
    }
}

// Function to initialize the application
fn init_app() {
    // Initialize any necessary data structures or configurations here
    println!("Initializing the application...");
    // Print a welcome message
    println!("Welcome to the interactive console application!");
    println!("Type 'exit' to quit.");
    println!("Type 'help' for available commands.");
    println!("Type 'bank <name> <accountnumber> <balance>' to add a bank account.");
    println!("Type 'bucket <name> <balance> [bank_name] [account_number]' to add a bucket.");
    println!("Type 'tag <name> <bucket_name> <description>' to add a tag to a bucket.");
    println!("Type 'txn <type> <amount> [tags] [description]' to add a transaction with details.");
    println!("Type 'ls' to list all transactions.");
    println!("Type 'ls banks' to list all banks.");
    println!("Type 'ls buckets' to list all buckets.");
    println!("Type 'ls tags' to list all tags.");
}

fn handle_exit() {
    // Handle exit command
    println!("Exiting the application...");
    std::process::exit(0);
}

fn main() {
    // Set the current working directory to the directory of the executable
    // if let Some(exe_path) = env::current_exe().ok() {
    //     if let Some(dir) = exe_path.parent() {
    //         env::set_current_dir(dir).expect("Failed to set current directory");
    //     }
    // }
    // Start the interactive console application
    run_console();
}
// Function to run the interactive console application
fn run_console() {
    use std::io::{self, Write};

    let mut user1 = UserData {
        bank: Vec::new(),
        bucket: Vec::new(),
        tag: Vec::new(),
        transaction: Vec::new(),
        budget: Vec::new(),
    };
    init_app(); // Initialize the application

    // Loop to read user input
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt is printed immediately

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();
        // Handle special commands
        if trimmed_input.eq_ignore_ascii_case("help") {
            println!(
                "Available commands: help, exit, bank <name> <accountnumber> <balance>, bucket <name> <balance> [bank_name] [account_number], tag <name> <bucket_name> <description>, txn <type> <amount> [tags] [description], ls, ls banks, ls buckets, ls tags"
            );
            continue;
        } else if trimmed_input.eq_ignore_ascii_case("exit") {
            handle_exit();
        } else if trimmed_input.starts_with("bank ") {
            handle_bank(trimmed_input, &mut user1);
            continue;
        } else if trimmed_input.starts_with("bucket ") {
            handle_bucket(trimmed_input, &mut user1);
            continue;
        } else if trimmed_input.starts_with("tag ") {
            handle_tag(trimmed_input, &mut user1);
            continue;
        } else if trimmed_input.starts_with("txn ") {
            handle_transaction(trimmed_input, &mut user1);
            continue;
        } else if trimmed_input.eq_ignore_ascii_case("ls banks") {
            print_banks(&user1);
            continue;
        } else if trimmed_input.eq_ignore_ascii_case("ls buckets") {
            print_buckets(&user1);
            continue;
        } else if trimmed_input.eq_ignore_ascii_case("ls tags") {
            print_tags(&user1);
            continue;
        } else if trimmed_input.eq_ignore_ascii_case("ls") {
            print_transactions(&user1);
            continue;
        } else {
            println!("Unknown command: {}", trimmed_input);
            continue;
        }
    }
}
