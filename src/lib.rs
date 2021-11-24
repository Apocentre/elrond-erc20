#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract]
pub trait ERC20 {
  // Storage
  #[view(totalSupply)]
  #[storage_mapper("total_supply")]
  fn total_supply(&self) -> SingleValueMapper<BigUint>;

  #[view(balanceOf)]
  #[storage_mapper("balance_of")]
  fn balance_of(&self, account: &ManagedAddress) -> SingleValueMapper<BigUint>;

  #[view(allowance)]
  #[storage_mapper("allowance")]
  fn allowance(&self, owner: &ManagedAddress, spender: &ManagedAddress) -> SingleValueMapper<BigUint>;
  
  // Events
  #[event("transfer")]
  fn transfer_event(
    &self,
    #[indexed] sender: &ManagedAddress,
    #[indexed] recipient: &ManagedAddress,
    amount: &BigUint
  );

  #[event("approve")]
  fn approve_event(
    &self,
    #[indexed] owner: &ManagedAddress,
    #[indexed] spender: &ManagedAddress,
    amount: &BigUint,
  );

  #[init]
  fn init(&self, total_supply: &BigUint) {
    let creator = self.blockchain().get_caller();
    
    // set total supply
    self.total_supply().set(total_supply);

    // total supply is pre-minted and sent to the deployer
    self.balance_of(&creator).update(|balance| *balance += total_supply);
  }

  fn exec_transfer(
    &self,
    sender: ManagedAddress,
    recipient: ManagedAddress,
    amount: BigUint
  ) -> SCResult<()> {
    // check sender's balance
    self.balance_of(&sender).update(|balance| {
      require!(amount <= *balance, &b"insufficient funds"[..]);
      *balance -= &amount;

      Ok(())
    })?;

    // increase recipients amount
    self.balance_of(&recipient).update(|balance| *balance += &amount);

    self.transfer_event(&sender, &recipient, &amount);

    Ok(())
  }

  // API
  #[endpoint]
  fn transfer(
    &self,
    to: ManagedAddress,
    amount: BigUint
  ) -> SCResult<()> {
    let caller = self.blockchain().get_caller();
    self.exec_transfer(caller, to, amount)
  }

  #[endpoint]
  fn transfer_from(
    &self,
    from: ManagedAddress,
    to: ManagedAddress,
    amount: BigUint
  ) -> SCResult<()> {
    let caller = self.blockchain().get_caller();

    // check allowance
    self.allowance(&from, &caller).update(|allowance| {
      require!(amount <= *allowance, &b"Insufficient allowance"[..]);
      *allowance += &amount;

      Ok(())
    })?;

    self.exec_transfer(from, to, amount)
  }

  #[endpoint]
  fn approve(
    &self,
    spender: ManagedAddress,
    amount: BigUint
  ) -> SCResult<()> {
    let caller = self.blockchain().get_caller();

    // user should have enough balance
    let balance = self.balance_of(&caller).get();
    require!(amount <= balance, &b"approval amount exceeds balance"[..]);

    self.allowance(&caller, &spender).set(&amount);

    self.approve_event(&caller, &spender, &amount);

    Ok(())
  }
}
