#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    Address, Env, String, symbol_short, Vec,
};

const DAY: u32 = 17280;

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    TotalSupply,
    Name,
    Symbol,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotAdmin = 1,
    InsufficientBalance = 2,
    InvalidAmount = 3,
}

#[contract]
pub struct PaymentToken;

#[contractimpl]
impl PaymentToken {
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 { return Err(Error::InvalidAmount); }
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        
        let balance: i128 = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(balance + amount));
        // Dùng to.clone() ở đây để dành 'to' cho hàm publish bên dưới
        env.storage().persistent().extend_ttl(&DataKey::Balance(to.clone()), 29 * DAY, 30 * DAY);
        
        let supply: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(supply + amount));
        env.storage().instance().extend_ttl(6 * DAY, 7 * DAY);
        
        // Lần sử dụng cuối cùng của 'to' nên không cần clone
        env.events().publish((symbol_short!("mint"), to), amount);
        Ok(())
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 { return Err(Error::InvalidAmount); }
        from.require_auth();
        
        let from_bal = Self::balance(env.clone(), from.clone());
        if from_bal < amount { return Err(Error::InsufficientBalance); }
        let to_bal = Self::balance(env.clone(), to.clone());
        
        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_bal - amount));
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_bal + amount));
        
        // Clone ở đây để dành biến gốc cho sự kiện publish
        env.storage().persistent().extend_ttl(&DataKey::Balance(from.clone()), 29 * DAY, 30 * DAY);
        env.storage().persistent().extend_ttl(&DataKey::Balance(to.clone()), 29 * DAY, 30 * DAY);
        
        env.events().publish((symbol_short!("transfer"), from, to), amount);
        Ok(())
    }

    pub fn pay_invoice(env: Env, from: Address, to: Address, amount: i128, memo: String) -> Result<(), Error> {
        // Cần clone vì transfer sẽ tiêu thụ (consume) quyền sở hữu của from và to
        Self::transfer(env.clone(), from.clone(), to.clone(), amount)?;
        env.events().publish((symbol_short!("invoice"), from, to), memo);
        Ok(())
    }

    pub fn split_bill(env: Env, from_list: Vec<Address>, to: Address, amount_each: i128) -> Result<(), Error> {
        for from in from_list.iter() {
            // to.clone() cực kỳ quan trọng vì nó nằm trong vòng lặp
            Self::transfer(env.clone(), from, to.clone(), amount_each)?;
        }
        Ok(())
    }

    pub fn top_up(env: Env, student: Address, credits: i128) -> Result<(), Error> {
        Self::mint(env, student, credits)
    }

    pub fn tip(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        Self::transfer(env.clone(), from.clone(), to.clone(), amount)?;
        env.events().publish((symbol_short!("tip"), from, to), amount);
        Ok(())
    }

    pub fn balance(env: Env, addr: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(addr)).unwrap_or(0)
    }

    pub fn name(env: Env) -> String { env.storage().instance().get(&DataKey::Name).unwrap() }
    pub fn symbol(env: Env) -> String { env.storage().instance().get(&DataKey::Symbol).unwrap() }
    pub fn total_supply(env: Env) -> i128 { env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0) }
}