ALICE="${USERS}/alice.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
TOKEN_1=0x4c4b4d45582d6366613133642d3033 # LKMEX-cfa13d-03
TOKEN_2=0x4c4b4d45582d6366613133642d32383831 # LKMEX-cfa13d-2881

deploy() {
    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${ALICE} \
    --arguments ${TOKEN_1} \
    --gas-limit=50000000 --send \
    --outfile="deploy-devnet.interaction.json" || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo "initial allowed token ${TOKEN_1}"
    echo "Smart contract address: ${ADDRESS}"
}

addTokenAllowed() {
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${ALICE} --gas-limit=50000000 \
    --function="addTokenAllowed" --arguments ${TOKEN_2} --send
}

getTokensAllowed() {
    erdpy --verbose contract query ${ADDRESS} --function="getTokensAllowed"
}
