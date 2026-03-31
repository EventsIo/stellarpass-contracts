#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct PaymentRecord {
    pub buyer: Address,
    pub event_id: u64,
    pub currency: Symbol,
    pub amount: i128,
    pub refunded: bool,
}

#[contracttype]
pub enum DataKey {
    Payment(u64),
    PaymentCount,
}

#[contract]
pub struct PaymentContract;

#[contractimpl]
impl PaymentContract {
    /// Processes XLM or USDC payment for a ticket purchase
    pub fn process_payment(
        env: Env,
        buyer: Address,
        event_id: u64,
        token_address: Address,
        amount: i128,
        organizer: Address,
    ) -> u64 {
        buyer.require_auth();

        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&buyer, &organizer, &amount);

        let payment_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::PaymentCount)
            .unwrap_or(0)
            + 1;

        let currency = Symbol::new(&env, "XLM");

        let record = PaymentRecord {
            buyer,
            event_id,
            currency,
            amount,
            refunded: false,
        };

        env.storage()
            .instance()
            .set(&DataKey::Payment(payment_id), &record);
        env.storage()
            .instance()
            .set(&DataKey::PaymentCount, &payment_id);

        payment_id
    }

    /// Refunds attendee on event cancellation
    pub fn refund(
        env: Env,
        payment_id: u64,
        token_address: Address,
        organizer: Address,
    ) {
        organizer.require_auth();

        let mut record: PaymentRecord = env
            .storage()
            .instance()
            .get(&DataKey::Payment(payment_id))
            .expect("Payment not found");

        assert!(!record.refunded, "Already refunded");

        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&organizer, &record.buyer, &record.amount);

        record.refunded = true;
        env.storage()
            .instance()
            .set(&DataKey::Payment(payment_id), &record);
    }
}
