#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct TicketData {
    pub event_id: u64,
    pub owner: Address,
    pub tier: Symbol,
    pub is_checked_in: bool,
    pub is_revoked: bool,
}

#[contracttype]
pub enum DataKey {
    Ticket(u64),
    TicketCount,
}

#[contract]
pub struct TicketContract;

#[contractimpl]
impl TicketContract {
    /// Mints a soulbound (non-transferable) ticket to an attendee wallet
    pub fn mint_ticket(
        env: Env,
        event_id: u64,
        attendee: Address,
        tier: Symbol,
    ) -> u64 {
        let ticket_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::TicketCount)
            .unwrap_or(0)
            + 1;

        let ticket = TicketData {
            event_id,
            owner: attendee,
            tier,
            is_checked_in: false,
            is_revoked: false,
        };

        env.storage()
            .instance()
            .set(&DataKey::Ticket(ticket_id), &ticket);
        env.storage()
            .instance()
            .set(&DataKey::TicketCount, &ticket_id);

        ticket_id
    }

    pub fn verify_ticket(env: Env, ticket_id: u64, wallet: Address) -> bool {
        let ticket: TicketData = env
            .storage()
            .instance()
            .get(&DataKey::Ticket(ticket_id))
            .expect("Ticket not found");

        ticket.owner == wallet && !ticket.is_revoked
    }

    pub fn check_in(env: Env, ticket_id: u64, organizer: Address) {
        organizer.require_auth();
        let mut ticket: TicketData = env
            .storage()
            .instance()
            .get(&DataKey::Ticket(ticket_id))
            .expect("Ticket not found");

        assert!(!ticket.is_checked_in, "Already checked in");
        assert!(!ticket.is_revoked, "Ticket revoked");

        ticket.is_checked_in = true;
        env.storage()
            .instance()
            .set(&DataKey::Ticket(ticket_id), &ticket);
    }

    pub fn revoke_ticket(env: Env, ticket_id: u64, organizer: Address) {
        organizer.require_auth();
        let mut ticket: TicketData = env
            .storage()
            .instance()
            .get(&DataKey::Ticket(ticket_id))
            .expect("Ticket not found");

        ticket.is_revoked = true;
        env.storage()
            .instance()
            .set(&DataKey::Ticket(ticket_id), &ticket);
    }
}
