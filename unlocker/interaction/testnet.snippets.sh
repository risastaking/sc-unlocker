CAROL="${USERS}/carol.pem"
ADDRESS=$(erdpy data load --key=address-testnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-testnet)
FROM_TOKEN=0x4c4b4d45582d636661313364 # LKMEX-cfa13d
TO_TOKEN=0x4d45582d313065373838 # MEX-10e788
FEE_PERCENT=0x05dc # 1500 - 15%
PROXY=https://testnet-api.elrond.com

deploy() {
    erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${CAROL} --gas-limit=50000000 \
    --metadata-payable \
    --arguments ${FROM_TOKEN} ${TO_TOKEN} ${FEE_PERCENT} \
    --send --outfile="deploy-testnet.interaction.json" --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --key=address-testnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-testnet --value=${TRANSACTION}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

add() {
    read -p "Enter number: " NUMBER
    erdpy --verbose contract call ${ADDRESS} --recall-nonce --pem=${CAROL} --gas-limit=5000000 \
     --function="add" --arguments ${NUMBER} --send --proxy=${PROXY} --chain=T
}

getFee() {
    erdpy --verbose contract query ${ADDRESS} --function="getFee" --proxy=${PROXY}
}
