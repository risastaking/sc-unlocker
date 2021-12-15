use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("unlocker");

    blockchain.register_contract(
        "file:output/unlocker.wasm",
        Box::new(|context| Box::new(unlocker::contract_obj(context))),
    );
    blockchain
}

#[test]
fn unlocker_rs() {
    elrond_wasm_debug::mandos_rs("mandos/unlocker.scen.json", contract_map());
}

#[test]
fn unlocker_deploy_validations_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/unlocker-deploy-validations.scen.json",
        contract_map(),
    );
}

#[test]
fn unlocker_swap_rs() {
    elrond_wasm_debug::mandos_rs("mandos/unlocker-swap.scen.json", contract_map());
}

#[test]
fn unlocker_withdraw_rs() {
    elrond_wasm_debug::mandos_rs("mandos/unlocker-withdraw.scen.json", contract_map());
}

#[test]
fn unlocker_deposit_rs() {
    elrond_wasm_debug::mandos_rs("mandos/unlocker-deposit.scen.json", contract_map());
}

#[test]
fn unlocker_deposit_multi_rs() {
    elrond_wasm_debug::mandos_rs("mandos/unlocker-deposit-multi.scen.json", contract_map());
}
