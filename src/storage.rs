multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// CUSTOM FORMAT
#[derive(PartialEq, TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Service {
    pub price: u16, // Price stored as a 2 decimal value so 3.25$ will be stored as 325
    pub periodicity: u64, // Periodicity stored in seconds
}

// STORAGE

#[multiversx_sc::module]
pub trait StorageModule {
    // Store Netflix Smart Contract addresss
    #[view(getNetflix)]
    #[storage_mapper("netflix")]
    fn netflix(&self) -> SingleValueMapper<ManagedAddress>;

    // Number of Tokens mapper
    #[view(getTokensCount)]
    #[storage_mapper("tokens_count")]
    fn tokens_count(&self) -> SingleValueMapper<usize>;

    // Allowed tokens
    #[view(getTokens)]
    #[storage_mapper("tokens")]
    fn tokens(&self, id: &usize) -> SingleValueMapper<TokenIdentifier>;

    // Number of Services mapper
    #[view(getServicesCount)]
    #[storage_mapper("services_count")]
    fn services_count(&self) -> SingleValueMapper<usize>;

    // Id To Services mapper
    #[view(getServices)]
    #[storage_mapper("services")]
    fn services(&self, id: &usize) -> SingleValueMapper<Service>;

    // Address to Balance mapper
    #[view(getBalance)]
    #[storage_mapper("balance")]
    fn balance(&self, address: &ManagedAddress) -> MapMapper<usize, BigUint>;
}
