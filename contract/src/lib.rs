use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen, AccountId, Promise};

// Login Info structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Login {
    message: String,
    records: LookupMap<AccountId, String>,
}

// Routes Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Routes {
    start: String,
    end: String,
    fare: u128,
}

// Payment History Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PaymentHistory {
    account: AccountId,
    fare: u128,
}

// Vehicle Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Vehicle {
    route: Routes,
    account: AccountId,
    reg_number: String,
    payments: Vector<PaymentHistory>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct App {
    vehicles: Vec<Vehicle>,
}

impl Default for App {
    fn default() -> Self {
        App {
            vehicles: vec![], //Vector::new(b"r".to_vec())
        }
    }
}

#[near_bindgen]
impl App {
    fn create_vehicle(
        &mut self,
        reg_number: String,
        route_start: String,
        route_end: String,
        fare: u128,
    ) {
        let rte = Routes {
            start: route_start,
            end: route_end,
            fare: fare,
        };
        let vh = Vehicle {
            route: rte,
            account: env::current_account_id(),
            payments: Vector::new(b'd'),
            reg_number: reg_number,
        };

        //  Add created vehicle to list of vehicles
        self.vehicles.push(vh);
    }
    fn pay(&mut self, reg_number: String) -> String {
        let mut vehicle_item: Option<& Vehicle> = None;
        let mut vehicle_index: Option<usize> = None;
        for (index, elem) in self.vehicles.iter().enumerate() {
            if elem.reg_number == reg_number {
                vehicle_index = Some(index);
            }
        }

        match vehicle_index {
            Some(x) => {
                vehicle_item = self.vehicles.get(x);
            }
            None => env::log_str("vehicle index not found"),
        }

        match vehicle_item {
            Some(item) => {
                // implement NEAR payment
                if env::account_balance() >= item.route.fare {
                    Promise::new(env::current_account_id()).transfer(item.route.fare);

                    let py = PaymentHistory {
                        fare: item.route.fare,
                        account: env::current_account_id(),
                    };

                    for elem in self.vehicles.iter_mut() {
                        if elem.reg_number == reg_number {
                            elem.payments.push(&py);
                        }
                    }
                    return "OKAY!".to_string();
                } else {
                    env::log_str("You have insufficient balance!");
                    return "Error!".to_string();
                }
            }
            None => {
                env::log_str("Vehicle is unknown!");
                return "Error".to_string();
            }
        }
    }
}

// Define the default, which automatically initializes the contract
impl Default for Login {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            records: LookupMap::new(b"r".to_vec()),
        }
    }
}

// Implement the Login structure
#[near_bindgen]
impl Login {
    pub fn confirm_login(&mut self, message: String) {
        let account_id = env::signer_account_id(); //gets ID of account owner
        self.records.insert(&account_id, &message);
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_message(&self) -> String {
        return self.message.clone();
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    use near_sdk::test_utils::{VMContextBuilder};
    use near_sdk::{AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn create_vehicle() {
        let mut app = App::default();
        app.create_vehicle(
            "KCP 247L".to_string(),
            "Nairobi".to_string(),
            "Thika".to_string(),
            150,
        );
        assert_eq!(app.vehicles.len(), 1);
    }

    #[test]
    fn test_pay() {
        let mut app = App::default();
        app.create_vehicle(
            "KCP 247L".to_string(),
            "Nairobi".to_string(),
            "Thika".to_string(),
            150,
        );
        assert_eq!(app.vehicles.len(), 1);
        let user: AccountId = AccountId::new_unchecked("sample.testnet".to_string());
        let mut _context: VMContextBuilder = get_context(user.clone());
        _context.attached_deposit(1222);
        let res = app.pay("KCP 247L".to_string());
        assert_eq!(res, "OKAY!".to_string());
    }
}

