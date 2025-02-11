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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account)
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
    /*
     * If you wanna put new account into bank's accounts
     * make sure to set bank to mutable
     */
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("me"));

    bank.add_account(account);

    println!("{:#?}", bank)
}
