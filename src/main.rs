use std::io;
use num_format::{Locale, ToFormattedString};

struct Account {
    name: String,
    balance: f32
}

fn to_dollar_format(amount: f32) -> String {
    let input = format!("{:.2}", amount);
    let unformatted: Vec<&str> = input.split(".").collect();
    let int_part: u32 = unformatted[0].parse().expect("unable to parse int");
    let formatted = format!("${}.{}", int_part.to_formatted_string(&Locale::en), unformatted[1]);
    return formatted;
}

fn create_account(name: String) -> Account {
    return Account { name: name, balance: 0.0 };
}

fn get_user_input() -> String {
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("failed to read line");
    return response.trim().to_string();
}

fn deposit(mut account: Account) -> Account {
    println!("how much do you want to deposit?\n[(q)uit action]");
    let action = get_user_input();
    if action == "q".to_string() { return account };

    let amount: f32 = action.parse().unwrap();
    account.balance += amount;
    println!("added {} to account", to_dollar_format(amount));

    return account;
}

fn withdraw(mut account: Account) -> Account {
    println!("how much money do you want to withdraw?\n[(q)uit action]");
    let action = get_user_input();
    if action == "q".to_string() { return account };

    let amount: f32 = action.parse().unwrap();
    if amount > account.balance {
        println!("you do not have that amount of money!");
        return account;
    }
    
    account.balance -= amount;
    println!("withdrawed {} from account", to_dollar_format(amount));

    return account;
}

fn setup() -> Account {
    println!("hello there! welcome to coops bank.\nwhat is your name?");
    let name = get_user_input();
    if name == "null".to_string() {
        println!("your name is not null! bye!");
        return create_account("null".to_string());
    }
    
    println!("welcome {}! would you like to setup an accountout with us?\n[(y)es, (n)o]", name);
    if get_user_input() != "y" {
        println!("sorry to hear that, bye!");
        return create_account("null".to_string());
    }

    return create_account(name);
}

fn main() {
    let mut account: Account = setup();
    if account.name == "null".to_string() { return };
    
    loop {
        println!("{}'s account has this balance: {}", account.name, to_dollar_format(account.balance));
        println!("what wound you like to do?\n[(d)eposit, (w)ithdraw, (q)uit]");
        let action = get_user_input();
        if action == "q".to_string() { return };
        if action == "d".to_string() { account = deposit(account) };
        if action == "w".to_string() { account = withdraw(account) };
    }
}

