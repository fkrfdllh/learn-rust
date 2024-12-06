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

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

impl Account {
    fn new(id: u128, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
