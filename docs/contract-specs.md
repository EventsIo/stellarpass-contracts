# Contract Specifications

## Event Contract

### `create_event`
Creates a new event on-chain.

**Parameters**
| Param | Type | Description |
|---|---|---|
| `organizer` | `Address` | Wallet address of the event organizer |
| `name` | `String` | Event name |
| `date` | `u64` | Unix timestamp of event date |
| `supply` | `u32` | Max number of tickets available |
| `price` | `i128` | Ticket price in stroops (XLM) or smallest USDC unit |
| `currency` | `Symbol` | `XLM` or `USDC` |
| `is_invite_only` | `bool` | Whether event requires an invite code |

**Returns** `event_id: u64`

---

### `get_event`
Fetches event data by ID.

**Parameters**
| Param | Type | Description |
|---|---|---|
| `event_id` | `u64` | The event ID |

**Returns** `EventData`

---

### `cancel_event`
Cancels an event. Only callable by the organizer.

**Parameters**
| Param | Type | Description |
|---|---|---|
| `organizer` | `Address` | Must match event organizer |
| `event_id` | `u64` | The event ID |

---

### `update_supply`
Increments tickets_sold counter. Called internally after a successful mint.

---

## Ticket Contract

### `mint_ticket`
Mints a soulbound (non-transferable) NFT ticket to an attendee wallet.

**Parameters**
| Param | Type | Description |
|---|---|---|
| `event_id` | `u64` | Associated event ID |
| `attendee` | `Address` | Attendee wallet address |
| `tier` | `Symbol` | Ticket tier name (e.g. `VIP`, `GENERAL`) |

**Returns** `ticket_id: u64`

---

### `verify_ticket`
Verifies that a wallet owns a valid, non-revoked ticket.

**Parameters**
| Param | Type | Description |
|---|---|---|
| `ticket_id` | `u64` | The ticket ID |
| `wallet` | `Address` | Wallet to verify against |

**Returns** `bool`

---

### `check_in`
Marks a ticket as checked-in. Prevents duplicate entry.

**Parameters**
| Param | Type | Description |
|---|---|---|
| `ticket_id` | `u64` | The ticket ID |
| `organizer` | `Address` | Must be the event organizer |

---

### `revoke_ticket`
Revokes a ticket (e.g. fraud case). Only callable by organizer.

**Parameters**
| Param | Type | Description |
|---|---|---|
| `ticket_id` | `u64` | The ticket ID |
| `organizer` | `Address` | Must be the event organizer |

---

## Payment Contract

### `process_payment`
Transfers XLM or USDC from buyer to organizer and records the payment.

**Parameters**
| Param | Type | Description |
|---|---|---|
| `buyer` | `Address` | Attendee wallet |
| `event_id` | `u64` | Associated event ID |
| `token_address` | `Address` | XLM or USDC token contract address |
| `amount` | `i128` | Amount in stroops or smallest USDC unit |
| `organizer` | `Address` | Organizer wallet to receive payment |

**Returns** `payment_id: u64`

---

### `refund`
Refunds the buyer. Only callable by organizer (e.g. on event cancellation).

**Parameters**
| Param | Type | Description |
|---|---|---|
| `payment_id` | `u64` | The payment record ID |
| `token_address` | `Address` | Token contract address |
| `organizer` | `Address` | Must be the organizer |
