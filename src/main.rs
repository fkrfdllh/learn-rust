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

fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account
}

fn print_holder(holder: String) {
    println!("{}", holder);
}

fn main() {
    let mut account = Account::new(1, String::from("me"));

    account = print_account(account);
    account = print_account(account);

    println!("{:#?}", account)
}
