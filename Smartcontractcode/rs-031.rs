use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum MoneyError {
    InsufficientBalance,
}

pub trait Money {
    fn balance_of(&self, account: &str) -> u32;
    fn mint(&mut self, dst: &str, amount: u32);
    fn transfer(&mut self, src: &str, dst: &str, amount: u32) -> Result<(), MoneyError>;
}

pub struct GoldCoin {
    // HashMap from account to balance
    pub balances: HashMap<String, u32>,
}

impl Money for GoldCoin {
    fn balance_of(&self, account: &str) -> u32 {
        *self.balances.get(account).unwrap_or(&0)
    }

    fn mint(&mut self, dst: &str, amount: u32) {
        let bal = self.balances.entry(dst.to_string()).or_insert(0);
        *bal += amount;
    }

    fn transfer(&mut self, src: &str, dst: &str, amount: u32) -> Result<(), MoneyError> {
        let src_bal = self.balances.entry(src.to_string()).or_insert(0);
        if *src_bal < amount {
            return Err(MoneyError::InsufficientBalance);
        }
        *src_bal -= amount;
        let dst_bal = self.balances.entry(dst.to_string()).or_insert(0);
        *dst_bal += amount;
        
        Ok(())
    }
}

pub fn balance_of(money: &impl Money, account: &str) -> u32 {
    money.balance_of(account)
}

pub fn mint(money: &mut impl Money, dst: &str, amount: u32) {
    money.mint(dst, amount);
}

pub fn transfer(
    money: &mut impl Money,
    src: &str,
    dst: &str,
    amount: u32,
) -> Result<(), MoneyError> {
    money.transfer(src, dst, amount)
}