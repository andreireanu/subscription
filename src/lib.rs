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

        #[view(getLPAddress)]
        fn lp_address(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;

        #[view(getSafePriceView)]
        fn safe_price_view(&self) -> SingleValueMapper<ManagedAddress>;
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
        for idx in 1..tokens_count {
            let token: TokenIdentifier = self
                .contract_proxy(self.netflix().get())
                .tokens(&idx)
                .execute_on_dest_context();
            self.id(&token).set(&idx);
            self.tokens(&idx).set(&token);
            let lp_address: ManagedAddress = self
                .contract_proxy(self.netflix().get())
                .lp_address(&idx)
                .execute_on_dest_context();
            self.lp_address(&idx).set(lp_address);
        }
        let services_count: usize = self
            .contract_proxy(self.netflix().get())
            .services_count()
            .execute_on_dest_context();
        self.services_count().set(services_count);
        for idx in 1..services_count {
            let service: Service = self
                .contract_proxy(self.netflix().get())
                .services(&idx)
                .execute_on_dest_context();
            self.services(&idx).set(service);
        }
        let safe_price_view_address: ManagedAddress = self
            .contract_proxy(self.netflix().get())
            .safe_price_view()
            .execute_on_dest_context();
        self.safe_price_view().set(safe_price_view_address);
    }


    #[payable("*")]
    #[endpoint(depositToken)]
    fn deposit_token(&self, supply: BigUint, token: TokenIdentifier) {
        let payment = self.call_value().single_esdt();
        let id = self.id(&payment.token_identifier);
        require!(!id.is_empty(), "Token not whitelisted for payment.");
        require!(
            &payment.token_identifier == &token,
            "Incorrect parameters for function call. Payment token other than deposited one."
        );
        require!(
            &payment.amount == &supply,
            "Incorrect parameters for function call. Payment amount other than deposited one."
        );
        let id = id.get();
        let caller = self.blockchain().get_caller();
        let balance_option = self.balance(&caller).get(&id);
        match balance_option {
            Some(balance) => self.balance(&caller).insert(id, balance + supply),
            None => self.balance(&caller).insert(id, supply),
        };
    }

    #[endpoint(withdrawToken)]
    fn withdraw_token(&self, supply: BigUint, token: TokenIdentifier) {
        // Check if withdraw call valid
        let id = self.id(&token);
        require!(!id.is_empty(), "Token non existent in Smart Contract");
        let id = id.get();
        let caller = self.blockchain().get_caller();
        require!(
            self.balance(&caller).contains_key(&id),
            "The caller has no balance for this token"
        );
        let mut balance = self.balance(&caller).get(&id).unwrap();
        require!(balance >= supply, "Token balance lower than requested one");

        let _ = self.send().direct_esdt(&caller, &token, 0u64, &supply);
        balance -= supply;
        if balance == 0 {
            self.balance(&caller).remove(&id);
        } else {
            self.balance(&caller).insert(id, balance);
        }
    }

    #[inline]
    fn subscribe_to_single_service(&self, service_id: &usize) {
        require!(
            service_id < &self.services_count().get(),
            "Service not found."
        );
        let caller = self.blockchain().get_caller();
        let service = self.subscription(service_id).get(&caller);
        match service {
            Some(_) => sc_panic!("Already subscribed to this service."),
            None => {
                let timestamp = self.blockchain().get_block_timestamp();
                let periodicity = self.services(service_id).get().periodicity;
                self.subscription(service_id)
                    .insert(caller, timestamp + periodicity);
            }
        };
    }

    #[endpoint(subscribeToMultipleServices)]
    fn subscribe_to_multiple_services(&self, services: MultiValueEncoded<usize>) {
        for service in services {
            self.subscribe_to_single_service(&service);
        }
    }

    #[inline]
    fn unsubscribe_from_single_service(&self, service_id: &usize) {
        require!(
            service_id < &self.services_count().get(),
            "Service not found."
        );
        let caller = self.blockchain().get_caller();
        let service = self.subscription(service_id).get(&caller);
        match service {
            Some(_) => self.subscription(service_id).remove(&caller),
            None => {
                sc_panic!("Not subscribed to this service.")
            }
        };
    }

    #[endpoint(unsubscribeFromMultipleServices)]
    fn unsubscribe_from_multiple_services(&self, services: MultiValueEncoded<usize>) {
        for service in services {
            self.unsubscribe_from_single_service(&service);
        }
    }
}
