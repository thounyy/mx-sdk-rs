// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           25
// Async Callback:                       1
// Total number of exported functions:  27

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    forwarder_raw
    (
        init => init
        forward_payment => forward_payment
        forward_direct_esdt_via_transf_exec => forward_direct_esdt_via_transf_exec
        forward_direct_esdt_multi => forward_direct_esdt_multi
        forward_async_call => forward_async_call
        forward_async_call_half_payment => forward_async_call_half_payment
        forward_transf_exec_egld => forward_transf_exec_egld
        forward_transf_exec_esdt => forward_transf_exec_esdt
        forward_transf_exec => forward_transf_exec
        forward_transf_exec_twice => forward_transf_exec_twice
        forward_async_retrieve_multi_transfer_funds => forward_async_retrieve_multi_transfer_funds
        forwarder_async_send_and_retrieve_multi_transfer_funds => forwarder_async_send_and_retrieve_multi_transfer_funds
        callback_args => callback_args
        callback_payments => callback_payments
        callback_payments_triples => callback_payments_triples
        clear_callback_info => clear_callback_info
        callback_args_at_index => callback_args_at_index
        callback_payment_at_index => callback_payment_at_index
        call_execute_on_dest_context => call_execute_on_dest_context
        call_execute_on_dest_context_twice => call_execute_on_dest_context_twice
        call_execute_on_same_context => call_execute_on_same_context
        call_execute_on_dest_context_readonly => call_execute_on_dest_context_readonly
        deploy_contract => deploy_contract
        deploy_from_source => deploy_from_source
        upgrade => upgrade
        upgrade_from_source => upgrade_from_source
    )
}

multiversx_sc_wasm_adapter::async_callback! { forwarder_raw }
