// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            5
// Async Callback (empty):               1
// Total number of exported functions:   7

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    digital_cash
    (
        fund
        withdraw
        claim
        amount
        deposit
    )
}

elrond_wasm_node::wasm_empty_callback! {}
