// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    erc721
    (
        init => init
        mint => mint
        approve => approve
        revoke => revoke
        transfer => transfer
        totalMinted => total_minted
        tokenOwner => token_owner
        tokenCount => token_count
        approval => approval
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
