#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount{
balance: initial_balance,
        }
        
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0
        {
        self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        // only withdraw if the amount is less than the balance and is not negative 
        if amount < self.balance && amount > 0.0
        {
        self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        // create the account 
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance,100.0); // testing if it matches 100.0
        let account2 = BankAccount::new(5050.55);
        assert_eq!(account2.balance,5050.55);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        // depositing a postive number 
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0); // balance is 150.0 
        assert_eq!(account.balance,150.0);
        // despositing a negative number EDGE CASE
        let mut account2 = BankAccount::new(200.0);
        account2.deposit(-50.0); // balance should remain the same
        assert_eq!(account2.balance,200.0);

    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        // writing a test for a withdraw greater than the balance, 
        // balance should remain the same 
        let mut account3 = BankAccount::new(500.0);
        account3.withdraw(1000.0);
        assert_eq!(account3.balance,500.0);
        // writing a test where the withdraw is a negative number 
        let mut account4 = BankAccount::new(500.0);
        account4.withdraw(-30.0); // EDGE CASE 
        // the balance should remain the same 
        assert_eq!(account4.balance,500.0);
    }

    // Add more tests here
    
}
    
