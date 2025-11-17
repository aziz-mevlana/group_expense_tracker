#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, vec, Address, Env, String, Vec, Map,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expense { // Harcama yapisi
    pub payer: Address, // Kim
    pub amount: u128, // Ne kadar
    pub description: String, // Neden
}

#[contracttype]
enum DataKey { // Depolama anahtari
    Members, // Uye listesi
    Expenses, // Harcama listesi
}

#[contract]
pub struct GroupExpenseContract;

#[contractimpl]
impl GroupExpenseContract {
    
    pub fn initialize(env: Env, members: Vec<Address>) { // Kurulum
        if env.storage().instance().has(&DataKey::Members) {  // Kontrol
            panic!("Sözleşme zaten başlatıldı");
        }

        env.storage().instance().set(&DataKey::Members, &members); // Uye kaydi

        let expenses: Vec<Expense> = vec![&env]; // Bos liste
        env.storage().instance().set(&DataKey::Expenses, &expenses); // Harcama listesi
    }

    pub fn add_expense(env: Env, payer: Address, amount: u128, description: String) { // Gruba harcama ekleme
        payer.require_auth(); // Guvenlik

        let members: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Members)
            .expect("Grup bulunamadı");

        if !members.contains(&payer) {
            panic!("Grup üyesi değil")
        }

        let mut expenses: Vec<Expense> = env
            .storage()
            .instance()
            .get(&DataKey::Expenses)
            .expect("Harcamalar bulunamadı");

        let new_expense = Expense {
            payer: payer,
            amount: amount,
            description: description,
        };
            
        expenses.push_back(new_expense);

        env.storage().instance().set(&DataKey::Expenses, &expenses);
    }

    pub fn get_balances(env: Env) -> Map<Address, i128> {
        let members: Vec<Address> = env
            .storage()
            .instance()
            .get(&DataKey::Members)
            .expect("Grup yok");

        let expenses: Vec<Expense> = env
            .storage()
            .instance()
            .get(&DataKey::Expenses)
            .unwrap_or(vec![&env]);

        let mut total_spent: u128 = 0;
        for expense in expenses.iter() {
            total_spent += expense.amount;
        }

        let member_count = members.len() as u128;
        if member_count == 0 {
            return Map::new(&env);
        }
        let share_per_person = total_spent / member_count;

        let mut balances: Map<Address, i128> = Map::new(&env);

        for member in members.iter() {
            let mut paid_by_member: u128 = 0;

            for expense in expenses.iter() {
                if expense.payer == member {
                    paid_by_member += expense.amount;
                }
            }

            let balance = (paid_by_member as i128) - (share_per_person as i128);

            balances.set(member, balance);
        }
        return balances;
    }
}

mod test;