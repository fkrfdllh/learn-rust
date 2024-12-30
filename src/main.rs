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

fn print_account(account: Account) {
    println!("{:#?}", account)
}

fn print_holder(holder: String) {
    println!("{}", holder);
}

fn main() {
    let account = Account::new(1, String::from("me"));

    print_holder(account.holder);
    print_account(account);
}
