#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract]
pub trait ERC20 {
  #[view(totalSupply)]
  #[storage_mapper("total_supply")]
  fn total_supply(&self) -> SingleValueMapper<BigUint>;

  #[view(balanceOf)]
  #[storage_mapper("balance_of")]
  fn balance_of(&self, account: &ManagedAddress) -> SingleValueMapper<BigUint>;

  #[view(allowance)]
  #[storage_mapper("allowance")]
  fn allowance(&self, owner: &ManagedAddress, spender: &ManagedAddress) -> SingleValueMapper<BigUint>;
  
  #[init]
  fn init(&self, total_supply: &BigUint) {
    let creator = self.blockchain().get_caller();
    
    // set total supply
    self.total_supply().set(total_supply);

    // total supply is pre-minted and sent to the deployer
    self.balance_of(&creator).update(|balance| *balance += total_supply);
  }
}
