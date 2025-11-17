#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, Address};
use soroban_sdk::testutils::Address as _;

#[test]
fn test_initialize() {
    let env = Env::default();

    let u1 = Address::generate(&env);
    let u2 = Address::generate(&env);
    let u3 = Address::generate(&env);

    let members = vec![&env, u1.clone(), u2.clone(), u3.clone()];

    let contract_id = env.register( GroupExpenseContract, ());
    let client = GroupExpenseContractClient::new(&env, &contract_id);

    client.initialize(&members);

    let stored_members: Vec<Address> = env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .get(&DataKey::Members)
            .expect("Kullanıcı oluşturulamadı")
    });

    assert_eq!(stored_members, members);

    let stored_expenses: Vec<Expense> = env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .get(&DataKey::Expenses)
            .expect("Harcamalar oluşturulamadı")
    });

    assert_eq!(stored_expenses.is_empty(), true);
}

#[test]
fn test_add_expense() {
    let env = Env::default();

    env.mock_all_auths();

    let u1 = Address::generate(&env);
    let u2 = Address::generate(&env);
    let u3 = Address::generate(&env);

    let members = vec![&env, u1.clone(), u2.clone(), u3.clone()];

    let contract_id = env.register(GroupExpenseContract, ());
    let client = GroupExpenseContractClient::new(&env,&contract_id);

    client.initialize(&members);

    let desc = String::from_str(&env, "Market alisverisi");
    client.add_expense(&u1, &100, &desc);

    let stored_expenses: Vec<Expense> = env.as_contract(&contract_id, || {
        env.storage()
            .instance()
            .get(&DataKey::Expenses)
            .expect("Harcamalar okunmadi")
    });

    assert_eq!(stored_expenses.len(), 1);

    let expense = &stored_expenses.get(0).unwrap();
    assert_eq!(expense.amount, 100);
    assert_eq!(expense.payer, u1);
}

#[test]
fn test_get_balances() {
    let env = Env::default();
    env.mock_all_auths();

    let u1 = Address::generate(&env);
    let u2 = Address::generate(&env);
    let u3 = Address::generate(&env);
    let members = vec![&env, u1.clone(), u2.clone(), u3.clone()];

    let contract_id = env.register(GroupExpenseContract, ());
    let client = GroupExpenseContractClient::new(&env, &contract_id);

    client.initialize(&members);

    client.add_expense(&u1, &90, &String::from_str(&env,"Yemek"));

    let balances = client.get_balances();

    let b1 = balances.get(u1.clone()).unwrap();
    assert_eq!(b1, 60);

    let b2 = balances.get(u2.clone()).unwrap();
    assert_eq!(b2, -30);

    let b3 = balances.get(u3.clone()).unwrap();
    assert_eq!(b3, -30);
}