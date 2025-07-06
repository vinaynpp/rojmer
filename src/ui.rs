use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Style, Modifier},
    text::{Span, Line, Text},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState, Table, Row, Cell},
    Frame,
};

use crate::app::{App, InputMode, InputField, TransactionType};

/// Render the UI
pub fn render(f: &mut Frame, app: &App) {
    // Create main vertical layout
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),  // Left sidebar
            Constraint::Percentage(80),  // Content area
        ].as_ref())
        .split(f.size());

    // Render the left sidebar with tabs
    render_sidebar(f, app, main_layout[0]);
    
    // Render the content area based on selected tab
    render_content(f, app, main_layout[1]);
}

fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    // Create vertical layout for sidebar
    let sidebar_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),       // Title
            Constraint::Min(5),          // Navigation list
            Constraint::Length(3),       // Help
        ].as_ref())
        .split(area);
    
    // Title block at top of sidebar
    let title = Paragraph::new("Rojmer")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, sidebar_layout[0]);
    
    // Create vertical navigation items
    let nav_items: Vec<ListItem> = app.menu_state.items
        .iter()
        .map(|item| {
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!(" {} ", item),
                    Style::default().fg(Color::White),
                )
            ]))
        })
        .collect();
    
    // Create vertical navigation list
    let nav_list = List::new(nav_items)
        .block(Block::default()
            .title("Menu")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White)))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" > ");  // Add indicator for selected item
    
    // Create a mutable list state and set the selected item
    let mut list_state = ListState::default();
    list_state.select(Some(app.menu_state.selected));
    
    // Render the list as a stateful widget
    f.render_stateful_widget(nav_list, sidebar_layout[1], &mut list_state);
    
    // Help text at bottom of sidebar
    let help_text = match app.input.mode {
        InputMode::Normal => "n: New | ↑/↓: Navigate | Enter: Select | q: Quit",
        InputMode::Editing { .. } => "↑/↓: Prev/Next Field | Enter: Next Field | Esc: Cancel",
    };
    
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, sidebar_layout[2]);
}

fn render_content(f: &mut Frame, app: &App, area: Rect) {
    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),     // Content title and data
            Constraint::Length(3),  // Message area
        ].as_ref())
        .split(area);
    
    let content_block = Block::default()
        .borders(Borders::ALL)
        .title(app.menu_state.items[app.menu_state.selected].clone());
    
    f.render_widget(content_block.clone(), content_layout[0]);
    
    // Calculate the inner area of the content block
    let inner_area = content_block.inner(content_layout[0]);
    
    // Display different content based on selected tab
    match app.menu_state.selected {
        0 => render_banks(f, app, inner_area),
        1 => render_buckets(f, app, inner_area),
        2 => render_tags(f, app, inner_area),
        3 => render_transactions(f, app, inner_area),
        _ => {}
    }
    
    // Render message area
    if !app.input.message.is_empty() {
        let message = Paragraph::new(app.input.message.clone())
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::ALL).title("Message"));
        f.render_widget(message, content_layout[1]);
    }
}

fn render_banks(f: &mut Frame, app: &App, area: Rect) {
    match &app.input.mode {
        InputMode::Normal => {
            // In normal mode, display the list of banks
            if app.user_data.bank.is_empty() {
                let empty_msg = Paragraph::new("No banks available. Press 'n' to add a new bank.")
                    .alignment(Alignment::Center);
                f.render_widget(empty_msg, area);
            } else {
                // Create a table to display banks
                let header_cells = ["Name", "Account Number", "Balance"]
                    .iter()
                    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
                let header = Row::new(header_cells)
                    .style(Style::default())
                    .height(1);
                
                let rows = app.user_data.bank.iter().map(|bank| {
                    let cells = [
                        Cell::from(bank.name.clone()),
                        Cell::from(bank.accountnumber.clone()),
                        Cell::from(format!("{:.2}", bank.balance)),
                    ];
                    Row::new(cells)
                });
                
                let widths = [
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ];
                
                // Initialize table with required widths
                let table = Table::new(
                    rows,
                    widths,
                )
                .header(header)
                .block(Block::default());
                
                f.render_widget(table, area);
            }
        },
        InputMode::Editing { field, .. } => {
            // In editing mode, show the input form for a bank
            let input_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ].as_ref())
                .split(area);
            
            // Determine which field is active
            let name_style = if matches!(field, InputField::BankName) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let account_style = if matches!(field, InputField::BankAccountNumber) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let balance_style = if matches!(field, InputField::BankBalance) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            // Render input fields
            let name_input = Paragraph::new(app.input.bank_name.clone())
                .style(name_style)
                .block(Block::default().borders(Borders::ALL).title("Bank Name"));
            
            let account_input = Paragraph::new(app.input.bank_account.clone())
                .style(account_style)
                .block(Block::default().borders(Borders::ALL).title("Account Number"));
            
            let balance_input = Paragraph::new(app.input.bank_balance.clone())
                .style(balance_style)
                .block(Block::default().borders(Borders::ALL).title("Balance"));
            
            f.render_widget(name_input, input_layout[0]);
            f.render_widget(account_input, input_layout[1]);
            f.render_widget(balance_input, input_layout[2]);
        }
    }
}

fn render_buckets(f: &mut Frame, app: &App, area: Rect) {
    match &app.input.mode {
        InputMode::Normal => {
            // In normal mode, display the list of buckets
            if app.user_data.bucket.is_empty() {
                let empty_msg = Paragraph::new("No buckets available. Press 'n' to add a new bucket.")
                    .alignment(Alignment::Center);
                f.render_widget(empty_msg, area);
            } else {
                // Create a table to display buckets
                let header_cells = ["Name", "Balance", "Bank", "Account Number"]
                    .iter()
                    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
                let header = Row::new(header_cells)
                    .style(Style::default())
                    .height(1);
                
                let rows = app.user_data.bucket.iter().map(|bucket| {
                    let cells = [
                        Cell::from(bucket.name.clone()),
                        Cell::from(format!("{:.2}", bucket.balance)),
                        Cell::from(bucket.bank.name.clone()),
                        Cell::from(bucket.bank.accountnumber.clone()),
                    ];
                    Row::new(cells)
                });
                
                let widths = [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ];
                
                // Initialize table with required widths
                let table = Table::new(
                    rows,
                    widths,
                )
                .header(header)
                .block(Block::default());
                
                f.render_widget(table, area);
            }
        },
        InputMode::Editing { field, .. } => {
            // In editing mode, show the input form for a bucket
            let input_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ].as_ref())
                .split(area);
            
            // Determine which field is active
            let name_style = if matches!(field, InputField::BucketName) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let balance_style = if matches!(field, InputField::BucketBalance) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let bank_name_style = if matches!(field, InputField::BucketBankName) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let account_style = if matches!(field, InputField::BucketAccountNumber) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            // Render input fields
            let name_input = Paragraph::new(app.input.bucket_name.clone())
                .style(name_style)
                .block(Block::default().borders(Borders::ALL).title("Bucket Name"));
            
            let balance_input = Paragraph::new(app.input.bucket_balance.clone())
                .style(balance_style)
                .block(Block::default().borders(Borders::ALL).title("Balance"));
            
            let bank_name_input = Paragraph::new(app.input.bucket_bank_name.clone())
                .style(bank_name_style)
                .block(Block::default().borders(Borders::ALL).title("Bank Name"));
            
            let account_input = Paragraph::new(app.input.bucket_account.clone())
                .style(account_style)
                .block(Block::default().borders(Borders::ALL).title("Account Number"));
            
            f.render_widget(name_input, input_layout[0]);
            f.render_widget(balance_input, input_layout[1]);
            f.render_widget(bank_name_input, input_layout[2]);
            f.render_widget(account_input, input_layout[3]);
        }
    }
}

fn render_tags(f: &mut Frame, app: &App, area: Rect) {
    match &app.input.mode {
        InputMode::Normal => {
            // In normal mode, display the list of tags
            if app.user_data.tag.is_empty() {
                let empty_msg = Paragraph::new("No tags available. Press 'n' to add a new tag.")
                    .alignment(Alignment::Center);
                f.render_widget(empty_msg, area);
            } else {
                // Create a table to display tags
                let header_cells = ["Name", "Description", "Bucket"]
                    .iter()
                    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
                let header = Row::new(header_cells)
                    .style(Style::default())
                    .height(1);
                
                let rows = app.user_data.tag.iter().map(|tag| {
                    let cells = [
                        Cell::from(tag.name.clone()),
                        Cell::from(tag.description.clone()),
                        Cell::from(tag.bucket.name.clone()),
                    ];
                    Row::new(cells)
                });
                
                let widths = [
                    Constraint::Percentage(20),
                    Constraint::Percentage(50),
                    Constraint::Percentage(30),
                ];
                
                // Initialize table with required widths
                let table = Table::new(
                    rows,
                    widths,
                )
                .header(header)
                .block(Block::default());
                
                f.render_widget(table, area);
            }
        },
        InputMode::Editing { field, .. } => {
            // In editing mode, show the input form for a tag
            let input_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ].as_ref())
                .split(area);
            
            // Determine which field is active
            let name_style = if matches!(field, InputField::TagName) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let bucket_style = if matches!(field, InputField::TagBucketName) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let description_style = if matches!(field, InputField::TagDescription) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            // Render input fields
            let name_input = Paragraph::new(app.input.tag_name.clone())
                .style(name_style)
                .block(Block::default().borders(Borders::ALL).title("Tag Name"));
            
            let bucket_input = Paragraph::new(app.input.tag_bucket.clone())
                .style(bucket_style)
                .block(Block::default().borders(Borders::ALL).title("Bucket Name"));
            
            let description_input = Paragraph::new(app.input.tag_description.clone())
                .style(description_style)
                .block(Block::default().borders(Borders::ALL).title("Description"));
            
            f.render_widget(name_input, input_layout[0]);
            f.render_widget(bucket_input, input_layout[1]);
            f.render_widget(description_input, input_layout[2]);
        }
    }
}

fn render_transactions(f: &mut Frame, app: &App, area: Rect) {
    match &app.input.mode {
        InputMode::Normal => {
            // In normal mode, display the list of transactions
            if app.user_data.transaction.is_empty() {
                let empty_msg = Paragraph::new("No transactions available. Press 'n' to add a new transaction.")
                    .alignment(Alignment::Center);
                f.render_widget(empty_msg, area);
            } else {
                // Create a table to display transactions
                let header_cells = ["ID", "Type", "Amount", "Description", "Tags"]
                    .iter()
                    .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
                let header = Row::new(header_cells)
                    .style(Style::default())
                    .height(1);
                
                let rows = app.user_data.transaction.iter().map(|transaction| {
                    let txn_type = match transaction.transaction_type {
                        TransactionType::Income => "Income",
                        TransactionType::Expense => "Expense",
                    };
                    
                    let tags = transaction.tags
                        .iter()
                        .map(|tag| tag.name.clone())
                        .collect::<Vec<String>>()
                        .join(", ");
                    
                    let cells = [
                        Cell::from(transaction.id.to_string()),
                        Cell::from(txn_type),
                        Cell::from(format!("{:.2}", transaction.amount)),
                        Cell::from(transaction.description.clone()),
                        Cell::from(tags),
                    ];
                    Row::new(cells)
                });
                
                let widths = [
                    Constraint::Percentage(10),
                    Constraint::Percentage(15),
                    Constraint::Percentage(15),
                    Constraint::Percentage(30),
                    Constraint::Percentage(30),
                ];
                
                // Initialize table with required widths
                let table = Table::new(
                    rows,
                    widths,
                )
                .header(header)
                .block(Block::default());
                
                f.render_widget(table, area);
            }
        },
        InputMode::Editing { field, .. } => {
            // In editing mode, show the input form for a transaction
            let input_layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ].as_ref())
                .split(area);
            
            // Determine which field is active
            let type_style = if matches!(field, InputField::TransactionType) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let amount_style = if matches!(field, InputField::TransactionAmount) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let tags_style = if matches!(field, InputField::TransactionTags) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            let description_style = if matches!(field, InputField::TransactionDescription) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            
            // Render input fields
            let type_input = Paragraph::new(app.input.transaction_type.clone())
                .style(type_style)
                .block(Block::default().borders(Borders::ALL).title("Type (i/e)"));
            
            let amount_input = Paragraph::new(app.input.transaction_amount.clone())
                .style(amount_style)
                .block(Block::default().borders(Borders::ALL).title("Amount"));
            
            let tags_input = Paragraph::new(app.input.transaction_tags.clone())
                .style(tags_style)
                .block(Block::default().borders(Borders::ALL).title("Tags (comma-separated)"));
            
            let description_input = Paragraph::new(app.input.transaction_description.clone())
                .style(description_style)
                .block(Block::default().borders(Borders::ALL).title("Description"));
            
            f.render_widget(type_input, input_layout[0]);
            f.render_widget(amount_input, input_layout[1]);
            f.render_widget(tags_input, input_layout[2]);
            f.render_widget(description_input, input_layout[3]);
        }
    }
}