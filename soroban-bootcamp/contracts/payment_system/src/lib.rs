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

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_initialize_and_mint() {
        let env = Env::default();
        env.mock_all_auths(); 
        let contract_id = env.register_contract(None, PaymentToken);
        let client = PaymentTokenClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        client.initialize(&admin, &String::from_str(&env, "Token"), &String::from_str(&env, "TKN"));
        
        client.mint(&user, &1000);
        assert_eq!(client.balance(&user), 1000);
    }

    #[test]
    fn test_successful_transfer() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, PaymentToken);
        let client = PaymentTokenClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &String::from_str(&env, "Token"), &String::from_str(&env, "TKN"));
        client.mint(&user1, &500);
        client.transfer(&user1, &user2, &200);

        assert_eq!(client.balance(&user1), 300);
        assert_eq!(client.balance(&user2), 200);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")] 
    fn test_transfer_insufficient_balance() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, PaymentToken);
        let client = PaymentTokenClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &String::from_str(&env, "Token"), &String::from_str(&env, "TKN"));
        client.mint(&user1, &100);
        client.transfer(&user1, &user2, &150); // Lỗi vì chuyển 150 nhưng chỉ có 100
    }
}