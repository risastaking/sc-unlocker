use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("adder");

    blockchain.register_contract(
        "file:output/adder.wasm",
        Box::new(|context| Box::new(adder::contract_obj(context))),
    );
    blockchain
}

#[test]
fn adder_rs() {
    elrond_wasm_debug::mandos_rs("mandos/adder.scen.json", contract_map());
}
#[test]
fn adder_deploy_validations_rs() {
    elrond_wasm_debug::mandos_rs("mandos/adder-deploy-validations.scen.json", contract_map());
}
#[test]
fn adder_swap_rs() {
    elrond_wasm_debug::mandos_rs("mandos/adder-swap.scen.json", contract_map());
}