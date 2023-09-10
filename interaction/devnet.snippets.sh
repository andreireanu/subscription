# load file with ". /path/to/file"

PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/subscription/wallets/alice.pem"
WALLET_BOB="${PWD}/subscription/wallets/bob.pem"
# SC ADDRESS WITH LOCAL MINT:
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqtxj7x99yz4x97g0c3kdgf03stk2uvt807wpq8emhnq"
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
    --gas-limit=80000000 \
    --send \
    --metadata-payable \
    --arguments ${NETFLIX_ADDRESS}
}

########

getTokensCount() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokensCount"  
} 

TOKEN_1=1
TOKEN_2=2

getTokens() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokens" \
    --arguments ${TOKEN_1} 
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getTokens" \
    --arguments ${TOKEN_2}
}

getServicesCount() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getServicesCount"  
}

SERVICE_ID=2

getServices() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getServices" \
    --arguments ${SERVICE_ID} 
}

########

DEPOSIT_FUNCTION=depositToken
DEPOSIT_TOKEN=AMS-3a6740
DEPOSIT_SUPPLY=1
DEPOSIT_TOKEN_DUMMY=BND2-90614b
DEPOSIT_SUPPLY_DUMMY=2
 
depositToken() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="erc1155/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="ESDTTransfer" \
    --arguments "str:"${DEPOSIT_TOKEN} ${DEPOSIT_SUPPLY} "str:"${DEPOSIT_FUNCTION} ${DEPOSIT_SUPPLY} "str:"${DEPOSIT_TOKEN}
} 


getBalance() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
    --proxy=${PROXY} \
    --function="getBalance" \
    --arguments ${ALICE_ADDRESS} 
}