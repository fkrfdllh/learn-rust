#[derive(Debug)]
struct Account {
    id: u128,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

fn main() {
    println!("Hello, world!");
}
