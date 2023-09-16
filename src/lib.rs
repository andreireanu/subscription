#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod storage;
use multiversx_sc::storage::StorageKey;
use storage::Service;

mod netflix_proxy {
    multiversx_sc::imports!();
    use crate::storage::Service;

    #[multiversx_sc::proxy]
    pub trait NetflixContract {
        #[view(getTokensCount)]
        fn tokens_count(&self) -> SingleValueMapper<usize>;

        #[endpoint(getTokens)]
        fn tokens(&self, id: &usize) -> SingleValueMapper<TokenIdentifier>;

        #[view(getServicesCount)]
        fn services_count(&self) -> SingleValueMapper<usize>;

        #[view(getServices)]
        fn services(&self, id: &usize) -> SingleValueMapper<Service<Self::Api>>;

        #[view(getLPAddress)]
        fn lp_address(&self, id: &usize) -> SingleValueMapper<ManagedAddress>;

        #[view(getSafePriceView)]
        fn safe_price_view(&self) -> SingleValueMapper<ManagedAddress>;
    }
}

mod safe_price_view_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait SafePriceViewContract {
        #[view(getSafePriceByDefaultOffset)]
        fn get_safe_price_by_default_offset(
            &self,
            pair_address: ManagedAddress,
            input_payment: EsdtTokenPayment,
        ) -> EsdtTokenPayment;
    }
}


#[multiversx_sc::contract]
pub trait SubscriptionContract: crate::storage::StorageModule {
    #[proxy]
    fn netflix_contract_proxy(&self, sc_address: ManagedAddress)
        -> netflix_proxy::Proxy<Self::Api>;

    #[proxy]
    fn safe_price_view_contract_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> safe_price_view_proxy::Proxy<Self::Api>;

    
        
    

    #[init]
    fn init(&self, netflix_address: ManagedAddress) {
        self.netflix().set(netflix_address);
        let tokens_count: usize = self
            .netflix_contract_proxy(self.netflix().get())
            .tokens_count()
            .execute_on_dest_context();
        self.tokens_count().set(tokens_count);
        for idx in 1..tokens_count {
            let token: TokenIdentifier = self
                .netflix_contract_proxy(self.netflix().get())
                .tokens(&idx)
                .execute_on_dest_context();
            self.id(&token).set(&idx);
            self.tokens(&idx).set(&token);
            let lp_address: ManagedAddress = self
                .netflix_contract_proxy(self.netflix().get())
                .lp_address(&idx)
                .execute_on_dest_context();
            self.lp_address(&idx).set(lp_address);
        }
        let services_count: usize = self
            .netflix_contract_proxy(self.netflix().get())
            .services_count()
            .execute_on_dest_context();
        self.services_count().set(services_count);
        for idx in 1..services_count {
            let service: Service<Self::Api> = self
                .netflix_contract_proxy(self.netflix().get())
                .services(&idx)
                .execute_on_dest_context();
            self.services(&idx).set(service);
        }
        let safe_price_view_address: ManagedAddress = self
            .netflix_contract_proxy(self.netflix().get())
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
        require!(balance >= supply, "Token balance lower than requested for withdrawal");

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
            Some(_) =>  {
                // Normally should panic but in the context of subscription to 
                // multiple services if already subscribed we pass
                // sc_panic!("Already subscribed to this service.");
             },
            None => {
                let timestamp = self.blockchain().get_block_timestamp();
                self.subscription(service_id)
                    .insert(caller, timestamp);
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
            Some(_) => { self.subscription(service_id).remove(&caller); },
            None => {
                // Normally should panic but in the context of unsubscription to 
                // multiple services if already unsubscribed we pass
                // sc_panic!("Not subscribed to this service.")
            }
        };
    }

    #[endpoint(unsubscribeFromMultipleServices)]
    fn unsubscribe_from_multiple_services(&self, services: MultiValueEncoded<usize>) {
        for service in services {
            self.unsubscribe_from_single_service(&service);
        }
    }

    // Get pair token equivalent
    #[inline]
    #[endpoint(getTokenPairValue)]
    fn get_token_pair_value(
        &self,
        token: TokenIdentifier,
        amount: BigUint,
        lp_address: &ManagedAddress,
    ) -> BigUint {
        // let token_id = self.id(&token).get();
        // let lp_address = self.lp_address(&token_id).get();
        // let one_usdc_payment = EsdtTokenPayment::new(*usdc, 0, BigUint::from(10u64.pow(18)));
        // let view_pair_address = self.safe_price_view().get();
        // // Call LP Safe Price View with 1$ as payment
        // let result: EsdtTokenPayment = self
        //     .safe_price_view_contract_proxy(view_pair_address)
        //     .get_safe_price_by_default_offset(lp_address, one_usdc_payment)
        //     .execute_on_dest_context();
        // result.amount
        let view_pair_address = self.safe_price_view().get();
        let payment = EsdtTokenPayment::new(token, 0, amount);
        let result: EsdtTokenPayment = self
            .safe_price_view_contract_proxy(view_pair_address)
            .get_safe_price_by_default_offset(lp_address, payment)
            .execute_on_dest_context();
        result.amount
    }

    // Calculate tokens payment for an address that subscribed to a service
    #[inline]
    #[endpoint(calculateTokensPayment)]
    fn calculate_tokens_payment(&self, service_id: usize, address: ManagedAddress, timestamp: u64) {
        // service_id - The service id
        // address    - The address that subscribed to the service
        // timestamp  - Subscription time or last payment time
        let current_timestamp = self.blockchain().get_block_timestamp();
        let service = self.services(&service_id).get();
        let amount_owned =
            BigUint::from((current_timestamp - timestamp) / service.periodicity) * service.price;
        self.amount_owned().set(&amount_owned);
        // Cycle through owned tokens and check if payment is possible
        let usdc_id = 3;
        let usdc_token = self.tokens(&usdc_id).get();
        let my_storage_key: StorageKey<_> = StorageKey::from("unorderded_storage_key");
        let mut payment_vec: UnorderedSetMapper<EsdtTokenPayment> =
            UnorderedSetMapper::new(my_storage_key);
        // let mut amount_in_balance = BigUint::from(0u64);
        for (token_id, balance) in self.balance(&address).iter() {
            let lp_address = self.lp_address(&token_id).get();
            let token: TokenIdentifier = self.tokens(&token_id).get();
            let dollar_equivalent = self.get_token_pair_value(token.clone(), balance, &lp_address);
            // if dollar_equivalent is greater than needed payment difference
            // we get the exact amount needed for the difference
            if amount_owned.clone() < dollar_equivalent {
                // token_rest is the left amount in token needed to cover the payment difference
                let amount_rest = self.get_token_pair_value(
                    usdc_token.clone(),
                    amount_owned.clone(),
                    &lp_address,
                );
                // add last payment to paymentVec and break
                payment_vec.insert(EsdtTokenPayment::new(token, 0, amount_rest));
            }
        }
        // self.dollar_equivalent().set(amount_in_balance);
    }

    #[endpoint(sendTokens)]
    fn send_tokens(&self) {
        // check all services
        for idx in 1..2 {
            // for each service check all subscriptions
            for (address, timestamp) in self.subscription(&idx).iter() {
                self.calculate_tokens_payment(idx, address, timestamp);
            }
        }
    }

    #[endpoint(clearPairValue)]
    fn clear_pair_value(&self) {
        self.pair_value().set(BigUint::from(0u64));
    }
}
