#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Adder {
    #[proxy]
    fn self_proxy(&self) -> self::Proxy<Self::Api>;

	#[init]
	fn init(&self, from_token: TokenIdentifier, fee_percent: BigUint) -> SCResult<()>  {
		let caller = self.blockchain().get_caller();
		require!(!caller.is_zero(), "invalid caller");
		require!(&fee_percent > &0, "no fee");

		self.fee_percent().set(&fee_percent);
		return self.add_from_token(from_token);
	}

	#[payable("*")]
	#[endpoint(sendSc)]
	fn send_sc(
		&self,
		#[payment_token] token_id: TokenIdentifier,
		#[payment_nonce] nonce: u64,
		#[payment_amount] amount: BigUint
	) -> SCResult<BigUint>  {
		require!(&amount > &0, "invalid amount");

		self.send().direct(
			&self.blockchain().get_sc_address(),
			&token_id,
			nonce,
			&amount,
			&[]);
		Ok(amount)
	}

	#[payable("*")]
	#[endpoint(sendMex)]
	fn send_mex(
		&self,
        caller: &ManagedAddress,
		amount: BigUint
	) -> SCResult<()>  {
		require!(!&caller.is_zero(), "invalid caller");

		let fee_percent = self.fee_percent().get();
		let amount_after_fee = &amount * &fee_percent / 100_u32;
		let sc_balance = self.blockchain().get_sc_balance(&self.to_token().get(), 0);

		require!(&amount_after_fee < &amount, "incorrect fee");
		require!(&amount_after_fee <= &sc_balance, "no liquidity");
		self.send().direct(
			caller,
			&self.to_token().get(),
			0,
			&amount_after_fee,
			&[]);
		Ok(())
	}



	#[payable("*")]
	#[endpoint(swap)]
	fn swap(
		&self,
		#[payment_token] token_id: TokenIdentifier,
		#[payment_nonce] nonce: u64,
		#[payment_amount] amount: BigUint
	) -> SCResult<AsyncCall>   {
		require!(amount >= 1, "minimum amount of 1 token");
		require!(!self.blockchain().get_caller().is_zero(), "invalid caller");
		require!(self.from_tokens().contains(&token_id), "token not supported");

		Ok(self.self_proxy()
				.contract(self.blockchain().get_sc_address())
				.send_sc(token_id, nonce, amount)
				.async_call()
				.with_callback(
					self.callbacks()
				    .send_sc_callback(
						&self.blockchain().get_caller())))
	}

    // callbacks

    #[callback]
    fn send_sc_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<BigUint>,
        caller: &ManagedAddress
    )  {
        match result {
            ManagedAsyncCallResult::Ok(amount) => {

				self.self_proxy()
				.contract(self.blockchain().get_sc_address())
				.send_mex(caller, amount)
				.async_call()
				.with_callback(
					self.callbacks()
				    .send_mex_callback(
						&caller));

			self.last_error_message().clear();
            },
            ManagedAsyncCallResult::Err(message) => {
				self.last_error_message().set(&message.err_msg);
				//sc_error!("error sending LKMEX")
            }
        }
    }

	#[callback]
    fn send_mex_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<BigUint>,
        caller: &ManagedAddress
    )  {
        match result {
            ManagedAsyncCallResult::Ok(amount) => {
			self.last_error_message().clear();
            },
            ManagedAsyncCallResult::Err(message) => {
                //TODO: return the LKMEX

				self.last_error_message().set(&message.err_msg);
            }
        }
    }


	#[only_owner]
	#[endpoint(addFromToken)]
	fn add_from_token(&self, asset: TokenIdentifier) -> SCResult<()>{
		require!(asset.is_valid_esdt_identifier(), "Invalid ESDT");
		self.from_tokens().insert(asset);
		Ok(())
	}

	#[only_owner]
	#[endpoint(setToToken)]
	fn add_to_token(&self, asset: TokenIdentifier) -> SCResult<()>{
		require!(asset.is_valid_esdt_identifier(), "Invalid ESDT");
		self.to_token().set(&asset);
		Ok(())
	}

	// STORAGE
	#[view(getFee)]
	#[storage_mapper("fee_percent")]
    fn fee_percent(&self) -> SingleValueMapper<BigUint>;

	#[storage_mapper("to_token")]
    fn to_token(&self) -> SingleValueMapper<TokenIdentifier>;

	#[view(getFromTokens)]
	#[storage_mapper("from_tokens")]
	fn from_tokens(&self) -> SetMapper<TokenIdentifier>;

	#[view(getLiquidityBalance)]
	fn get_balance(&self) -> BigUint{
		self.blockchain().get_sc_balance(&self.to_token().get(), 0)
	}

	#[view(getLastError)]
	#[storage_mapper("lastErrorMessage")]
    fn last_error_message(&self) -> SingleValueMapper<ManagedBuffer>;
}
