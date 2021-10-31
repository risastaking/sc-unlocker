#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm_derive::contract]
pub trait Adder {
	#[init]
	fn init(&self, asset: TokenIdentifier) -> SCResult<()>  {
		let caller = self.blockchain().get_caller();
		require!(!caller.is_zero(), "address is zero");

		return self.add_token_allowed(asset);
	}

	#[payable("*")]
	#[endpoint]
	fn swap(
        &self,
        #[payment_token] from_token: TokenIdentifier,
        #[payment_amount] amount: BigUint,
	) -> SCResult<()> {
		require!(amount > 0, "amount must be greater than 0");

        require!(
            from_token.is_valid_esdt_identifier(),
            "Invalid ESDT token"
        );

        require!(self.tokens_allowed().contains(&from_token), "token not supported");

		// TODO
        // pay the MEX to caller
		// self.send().direct_esdt_via_transf_exec(&address, &esdt_token_name, &amount, &[]);

		Ok(())
	}

    #[only_owner]
    #[endpoint(addTokenAllowed)]
	fn add_token_allowed(&self, asset: TokenIdentifier) -> SCResult<()>{
		require!(asset.is_valid_esdt_identifier(), "Invalid ESDT");
		self.tokens_allowed().insert(asset);
		Ok(())
	}

    #[view(getTokensAllowed)]
    #[storage_mapper("tokens_allowed")]
    fn tokens_allowed(&self) -> SetMapper<TokenIdentifier>;
}
