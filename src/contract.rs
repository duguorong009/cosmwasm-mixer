#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, DepositMsg, WithdrawMsg};
use crate::state::{};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmwasm-mixer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // TODO: Initialize the state variables.
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit(msg)=> try_deposit(deps, info, msg),
        ExecuteMsg::Withdraw(msg) => try_withdraw(deps, info, msg),
    }
}

pub fn try_deposit(deps: DepsMut, info: MessageInfo, msg: DepositMsg) -> Result<Response, ContractError> {
    // TODO

    Ok(Response::new().add_attribute("method", "try_deposit"))
}
pub fn try_withdraw(deps: DepsMut, info: MessageInfo, msg: WithdrawMsg) -> Result<Response, ContractError> {
    // TODO
    Ok(Response::new().add_attribute("method", "try_withdraw"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        // TODO
    }

    #[test]
    fn test_try_deposit() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        // TODO
    }

    #[test]
    fn test_try_withdraw() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        // TODO
    }
}
