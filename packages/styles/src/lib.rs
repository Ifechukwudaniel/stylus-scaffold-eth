// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

// Use an efficient WASM allocator for memory management.
// #[global_allocator]
// static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;
use alloy_sol_types::sol;
use stylus_sdk::{
    call::transfer_eth, console, evm, msg , contract::balance
};
use stylus_sdk::prelude::*;
use alloy_primitives::private;
use alloy_primitives::U256;

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
        uint256  totalCounter ;
        mapping(address => uint256) userGreetingCounter;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl YourContract {
   
   #[payable]
    pub fn setGreeting(&mut self, _newGreating: String) -> (){
       self.greeting.set_str(_newGreating.clone());
       self.totalCounter.set(*self.totalCounter + U256::from(1));
       let mut address_greetings_count = self.userGreetingCounter.setter(msg::sender());
       let count =  address_greetings_count.get();
       address_greetings_count.set(count + U256::from(1));
       if msg::value() > U256::from(0){
           self.premium.set(true);
       }
       else {
           self.premium.set(false);
       }
     
       evm::log(GreetingChange {
           greetingSetter: msg::sender(),
           newGreeting: _newGreating, 
           premium:*self.premium, 
           value:msg::value()
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

}
