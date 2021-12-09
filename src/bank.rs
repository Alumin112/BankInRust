use crate::currency::{Currency, ITC};

#[derive(Debug)]
pub struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
        }
    }

    pub fn add_account(
        &mut self,
        name: &str,
        balance: impl Currency,
        pin: &str,
    ) -> Result<(), &str> {
        if self.is_same_name(name) {
            return Err("Account with the same name already exists");
        }
        self.accounts.push(Account::new(name, balance, pin));
        Ok(())
    }

    pub fn remove_account(&mut self, account: &str, pin: &str) -> Result<ITC, &str> {
        let mut i_ = None;
        for (i, account_) in self.accounts.iter().enumerate() {
            if account_.name == account {
                if account_.pin != pin {
                    return Err("Wrong pin");
                }
                i_ = Some(i);
                break;
            }
        }
        if let Some(i) = i_ {
            return Ok(self.accounts.remove(i).balance);
        }
        Err("Account not found")
    }

    pub fn get_account(&self, name: &str, pin: &str) -> Result<&Account, &str> {
        for account in self.accounts.iter() {
            if account.name == name {
                if account.pin != pin {
                    return Err("Wrong pin");
                }
                return Ok(account);
            }
        }
        Err("Account not found")
    }

    fn is_same_name(&self, name: &str) -> bool {
        for account in self.accounts.iter() {
            if account.name == name {
                return true;
            }
        }
        false
    }

    pub fn deposit(&mut self, account: &str, amount: impl Currency) -> Result<(), &str> {
        match self.get_account_mut(account) {
            Some(account) => {
                account.deposit(amount);
                Ok(())
            }
            None => Err("Account not found"),
        }
    }

    pub fn withdraw(
        &mut self,
        account: &str,
        amount: impl Currency,
        pin: &str,
    ) -> Result<ITC, &str> {
        match self.get_account_mut(account) {
            Some(acc) => {
                if acc.pin != pin {
                    return Err("Wrong Pin");
                }
                match acc.withdraw(amount) {
                    Some(mony) => Ok(mony),
                    None => Err("Account Underflow"),
                }
            }
            None => Err("Account not found"),
        }
    }

    fn get_account_mut(&mut self, name: &str) -> Option<&mut Account> {
        for account in self.accounts.iter_mut() {
            if account.name == name {
                return Some(account);
            }
        }
        None
    }

    pub fn transfer(
        &mut self,
        from: &str,
        to: &str,
        amount: impl Currency,
        pin: &str,
    ) -> Result<(), &str> {
        let acc = match self.get_account_mut(from) {
            Some(acc) => acc,
            None => return Err("from Account not found"),
        };
        if acc.pin != pin {
            return Err("Wrong pin");
        }
        let money = match acc.withdraw(amount) {
            Some(mony) => mony,
            None => return Err("Account Underflow"),
        };
        match self.get_account_mut(to) {
            Some(acc) => {
                acc.deposit(money);
                Ok(())
            }
            None => Err("to Account not found"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Account {
    name: String,
    balance: ITC,
    pin: String,
}

impl Account {
    fn deposit(&mut self, amount: impl Currency) {
        let amount = amount.to_itc();
        self.balance.0 += amount.0;
    }

    fn withdraw(&mut self, amount: impl Currency) -> Option<ITC> {
        let amount = amount.to_itc();
        if self.balance.0 >= amount.0 {
            self.balance.0 -= amount.0;
            return Some(amount);
        }
        None
    }

    fn new(name: &str, balance: impl Currency, pin: &str) -> Self {
        Self {
            name: name.to_string(),
            balance: balance.to_itc(),
            pin: pin.to_string(),
        }
    }

    pub fn balance(&self) -> &ITC {
        &self.balance
    }
}

#[macro_export]
macro_rules! account {
    ($bank: expr => + $name: expr, $balance: expr, $pin: tt) => {
        $bank
            .add_account($name, $balance, stringify!($pin))
            .unwrap()
    };

    ($bank: expr => - $name: expr, $pin: tt) => {
        $bank.remove_account($name, stringify!($pin)).unwrap()
    };

    ($bank: expr => ? $name: expr, $pin: tt) => {
        $bank.get_account($name, stringify!($pin)).unwrap()
    };
}

#[macro_export]
macro_rules! transaction {
    ($bank: expr => > $from: expr, $to: expr, $amount: expr, $pin: tt) => {
        $bank
            .transfer($from, $to, $amount, stringify!($pin))
            .unwrap()
    };
    ($bank: expr => - $account: expr, $amount: expr, $pin: tt) => {
        $bank.withdraw($account, $amount, stringify!($pin)).unwrap()
    };
    ($bank: expr => + $account: expr, $amount: expr) => {
        $bank.deposit($account, $amount).unwrap()
    };
}
