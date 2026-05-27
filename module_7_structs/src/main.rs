struct BankAccount {
    pub balance: i32,
    pub transactions: Vec<i32>,
}

fn main() {
    let mut account = BankAccount {
        balance: 0,
        transactions: Vec::new()
    };

    // I deposited 100$
    account.balance += 100;
    account.transactions.push(100);

    // I withdrew 20$
    account.balance -= 20;
    account.transactions.push(-20);

    // I withdrew another 30$
    account.balance -= 30;
    account.transactions.push(-30);

    // What's my current balance?
    println!("{}", account.balance);
    
    // How many withdrawals have I done overall?
}
