# load file with ". /path/to/file"

PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"
WALLET_ALICE="${PWD}/wallets/alice.pem"
WALLET_BOB="${PWD}/wallets/bob.pem"
ALICE_ADDRESS="erd1aqd2v3hsrpgpcscls6a6al35uc3vqjjmskj6vnvl0k93e73x7wpqtpctqw"
ALICE_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
ALICE_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${ALICE_ADDRESS})"
BOB_ADDRESS="erd1wh2rz67zlq5nea7j4lvs39n0yavjlaxal88f744k2ps036ary8dq3ptyd4"
BOB_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
BOB_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${BOB_ADDRESS})"
MARTA_ADDRESS="erd1uycnjd0epww6xrmn0xjdkfhjengpaf4l5866rlrd8qpcsamrqr8qs6ucxx"
MARTA_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"
MARTA_ADDRESS_HEXX="0x$(mxpy wallet bech32 --decode ${MARTA_ADDRESS})"

ROUTER_ADDRESS="erd1qqqqqqqqqqqqqpgquy3yzju4myzmxmpsflh6wpg3j5heytez7wpqeykdhd"
PAIR_SMART_CONTRACT="erd1qqqqqqqqqqqqqpgq69t5g08m6n825mjmj0ueh0vt9v8x4jfh7wpqe3y9d9"
PAIR_CREATED_ADDRESS="erd1qqqqqqqqqqqqqpgqfs5vg9n23hvrgfmye4h9vj8p6ljtz2m37wpqj726a5"
SAFE_PRICE_VIEW="erd1qqqqqqqqqqqqqpgqs5t29sk6v0knxxnnc29pv6y0zgmemh287wpqmcx970"
 
### MAIN

#### ROUTER ####

deployRouterContract() {
    mxpy --verbose contract deploy --recall-nonce \
          --pem="${PWD}/wallets/alice.pem" \
          --gas-limit=600000000 \
          --proxy=${PROXY} --chain=${CHAIN_ID} \
          --bytecode="${PWD}/router/output/router.wasm" \
          --outfile="deploy-route-internal.interaction.json" --send || return
    
    ADDRESS=$(mxpy data parse --file="deploy-route-internal.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=router-address --value=${ADDRESS}

    echo ""
    echo "Route Smart contract address: ${ADDRESS}"
}

 
 
#####################

TOKEN_1=AMS-3a6740
TOKEN_2=USDC-79d9a4

deployPairContract() {
    first_token="0x$(echo -n $TOKEN_1 | xxd -p -u | tr -d '\n')"
    second_token="0x$(echo -n $TOKEN_2 | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract deploy --recall-nonce \
          --pem="${PWD}/wallets/alice.pem" \
          --gas-limit=600000000 \
          --metadata-payable \
          --proxy=${PROXY} --chain=${CHAIN_ID} \
          --bytecode="${PWD}/pair/output/pair.wasm" \
          --arguments "str:"$TOKEN_1 "str:"$TOKEN_2 ${ROUTER_ADDRESS} ${ALICE_ADDRESS} 0x000000000000012C 0x0000000000000032 ${ALICE_ADDRESS} ${ALICE_ADDRESS} \
          --outfile="deploy-pair-internal.interaction.json" --send || return
    
    ADDRESS=$(mxpy data parse --file="deploy-pair-internal.interaction.json" --expression="data['contractAddress']")

    echo ""
    echo "Pair Smart contract address: ${ADDRESS}"
}

################# UPGRADE

upgrade() {
    mxpy --verbose contract upgrade ${ROUTER_ADDRESS} \
          --recall-nonce \
          --pem="${PWD}/wallets/alice.pem" \
          --gas-limit=600000000 \
          --proxy=${PROXY} --chain=${CHAIN_ID} \
          --bytecode="${PWD}/router/output/router.wasm" \
          --arguments ${PAIR_SMART_CONTRACT} \
          --send || return
}
 

#####################

NONCE=0
NO_OF_TOKENS=2
AMOUNT_TOKEN_1=500
AMOUNT_TOKEN_2=10
DECIMALS_TOKEN_2=18
POWER_TOKEN_2=$((10**${DECIMALS_TOKEN_2}))
AMOUNT_TOKEN_2_POWERED=$( printf "%.0f" $(echo "${AMOUNT_TOKEN_2} * ${POWER_TOKEN_2}" | bc) ) 
FUNCTION_NAME="addLiquidity"
FUNCTION_NAME_INITIAL="addInitialLiquidity"
LP_TOKEN=AMSUSDC


createPair() {
    mxpy --verbose contract call ${ROUTER_ADDRESS} \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="${PWD}/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="createPair" \
    --arguments "str:"$TOKEN_1 "str:"$TOKEN_2 ${ALICE_ADDRESS} 0x000000000000012C 0x0000000000000032 ${ALICE_ADDRESS} ${ALICE_ADDRESS} ${ROUTER_ADDRESS} \
    --outfile="create-pair.interaction.json" \
    --send || return

}

issueLpToken() {
    mxpy --verbose contract call ${ROUTER_ADDRESS} \
    --send \
    --value=50000000000000000 \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="${PWD}/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="issueLpToken" \
    --arguments ${PAIR_CREATED_ADDRESS} "str:"$LP_TOKEN "str:"$LP_TOKEN    
}


setLocalRoles() {
    mxpy --verbose contract call ${ROUTER_ADDRESS} \
    --send \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} \
    --recall-nonce \
    --pem="${PWD}/wallets/alice.pem" \
    --gas-limit=100000000 \
    --function="setLocalRoles" \
    --arguments ${PAIR_CREATED_ADDRESS}    
}


addInitialLiquidity() {
    mxpy --verbose contract call ${ALICE_ADDRESS} --recall-nonce \
        --pem="${PWD}/wallets/alice.pem" \
        --gas-limit=30000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments ${PAIR_CREATED_ADDRESS} ${NO_OF_TOKENS} "str:"${TOKEN_1} ${NONCE} ${AMOUNT_TOKEN_1} "str:"${TOKEN_2} ${NONCE} ${AMOUNT_TOKEN_2_POWERED} "str:"${FUNCTION_NAME_INITIAL} \
        --send || return
}
 
addLiquidity() {
    mxpy --verbose contract call ${ALICE_ADDRESS} --recall-nonce \
        --pem="${PWD}/wallets/alice.pem" \
        --gas-limit=30000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="MultiESDTNFTTransfer" \
        --arguments ${PAIR_CREATED_ADDRESS} ${NO_OF_TOKENS} "str:"${TOKEN_1} ${NONCE} ${AMOUNT_TOKEN_1} "str:"${TOKEN_2} ${NONCE} ${AMOUNT_TOKEN_2_POWERED} "str:"${FUNCTION_NAME} ${AMOUNT_TOKEN_1} ${AMOUNT_TOKEN_2}\
        --send || return
}


deploySafePriceContract() {
    mxpy --verbose contract deploy --recall-nonce \
          --pem="${PWD}/wallets/alice.pem" \
          --gas-limit=600000000 \
          --metadata-payable \
          --proxy=${PROXY} --chain=${CHAIN_ID} \
          --bytecode="${PWD}/pair/output/safe-price-view.wasm" \
          --outfile="safe-price-view.interaction.json" \
          --arguments ${PAIR_CREATED_ADDRESS} \
          --send || return
}



 