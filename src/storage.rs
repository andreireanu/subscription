multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// CUSTOM FORMAT
#[derive(PartialEq, TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Service<M: ManagedTypeApi> {
    pub price: BigUint<M>, // Price stored as a 2 decimal value so 3.25$ will be stored as 325
    pub periodicity: u64,  // Periodicity stored in seconds
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

    // Allowed tokens LP address
    // Allowed tokens LP address
    #[view(getLPAddress)]
    #[storage_mapper("lp_address")]
    fn lp_address(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;

    // Token Ids
    #[view(getIds)]
    #[storage_mapper("id")]
    fn id(&self, token: &TokenIdentifier) -> SingleValueMapper<usize>;

    // Number of Services mapper
    #[view(getServicesCount)]
    #[storage_mapper("services_count")]
    fn services_count(&self) -> SingleValueMapper<usize>;

    // Id To Services mapper
    #[view(getServices)]
    #[storage_mapper("services")]
    fn services(&self, id: &usize) -> SingleValueMapper<Service<Self::Api>>;

    // Address to Balance mapper
    #[view(getBalance)]
    #[storage_mapper("balance")]
    fn balance(&self, address: &ManagedAddress) -> MapMapper<usize, BigUint>;

    // Service Id to Address subscription details mapper
    // Store (address, datetime for next payment) in map mapper
    #[view(getSubscription)]
    #[storage_mapper("subscription")]
    fn subscription(&self, id: &usize) -> MapMapper<ManagedAddress, u64>;

    // Safe Price view address
    #[view(getSafePriceView)]
    #[storage_mapper("safe_price_view")]
    fn safe_price_view(&self) -> SingleValueMapper<ManagedAddress>;

    // DEV TEMP STORAGE
    #[view(getLastPaymentVec)]
    #[storage_mapper("last_payment_vec")]
    fn last_payment_vec(&self) -> UnorderedSetMapper<EsdtTokenPayment>;
}
