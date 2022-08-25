use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};
use near_sdk::collections::Vector;
use near_sdk::collections::LookupMap;


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
    fare: u128
}

// Payment History Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PaymentHistory {
    account: AccountId,
    fare: u128
}

// Vehicle Structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Vehicle {
    route:Routes,
    account: AccountId,
    reg_number: String,
    payments: Vector<PaymentHistory>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct App {   
    vehicles: Vec<Vehicle>,
}

impl Default for App{
    fn default() -> Self {
        App {
             vehicles: vec![], //Vector::new(b"r".to_vec())
        }
    }
}

#[near_bindgen]
impl App{   
    fn create_vehicle(
        &mut self, 
        reg_number: String, 
        route_start:String, 
        route_end:String, 
        fare:u128)
        {
        let rte = Routes{
            start: route_start,
            end:route_end,
            fare: fare,
        };
        let vh = Vehicle { 
            route: rte,
            account: env::current_account_id(),
            payments: Vector::new(b'd'),
            reg_number: reg_number
         };

        //  Add created vehicle to list of vehicles
         self.vehicles.push(vh);

    } 
    fn pay(&mut self ,reg_number: String ) -> String{
        let mut vehicle_item: Option<&mut Vehicle> = None;

        // for elem in self.vehicles.iter(){
        //     if elem.reg_number = reg_number{
        //         vehicle_item = Some(elem)
        //     }
        // }

        match vehicle_item{
            Some(item) => {
            
                // implement NEAR payment
                if env::account_balance() >= item.route.fare{
                    
                    Promise::new(env::current_account_id()).transfer(item.route.fare);

                    let py = PaymentHistory{
                        fare: item.route.fare,
                        account: env::current_account_id()
                    };

                    for elem in self.vehicles.iter_mut(){
                        if elem.reg_number == reg_number{
                            elem.payments.push(&py);
                        }
                    }
                    return "OKAY!".to_string();
                }else{
                    env::log_str("You have insufficient balance!");
                    return "Error!".to_string();

                }
// let cost = item.route.amount;

                // implement NEAR payment


                // let curr = env::account_balance();

            }
            None=>{
                env::log_str("Vehicle is unknown!");
                return "Error".to_string();
            }
        }
    }
}


// Define the default, which automatically initializes the contract
impl Default for Login{
    fn default() -> Self{
        Self{
            message: DEFAULT_MESSAGE.to_string(),
            records: LookupMap::new(b"r".to_vec()),
        }
    }
}

// Implement the Login structure
#[near_bindgen]
impl Login {
    pub fn confirm_login(&mut self, message: String) {
        let account_id = env::signer_account_id();      //gets ID of account owner
        self.records.insert(&account_id, &message);
    }

    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_message(&self) -> String {
        return self.message.clone();
    }
    // TODO
}

// Write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_created_vehicle() {

        
    }

    #[test]
    fn test_payment(){
        
    }
}