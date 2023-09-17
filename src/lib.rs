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

        #[endpoint(setSubscriptionAddress)]
        fn set_subscription_address(&self);
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
        let _: IgnoreValue = self
            .netflix_contract_proxy(self.netflix().get())
            .set_subscription_address()
            .execute_on_dest_context();
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

    #[inline]
    #[endpoint(withdrawToken)]
    fn withdraw_token(
        &self,
        supply: &BigUint,
        token: &TokenIdentifier,
        from: &ManagedAddress,
        to: &ManagedAddress,
    ) {
        // Check if withdraw call valid
        let id = self.id(&token);
        require!(!id.is_empty(), "Token non existent in Smart Contract");
        let id = id.get();
        require!(
            self.balance(&from).contains_key(&id),
            "The caller has no balance for this token"
        );
        let mut balance = self.balance(&from).get(&id).unwrap();
        require!(
            balance >= *supply,
            "Token balance lower than requested for withdrawal"
        );

        self.send().direct_esdt(&to, &token, 0u64, &supply);
        balance -= supply;
        if balance == 0 {
            self.balance(&from).remove(&id);
        } else {
            self.balance(&from).insert(id, balance);
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
            Some(_) => {
                // Normally should panic but in the context of subscription to
                // multiple services if already subscribed we pass
                // sc_panic!("Already subscribed to this service.");
            }
            None => {
                let timestamp = self.blockchain().get_block_timestamp();
                self.subscription(service_id).insert(caller, timestamp);
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
            Some(_) => {
                self.subscription(service_id).remove(&caller);
            }
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

    #[inline]
    #[endpoint(getTokenPairValue)]
    fn get_token_pair_value(
        &self,
        token: TokenIdentifier,
        amount: BigUint,
        lp_address: &ManagedAddress,
    ) -> BigUint {
        let view_pair_address = self.safe_price_view().get();
        let payment = EsdtTokenPayment::new(token, 0, amount);
        let result: EsdtTokenPayment = self
            .safe_price_view_contract_proxy(view_pair_address)
            .get_safe_price_by_default_offset(lp_address, payment)
            .execute_on_dest_context();
        result.amount
    }

    #[inline]
    fn update_subscription_timestamp(
        &self,
        service_id: &usize,
        address: ManagedAddress,
        payments_count: u64,
    ) {
        let current_timestamp = self.subscription(&service_id).get(&address).unwrap();
        let periodicity = self.services(&service_id).get().periodicity;
        self.subscription(service_id)
            .insert(address, current_timestamp + payments_count * periodicity);
    }

    #[inline]
    fn send_subscription_tokens_and_update_timestamp(
        &self,
        payment_vec: UnorderedSetMapper<EsdtTokenPayment>,
        address: ManagedAddress,
        service_id: usize,
        payments_count: u64,
    ) {
        let netflix = self.netflix().get();
        for payment in payment_vec.iter() {
            self.withdraw_token(
                &payment.amount,
                &payment.token_identifier,
                &address,
                &netflix,
            )
        }
        // Update timestamp to reflect payment
        self.update_subscription_timestamp(&service_id, address, payments_count);
    }

    // Calculate tokens payment for an address that subscribed to a service
    #[inline]
    fn calculate_tokens_payment(&self, service_id: usize, address: ManagedAddress, timestamp: u64) {
        // service_id - The service id
        // address    - The address that subscribed to the service
        // timestamp  - Subscription time or last payment time
        let current_timestamp = self.blockchain().get_block_timestamp();
        let service = self.services(&service_id).get();
        let payments_count = (current_timestamp - timestamp) / service.periodicity;
        let mut amount_owned =
            BigUint::from(payments_count) * service.price;
        // Cycle through owned tokens and check if payment is possible
        // This can either be sequential or a greedy algorithm
        let usdc_id = 3;
        let usdc_token = self.tokens(&usdc_id).get();
        let my_storage_key: StorageKey<_> = StorageKey::from("unorderded_storage_key");
        let mut payment_vec: UnorderedSetMapper<EsdtTokenPayment> =
            UnorderedSetMapper::new(my_storage_key);
        payment_vec.clear();
        for (token_id, balance) in self.balance(&address).iter() {
            let lp_address = self.lp_address(&token_id).get();
            let token: TokenIdentifier = self.tokens(&token_id).get();
            let dollar_equivalent =
                self.get_token_pair_value(token.clone(), balance.clone(), &lp_address);
            // if dollar_equivalent is greater than needed payment difference
            // we get the exact amount needed for the difference and break
            if amount_owned.clone() < dollar_equivalent {
                // token_rest is the left amount in token needed to cover the payment difference
                let amount_rest = self.get_token_pair_value(
                    usdc_token.clone(),
                    amount_owned.clone(),
                    &lp_address,
                );
                // add last payment to payment_vec and break
                payment_vec.insert(EsdtTokenPayment::new(
                    token,
                    0,
                    amount_rest,
                ));
                amount_owned = BigUint::from(0u64);
                break;
            } else {
                // if dollar equivalent is lower than needed payment difference
                // we get that amount and move on to the next token
                payment_vec.insert(EsdtTokenPayment::new(token, 0, balance.clone()));
                amount_owned -= dollar_equivalent;
            }
        }
        // if amount owned equals 0 we have the token payment information
        // to make a valid payment to Netflix Smart Contract
        // else address can't pay the subscription so we move to the next one
        if amount_owned == 0 {
            self.send_subscription_tokens_and_update_timestamp(payment_vec, address, service_id, payments_count);
        }
    }

    #[endpoint(sendTokens)]
    fn send_tokens(&self) {
        // check all services
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.netflix().get(),
            "Only Netflix contract can call this endpoint"
        );
        for idx in 1..self.services_count().get() {
            // for each service check all subscriptions
            for (address, timestamp) in self.subscription(&idx).iter() {
                self.calculate_tokens_payment(idx, address, timestamp);
            }
        }
    }
}
