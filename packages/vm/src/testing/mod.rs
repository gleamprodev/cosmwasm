// The external interface is `use cosmwasm_vm::testing::X` for all integration testing symbols, no matter where they live internally.

mod calls;
mod instance;
mod mock;
mod storage;

pub use calls::{handle, init, query};
pub use instance::{
    mock_instance, mock_instance_with_balances, mock_instance_with_gas_limit, test_io,
};
pub use mock::{
    mock_dependencies, mock_dependencies_with_balances, mock_env, MockApi, MockQuerier,
    MOCK_CONTRACT_ADDR,
};
pub use storage::MockStorage;
