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

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    let mut account = Account::new(1, String::from("me"));

    /*
       immutable reference
       reference that cannot modify the value
    */
    let account_immutable_ref = &account;
    print_account(account_immutable_ref);

    /*
       mutable reference
       reference that can modify the value
       the rules:
       - mutable reference modify owner value
       - is only one mutable reference per variable
       - if there is value changed from the owner after initialization of mutable reference variable there will be an error
    */
    let account_mutable_ref = &mut account;
    account_mutable_ref.balance = 100;
    account_mutable_ref.holder = String::from("Fikri Fadillah");
    print_account(account_mutable_ref);

    println!("{:#?}", account)
}
