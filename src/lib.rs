#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;
use storage::Service;

mod callee_proxy {
    multiversx_sc::imports!();
    use crate::storage::Service;

    #[multiversx_sc::proxy]
    pub trait CalleeContract {
        #[view(getTokensCount)]
        fn tokens_count(&self) -> SingleValueMapper<usize>;

        #[endpoint(getTokens)]
        fn tokens(&self, id: &usize) -> SingleValueMapper<TokenIdentifier>;

        #[view(getServicesCount)]
        fn services_count(&self) -> SingleValueMapper<usize>;

        #[view(getServices)]
        fn services(&self, id: &usize) -> SingleValueMapper<Service>;
    }
}

#[multiversx_sc::contract]
pub trait SubscriptionContract: crate::storage::StorageModule {
    #[proxy]
    fn contract_proxy(&self, sc_address: ManagedAddress) -> callee_proxy::Proxy<Self::Api>;

    #[init]
    fn init(&self, netflix_address: ManagedAddress) {
        self.netflix().set(netflix_address);
        let tokens_count: usize = self
            .contract_proxy(self.netflix().get())
            .tokens_count()
            .execute_on_dest_context();
        self.tokens_count().set(tokens_count);
        for idx in 0..tokens_count {
            let token: TokenIdentifier = self
                .contract_proxy(self.netflix().get())
                .tokens(idx)
                .execute_on_dest_context();
            self.tokens(&idx).set(token);
        }
        let services_count: usize = self
            .contract_proxy(self.netflix().get())
            .services_count()
            .execute_on_dest_context();
        self.services_count().set(services_count);
        for idx in 0..services_count {
            let service: Service = self
                .contract_proxy(self.netflix().get())
                .services(idx)
                .execute_on_dest_context();
            self.services(&idx).set(service);
        }

    }
}
