use contract::msg::{InstantiateMsg, QueryMsg, ExecuteMsg};
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg
    }
}
