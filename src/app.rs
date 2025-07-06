use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};

// Import data structures from mark3.rs
#[derive(Clone)]
pub struct Bank {
    pub name: String,
    pub accountnumber: String,
    pub balance: f64,
}

#[derive(Clone)]
pub struct Bucket {
    pub name: String,
    pub balance: f64,
    pub bank: Bank,
}

#[derive(Clone)]
pub struct Tag {
    pub name: String,
    pub description: String,
    pub bucket: Bucket,
}

#[derive(Clone)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Clone)]
pub struct Transaction {
    pub id: u64,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub timestamp: u64,
    pub tags: Vec<Tag>,
    pub description: String,
}

pub enum InputMode {
    Normal,
    Editing {
        input: String,
        cursor_position: usize,
        field: InputField,
    },
}

#[derive(Clone)]
pub enum InputField {
    BankName,
    BankAccountNumber,
    BankBalance,
    BucketName,
    BucketBalance,
    BucketBankName,
    BucketAccountNumber,
    TagName,
    TagBucketName,
    TagDescription,
    TransactionType,
    TransactionAmount,
    TransactionTags,
    TransactionDescription,
}

pub struct InputState {
    pub mode: InputMode,
    pub bank_name: String,
    pub bank_account: String,
    pub bank_balance: String,
    pub bucket_name: String,
    pub bucket_balance: String,
    pub bucket_bank_name: String,
    pub bucket_account: String,
    pub tag_name: String,
    pub tag_bucket: String,
    pub tag_description: String,
    pub transaction_type: String,
    pub transaction_amount: String,
    pub transaction_tags: String,
    pub transaction_description: String,
    pub message: String,
}

/// Application state
pub struct App {
    pub should_quit: bool,
    pub menu_state: MenuState,
    pub user_data: UserData,
    pub input: InputState,
}

pub struct MenuState {
    pub selected: usize,
    pub items: Vec<String>,
}

pub struct UserData {
    pub bank: Vec<Bank>,
    pub bucket: Vec<Bucket>,
    pub tag: Vec<Tag>,
    pub transaction: Vec<Transaction>,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            menu_state: MenuState {
                selected: 0,
                items: vec![
                    "Banks".to_string(),
                    "Buckets".to_string(),
                    "Tags".to_string(),
                    "Transactions".to_string(),
                    "Exit".to_string(),
                ],
            },
            user_data: UserData {
                bank: Vec::new(),
                bucket: Vec::new(),
                tag: Vec::new(),
                transaction: Vec::new(),
            },
            input: InputState {
                mode: InputMode::Normal,
                bank_name: String::new(),
                bank_account: String::new(),
                bank_balance: String::new(),
                bucket_name: String::new(),
                bucket_balance: String::new(),
                bucket_bank_name: String::new(),
                bucket_account: String::new(),
                tag_name: String::new(),
                tag_bucket: String::new(),
                tag_description: String::new(),
                transaction_type: String::new(),
                transaction_amount: String::new(),
                transaction_tags: String::new(),
                transaction_description: String::new(),
                message: String::new(),
            },
        }
    }

    pub fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match &self.input.mode {
                    InputMode::Normal => self.handle_normal_mode(key.code, key.modifiers),
                    InputMode::Editing { field, .. } => self.handle_editing_mode(key.code, key.modifiers, (*field).clone()),
                }
            }
        }
        Ok(())
    }

    fn handle_normal_mode(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        match key {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Up => {
                if self.menu_state.selected > 0 {
                    self.menu_state.selected -= 1;
                } else {
                    // Wrap around to the bottom
                    self.menu_state.selected = self.menu_state.items.len() - 1;
                }
            }
            KeyCode::Down => {
                if self.menu_state.selected < self.menu_state.items.len() - 1 {
                    self.menu_state.selected += 1;
                } else {
                    // Wrap around to the top
                    self.menu_state.selected = 0;
                }
            }
            KeyCode::Enter => {
                // Handle menu selection
                if self.menu_state.selected == self.menu_state.items.len() - 1 {
                    // Exit option selected
                    self.should_quit = true;
                }
            }
            KeyCode::Char('n') => {
                // Start creating a new item based on the current tab
                match self.menu_state.selected {
                    0 => self.start_new_bank_input(),
                    1 => self.start_new_bucket_input(),
                    2 => self.start_new_tag_input(),
                    3 => self.start_new_transaction_input(),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn handle_editing_mode(&mut self, key: KeyCode, modifiers: KeyModifiers, field: InputField) {
        match key {
            KeyCode::Enter | KeyCode::Down => {
                // Use the move_to_next_field helper method
                self.move_to_next_field(field);
            }
            KeyCode::Up => {
                // Use the move_to_previous_field helper method
                self.move_to_previous_field(field);
            }
            KeyCode::Esc => {
                self.input.mode = InputMode::Normal;
                self.input.message = String::new();
            }
            KeyCode::Backspace => {
                if let InputMode::Editing { input, cursor_position, field: current_field } = &mut self.input.mode {
                    if *cursor_position > 0 {
                        input.remove(*cursor_position - 1);
                        *cursor_position -= 1;
                    }
                    // Update the appropriate field
                    let input_clone = input.clone();
                    let current_field = current_field.clone();
                    self.update_field_value(&current_field, &input_clone);
                }
            }
            KeyCode::Char(c) => {
                if let InputMode::Editing { input, cursor_position, field: current_field } = &mut self.input.mode {
                    input.insert(*cursor_position, c);
                    *cursor_position += 1;
                    // Update the appropriate field
                    let input_clone = input.clone();
                    let field_clone = current_field.clone();
                    self.update_field_value(&field_clone, &input_clone);
                }
            }
            _ => {}
        }
    }

    // Helper methods for starting input for different entities
    fn start_new_bank_input(&mut self) {
        self.clear_input_fields();
        self.input.mode = InputMode::Editing {
            input: String::new(),
            cursor_position: 0,
            field: InputField::BankName,
        };
    }

    fn start_new_bucket_input(&mut self) {
        self.clear_input_fields();
        self.input.mode = InputMode::Editing {
            input: String::new(),
            cursor_position: 0,
            field: InputField::BucketName,
        };
    }

    fn start_new_tag_input(&mut self) {
        self.clear_input_fields();
        self.input.mode = InputMode::Editing {
            input: String::new(),
            cursor_position: 0,
            field: InputField::TagName,
        };
    }

    fn start_new_transaction_input(&mut self) {
        self.clear_input_fields();
        self.input.mode = InputMode::Editing {
            input: String::new(),
            cursor_position: 0,
            field: InputField::TransactionType,
        };
    }

    fn clear_input_fields(&mut self) {
        self.input.bank_name = String::new();
        self.input.bank_account = String::new();
        self.input.bank_balance = String::new();
        self.input.bucket_name = String::new();
        self.input.bucket_balance = String::new();
        self.input.bucket_bank_name = String::new();
        self.input.bucket_account = String::new();
        self.input.tag_name = String::new();
        self.input.tag_bucket = String::new();
        self.input.tag_description = String::new();
        self.input.transaction_type = String::new();
        self.input.transaction_amount = String::new();
        self.input.transaction_tags = String::new();
        self.input.transaction_description = String::new();
        self.input.message = String::new();
    }

    // Implementation of add functions
    fn add_bank(&mut self) {
        // Parse bank balance
        let balance = match self.input.bank_balance.parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                self.input.message = "Invalid balance format".to_string();
                return;
            }
        };

        // Check if bank already exists
        if self.user_data.bank.iter().any(|b| 
            b.name == self.input.bank_name && 
            b.accountnumber == self.input.bank_account) {
                self.input.message = format!(
                    "Bank with name '{}' and account '{}' already exists",
                    self.input.bank_name, self.input.bank_account);
                return;
        }

        // Add new bank
        let new_bank = Bank {
            name: self.input.bank_name.clone(),
            accountnumber: self.input.bank_account.clone(),
            balance,
        };
        
        self.user_data.bank.push(new_bank);
        self.input.message = "Bank added successfully".to_string();
        self.clear_input_fields();
    }

    fn add_bucket(&mut self) {
        // Parse bucket balance
        let balance = match self.input.bucket_balance.parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                self.input.message = "Invalid balance format".to_string();
                return;
            }
        };

        // Check if bucket already exists
        if self.user_data.bucket.iter().any(|b| b.name == self.input.bucket_name) {
            self.input.message = format!("Bucket with name '{}' already exists", self.input.bucket_name);
            return;
        }

        // Find existing bank or create a new one
        let bank = if let Some(existing_bank) = self.user_data.bank.iter().find(|b| 
            b.name == self.input.bucket_bank_name && 
            b.accountnumber == self.input.bucket_account) {
                existing_bank.clone()
        } else {
            let new_bank = Bank {
                name: self.input.bucket_bank_name.clone(),
                accountnumber: self.input.bucket_account.clone(),
                balance: 0.0,
            };
            self.user_data.bank.push(new_bank.clone());
            new_bank
        };

        // Add new bucket
        let new_bucket = Bucket {
            name: self.input.bucket_name.clone(),
            balance,
            bank,
        };
        
        self.user_data.bucket.push(new_bucket);
        self.input.message = "Bucket added successfully".to_string();
        self.clear_input_fields();
    }

    fn add_tag(&mut self) {
        // Check if tag already exists
        if self.user_data.tag.iter().any(|t| t.name == self.input.tag_name) {
            self.input.message = format!("Tag with name '{}' already exists", self.input.tag_name);
            return;
        }

        // Find bucket or show error
        let bucket = if let Some(existing_bucket) = self.user_data.bucket.iter().find(|b| 
            b.name == self.input.tag_bucket) {
                existing_bucket.clone()
        } else {
            self.input.message = format!("Bucket with name '{}' does not exist", self.input.tag_bucket);
            return;
        };

        // Add new tag
        let new_tag = Tag {
            name: self.input.tag_name.clone(),
            description: self.input.tag_description.clone(),
            bucket,
        };
        
        self.user_data.tag.push(new_tag);
        self.input.message = "Tag added successfully".to_string();
        self.clear_input_fields();
    }

    fn add_transaction(&mut self) {
        // Parse transaction type
        let transaction_type = match self.input.transaction_type.to_lowercase().as_str() {
            "i" | "income" => TransactionType::Income,
            "e" | "expense" => TransactionType::Expense,
            _ => {
                self.input.message = "Invalid transaction type. Use 'i' for income or 'e' for expense".to_string();
                return;
            }
        };

        // Parse transaction amount
        let amount = match self.input.transaction_amount.parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                self.input.message = "Invalid amount format".to_string();
                return;
            }
        };

        // Parse and validate tags
        let mut tags = Vec::new();
        if !self.input.transaction_tags.is_empty() {
            for tag_name in self.input.transaction_tags.split(',') {
                let tag_name = tag_name.trim();
                if let Some(tag) = self.user_data.tag.iter().find(|t| t.name == tag_name) {
                    tags.push(tag.clone());
                } else {
                    self.input.message = format!("Tag '{}' does not exist", tag_name);
                    return;
                }
            }
        }

        // Get timestamp
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Create transaction ID
        let transaction_id = self.user_data.transaction.len() as u64 + 1;

        // Add new transaction
        let new_transaction = Transaction {
            id: transaction_id,
            transaction_type,
            amount,
            timestamp,
            tags,
            description: self.input.transaction_description.clone(),
        };
        
        self.user_data.transaction.push(new_transaction);
        self.input.message = "Transaction added successfully".to_string();
        self.clear_input_fields();
    }

    // New helper methods for editing mode navigation
    fn move_to_next_field(&mut self, current_field: InputField) {
        match current_field {
            // Bank form navigation
            InputField::BankName => {
                self.switch_to_field(InputField::BankAccountNumber, self.input.bank_account.clone());
            }
            InputField::BankAccountNumber => {
                self.switch_to_field(InputField::BankBalance, self.input.bank_balance.clone());
            }
            InputField::BankBalance => {
                self.add_bank();
                self.input.mode = InputMode::Normal;
            }
            
            // Bucket form navigation
            InputField::BucketName => {
                self.switch_to_field(InputField::BucketBalance, self.input.bucket_balance.clone());
            }
            InputField::BucketBalance => {
                self.switch_to_field(InputField::BucketBankName, self.input.bucket_bank_name.clone());
            }
            InputField::BucketBankName => {
                self.switch_to_field(InputField::BucketAccountNumber, self.input.bucket_account.clone());
            }
            InputField::BucketAccountNumber => {
                self.add_bucket();
                self.input.mode = InputMode::Normal;
            }
            
            // Tag form navigation
            InputField::TagName => {
                self.switch_to_field(InputField::TagBucketName, self.input.tag_bucket.clone());
            }
            InputField::TagBucketName => {
                self.switch_to_field(InputField::TagDescription, self.input.tag_description.clone());
            }
            InputField::TagDescription => {
                self.add_tag();
                self.input.mode = InputMode::Normal;
            }
            
            // Transaction form navigation
            InputField::TransactionType => {
                self.switch_to_field(InputField::TransactionAmount, self.input.transaction_amount.clone());
            }
            InputField::TransactionAmount => {
                self.switch_to_field(InputField::TransactionTags, self.input.transaction_tags.clone());
            }
            InputField::TransactionTags => {
                self.switch_to_field(InputField::TransactionDescription, self.input.transaction_description.clone());
            }
            InputField::TransactionDescription => {
                self.add_transaction();
                self.input.mode = InputMode::Normal;
            }
        }
    }

    fn move_to_previous_field(&mut self, current_field: InputField) {
        match current_field {
            // Bank form navigation - backwards
            InputField::BankName => {
                // Already at the first field, do nothing
            }
            InputField::BankAccountNumber => {
                self.switch_to_field(InputField::BankName, self.input.bank_name.clone());
            }
            InputField::BankBalance => {
                self.switch_to_field(InputField::BankAccountNumber, self.input.bank_account.clone());
            }
            
            // Bucket form navigation - backwards
            InputField::BucketName => {
                // Already at the first field, do nothing
            }
            InputField::BucketBalance => {
                self.switch_to_field(InputField::BucketName, self.input.bucket_name.clone());
            }
            InputField::BucketBankName => {
                self.switch_to_field(InputField::BucketBalance, self.input.bucket_balance.clone());
            }
            InputField::BucketAccountNumber => {
                self.switch_to_field(InputField::BucketBankName, self.input.bucket_bank_name.clone());
            }
            
            // Tag form navigation - backwards
            InputField::TagName => {
                // Already at the first field, do nothing
            }
            InputField::TagBucketName => {
                self.switch_to_field(InputField::TagName, self.input.tag_name.clone());
            }
            InputField::TagDescription => {
                self.switch_to_field(InputField::TagBucketName, self.input.tag_bucket.clone());
            }
            
            // Transaction form navigation - backwards
            InputField::TransactionType => {
                // Already at the first field, do nothing
            }
            InputField::TransactionAmount => {
                self.switch_to_field(InputField::TransactionType, self.input.transaction_type.clone());
            }
            InputField::TransactionTags => {
                self.switch_to_field(InputField::TransactionAmount, self.input.transaction_amount.clone());
            }
            InputField::TransactionDescription => {
                self.switch_to_field(InputField::TransactionTags, self.input.transaction_tags.clone());
            }
        }
    }

    fn switch_to_field(&mut self, field: InputField, value: String) {
        let cursor_position = value.len();
        self.input.mode = InputMode::Editing {
            input: value,
            cursor_position,
            field,
        };
    }

    fn update_field_value(&mut self, field: &InputField, value: &str) {
        match field {
            InputField::BankName => self.input.bank_name = value.to_string(),
            InputField::BankAccountNumber => self.input.bank_account = value.to_string(),
            InputField::BankBalance => self.input.bank_balance = value.to_string(),
            InputField::BucketName => self.input.bucket_name = value.to_string(),
            InputField::BucketBalance => self.input.bucket_balance = value.to_string(),
            InputField::BucketBankName => self.input.bucket_bank_name = value.to_string(),
            InputField::BucketAccountNumber => self.input.bucket_account = value.to_string(),
            InputField::TagName => self.input.tag_name = value.to_string(),
            InputField::TagBucketName => self.input.tag_bucket = value.to_string(),
            InputField::TagDescription => self.input.tag_description = value.to_string(),
            InputField::TransactionType => self.input.transaction_type = value.to_string(),
            InputField::TransactionAmount => self.input.transaction_amount = value.to_string(),
            InputField::TransactionTags => self.input.transaction_tags = value.to_string(),
            InputField::TransactionDescription => self.input.transaction_description = value.to_string(),
        }
    }
}