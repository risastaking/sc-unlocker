#![no_std]

elrond_wasm::imports!();
const PERCENTAGE_TOTAL: u32 = 10_000; // 100%

#[elrond_wasm::contract]
pub trait Adder {
    #[proxy]
    fn self_proxy(&self) -> self::Proxy<Self::Api>;

    #[init]
    fn init(
        &self,
        from_token: TokenIdentifier,
        to_token: TokenIdentifier,
        fee_percent: u32,
    ) -> SCResult<()> {
        let mut res = self.try_set_fee_percentage(fee_percent);
        if (res.is_err()) {
            return res;
        }

        res = self.add_from_token(from_token);
        if (res.is_err()) {
            return res;
        }

        self.add_to_token(to_token)
    }

    #[payable("*")]
    #[endpoint(swap)]
    fn swap(
        &self,
        #[payment_token] token_id: TokenIdentifier,
        #[payment_amount] amount: BigUint,
    ) -> SCResult<()> {
        require!(amount >= PERCENTAGE_TOTAL, "amount too small");
        require!(!self.blockchain().get_caller().is_zero(), "invalid caller");
        require!(
            self.from_tokens().contains(&token_id),
            "token not supported"
        );

        let fee_percent = self.fee_percent().get();

        require!(&fee_percent > &0, "zero fee");
        let fee = self.calculate_percentage(&amount, &fee_percent);
        let amount_after_fee = &amount - &fee;
        let sc_balance = self.get_balance();

        require!(&amount_after_fee < &amount, "incorrect fee");
        require!(&amount_after_fee <= &sc_balance, "no liquidity");
        require!(&amount_after_fee > &0, "nothing to send");
        self.send().direct(
            &self.blockchain().get_caller(),
            &self.to_token().get(),
            0,
            &amount_after_fee,
            &[],
        );
        Ok(())
    }

    fn calculate_percentage(&self, total_amount: &BigUint, percentage: &BigUint) -> BigUint {
        total_amount * percentage / PERCENTAGE_TOTAL
    }

    #[only_owner]
    #[endpoint(addFromToken)]
    fn add_from_token(&self, asset: TokenIdentifier) -> SCResult<()> {
        require!(asset.is_valid_esdt_identifier(), "Invalid ESDT");
        self.from_tokens().insert(asset);
        Ok(())
    }

    #[only_owner]
    #[endpoint(setToToken)]
    fn add_to_token(&self, asset: TokenIdentifier) -> SCResult<()> {
        require!(asset.is_valid_esdt_identifier(), "Invalid ESDT");
        self.to_token().set(&asset);
        Ok(())
    }

    #[only_owner]
    #[endpoint(setFee)]
    fn try_set_fee_percentage(&self, new_fee_percentage: u32) -> SCResult<()> {
        require!(
            new_fee_percentage > 0 && new_fee_percentage < PERCENTAGE_TOTAL,
            "Invalid percentage value, should be between 0 and 10,000"
        );

        self.fee_percent().set(&BigUint::from(new_fee_percentage));

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
    fn get_balance(&self) -> BigUint {
        self.blockchain().get_sc_balance(&self.to_token().get(), 0)
    }
}
