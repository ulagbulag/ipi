use ipi::account::Account;

fn main() {
    let account = Account::generate();
    println!("Public Key: {}", account.account_ref().to_string());
    println!("Private Key: {}", account.to_string());
}
