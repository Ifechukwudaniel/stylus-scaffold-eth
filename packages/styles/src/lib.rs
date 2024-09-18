// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
use alloy_sol_types::sol;
use stylus_sdk::{
    call::transfer_eth, console, evm, msg , contract::balance
};
use stylus_sdk::prelude::*;
use alloy_primitives::{Address, U256};

sol! {
    event GreetingChange(
		address indexed greetingSetter,
		string newGreeting,
		bool premium,
		uint256 value
	);

    error NotOwnerError();
}


#[derive(SolidityError)]
pub enum YourContractError {
     NotOwnerError(NotOwnerError)
}

// Define some persistent storage using the Solidity ABI.
// `YourContract` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct YourContract {
        address owner;
        string  greeting;
        bool premium;
        uint256  total_counter ;
        mapping(address => uint256) user_greeting_counter;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl YourContract {
   
   #[payable]
    pub fn set_greeting(&mut self, new_greeting: String) {
        // Set the greeting
        self.greeting.set_str(&new_greeting);

        // Increment the total counter
        let new_total_count = *self.total_counter + U256::from(1);
        self.total_counter.set(new_total_count);

        // Increment the user's greeting count
        let address_greetings_count = self.user_greeting_counter.get(msg::sender());
        let updated_count = address_greetings_count + U256::from(1);
        self.user_greeting_counter.insert(msg::sender(), updated_count);

        // Set the premium status based on the msg value in one step
        self.premium.set(msg::value() > U256::from(0));

        // Emit the event
        evm::log(GreetingChange {
            greetingSetter: msg::sender(),
            newGreeting: new_greeting, 
            premium: *self.premium, 
            value: msg::value(),
        });
    
    }

    pub fn withdraw (&mut self) -> Result<() ,YourContractError>{
        if  *self.owner == msg::sender() { 
            let _ = transfer_eth(*self.owner,balance());
            Ok(())
        }
        else { 
           return  Err(YourContractError::NotOwnerError(NotOwnerError{}));
        }
    }

    pub fn greeting(&self) -> String {
        self.greeting.get_string()
    }

    pub fn owner(&self) -> Address {
        self.owner.get()
    }     
    
    pub fn  premium(&self) -> bool {
        self.premium.get()
    }

    pub fn total_counter(&self) -> U256 {
        self.total_counter.get()
    }

    pub fn user_greeting_counter(&self, _user: Address) -> U256 {
        self.user_greeting_counter.get(_user)
    }

}
