#![no_std]

// Import necessary modules and macros from the MultiversX framework
elrond_wasm::imports!();
elrond_wasm::derive_imports!();

/// Define a smart contract with the SnowTokenIssuer trait
#[elrond_wasm::contract]
pub trait SnowTokenIssuer {
    /// The `issueTokenSnow` endpoint allows users to issue a new token called "SNOW"
    /// The endpoint is payable in EGLD, meaning it requires an EGLD payment to be executed.
    #[payable("EGLD")]
    #[endpoint(issueTokenSnow)]
    fn issue_token_snow(&self) {
        // Get the address of the caller who invoked this endpoint
        let caller = self.blockchain().get_caller();

        // Retrieve the amount of EGLD sent with the call
        let payment = self.call_value().egld_value();
        
        // Check if the payment is greater than zero. If not, the transaction fails with an error message.
        require!(payment > BigUint::zero(), "Payment is required");

        // Define the properties of the new token to be issued
        let token_name = "Snow Token".to_string();   // Name of the token
        let token_ticker = "SNOW".to_string();       // Ticker symbol of the token
        let token_decimals = 8u32;                   // Number of decimal places for the token

        // Mint a new ESDT token with the specified properties
        // Note: In MultiversX, `esdt_local_mint` can be used to mint tokens for a specific address.
        // Here, we are assuming a function exists to create a new token.
        self.send().esdt_local_mint(
            &caller,                            // Address to mint the token for (the caller)
            &token_ticker.as_bytes(),           // Ticker symbol for the token
            &BigUint::from(1u32),               // Amount of tokens to mint (1 in this example)
            token_decimals,                     // Number of decimals for the token
        );

        // Mint additional tokens for the caller
        self.send().esdt_local_mint(
            &caller,                            // Address to mint the token for (the caller)
            &token_ticker.as_bytes(),           // Ticker symbol for the token
            &BigUint::from(1000u32),            // Amount of tokens to mint (1000 SNOW tokens in this example)
            token_decimals,                     // Number of decimals for the token
        );
    }
}