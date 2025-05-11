#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id: id,
            holder: holder,
            balance: 0,
        }
    }
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

fn change_account(account: &mut Account) {
    account.balance = 10;
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("me"));

    bank.add_account(account);

    println!("{:#?}", bank)
}
