# load file with ". /path/to/file"

PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/subscription/wallets/alice.pem"
WALLET_BOB="${PWD}/subscription/wallets/bob.pem"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgq9d60a4wwgqrhsx8u2wfuvyp0d5e9py5q7wpqcp8j35"
CONTRACT_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${CONTRACT_ADDRESS})"
ALICE_ADDRESS="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
ALICE_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
ALICE_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
BOB_ADDRESS="erd1wh2rz67zlq5nea7j4lvs39n0yavjlaxal88f744k2ps036ary8dq3ptyd4"
BOB_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
BOB_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
MARTA_ADDRESS="erd1uycnjd0epww6xrmn0xjdkfhjengpaf4l5866rlrd8qpcsamrqr8qs6ucxx"
MARTA_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"
MARTA_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"

NETFLIX_ADDRESS="erd1qqqqqqqqqqqqqpgqfldvjsvemctf6p3vfc2k7tt83u4zpwe6y8dqmhkke2"


### MAIN

deploy() {
 mxpy contract deploy --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --bytecode=subscription/output/subscription.wasm \
    --pem="subscription/wallets/alice.pem" \
    --gas-limit=60000000 \
    --recall-nonce \
    --send \
    --metadata-payable \
    --arguments ${NETFLIX_ADDRESS}
}

upgrade() {
 mxpy contract upgrade ${CONTRACT_ADDRESS} \
    --pem="subscription/wallets/alice.pem" \
    --chain=${CHAIN_ID} \
    --proxy=${PROXY} \
    --recall-nonce \
    --bytecode=subscription/output/subscription.wasm \
    --gas-limit=100000000 \
    --send \
    --metadata-payable \
    --arguments ${NETFLIX_ADDRESS}
}

########

TOKEN_1=AMS-3a6740
TOKEN_2=BMS-e00535
TOKEN_3=USDC-79d9a4


DEPOSIT_FUNCTION=depositToken
DEPOSIT_TOKEN_1=AMS-3a6740
DEPOSIT_SUPPLY_1=100
DEPOSIT_TOKEN_2=BMS-e00535
DEPOSIT_SUPPLY_2=280
DEPOSIT_TOKEN_DUMMY=BND2-90614b
DEPOSIT_SUPPLY_DUMMY=2
 
depositToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="subscription/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="ESDTTransfer" \
    --arguments "str:"${DEPOSIT_TOKEN_2} ${DEPOSIT_SUPPLY_2} "str:"${DEPOSIT_FUNCTION} ${DEPOSIT_SUPPLY_2} "str:"${DEPOSIT_TOKEN_2}
} 

WITHDRAW_FUNCTION=withdrawToken
WITHDRAW_TOKEN_1=AMS-3a6740
WITHDRAW_SUPPLY_1=100
WITHDRAW_TOKEN_2=BMS-e00535
WITHDRAW_SUPPLY_2=100
 

withdrawToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="subscription/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="withdrawToken" \
    --arguments ${WITHDRAW_SUPPLY_2} "str:"${WITHDRAW_TOKEN_2} ${ALICE_ADDRESS} ${ALICE_ADDRESS}
}  

########

SERVICE_1=1
SERVICE_2=2
SERVICE_3=3

subscribeToMultipleServices() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="subscription/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="subscribeToMultipleServices" \
    --arguments $SERVICE_1     
}  

unsubscribeFromMultipleServices() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="subscription/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="unsubscribeFromMultipleServices" \
    --arguments $SERVICE_1      
}  

getSubscription() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getSubscription" \
    --arguments ${SERVICE_1}
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getSubscription" \
    --arguments ${SERVICE_2}
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getSubscription" \
    --arguments ${SERVICE_3}    
} 

########


sendTokens() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="subscription/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="sendTokens"
}  


########

getTokensCount() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokensCount"  
} 


TOKEN_ID_1=1
TOKEN_ID_2=2
TOKEN_ID_3=3

getTokens() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokens" \
    --arguments ${TOKEN_ID_1} 
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokens" \
    --arguments ${TOKEN_ID_2} 
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokens" \
    --arguments ${TOKEN_ID_3} 
}

getAddresses() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getLPAddress" \
    --arguments ${TOKEN_ID_1} 
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getLPAddress" \
    --arguments ${TOKEN_ID_2} 
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getLPAddress" \
    --arguments ${TOKEN_ID_3} 
}


getServicesCount() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getServicesCount"  
}
 
 
getServices() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getServices" \
    --arguments $SERVICE_1 
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getServices" \
    --arguments $SERVICE_2 
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getServices" \
    --arguments $SERVICE_3 
}


getNetflix() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getNetflix"
}



getBalance() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getBalance" \
    --arguments ${ALICE_ADDRESS} 
}

getSafePriceView() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getSafePriceView"
}

LP_ID_1=1

getPairCreatedAddress() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getLPAddress" \
    --arguments ${LP_ID_1}
}


GET_TOKEN_1=AMS-3a6740
GET_SUPPLY_1=20 
USDC=USDC-79d9a4

getIds() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getIds" \
    --arguments "str:"${GET_TOKEN_1} 
}


getLastPaymentVec() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getLastPaymentVec"
}
  
 
clearLastVec() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="subscription/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="clearLastVec"
}  

  