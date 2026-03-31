#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct EventData {
    pub organizer: Address,
    pub name: String,
    pub date: u64,
    pub supply: u32,
    pub price: i128,
    pub currency: Symbol,
    pub is_invite_only: bool,
    pub is_cancelled: bool,
    pub tickets_sold: u32,
}

#[contracttype]
pub enum DataKey {
    Event(u64),
    EventCount,
}

#[contract]
pub struct EventContract;

#[contractimpl]
impl EventContract {
    pub fn create_event(
        env: Env,
        organizer: Address,
        name: String,
        date: u64,
        supply: u32,
        price: i128,
        currency: Symbol,
        is_invite_only: bool,
    ) -> u64 {
        organizer.require_auth();

        let event_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::EventCount)
            .unwrap_or(0)
            + 1;

        let event = EventData {
            organizer,
            name,
            date,
            supply,
            price,
            currency,
            is_invite_only,
            is_cancelled: false,
            tickets_sold: 0,
        };

        env.storage()
            .instance()
            .set(&DataKey::Event(event_id), &event);
        env.storage()
            .instance()
            .set(&DataKey::EventCount, &event_id);

        event_id
    }

    pub fn get_event(env: Env, event_id: u64) -> EventData {
        env.storage()
            .instance()
            .get(&DataKey::Event(event_id))
            .expect("Event not found")
    }

    pub fn cancel_event(env: Env, organizer: Address, event_id: u64) {
        organizer.require_auth();
        let mut event: EventData = env
            .storage()
            .instance()
            .get(&DataKey::Event(event_id))
            .expect("Event not found");

        assert!(event.organizer == organizer, "Unauthorized");
        event.is_cancelled = true;

        env.storage()
            .instance()
            .set(&DataKey::Event(event_id), &event);
    }

    pub fn update_supply(env: Env, event_id: u64) {
        let mut event: EventData = env
            .storage()
            .instance()
            .get(&DataKey::Event(event_id))
            .expect("Event not found");

        assert!(event.tickets_sold < event.supply, "Sold out");
        event.tickets_sold += 1;

        env.storage()
            .instance()
            .set(&DataKey::Event(event_id), &event);
    }
}
