#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint256};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::mixer_verifier::MixerVerifier;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, DepositMsg, WithdrawMsg};
use crate::poseidon::Poseidon;
use crate::state::{Mixer, MerkleTree, MIXER, save_root, POSEIDON, MIXERVERIFIER, FILLEDSUBTREES, MERKLEROOTS, save_subtree};
use crate::zeroes::zeroes;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmwasm-mixer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Validation 1. Check if the funds are sent with this message
    if !info.funds.is_empty() {
        return Err(ContractError::UnnecessaryFunds {});
    }

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Initialize the merkle tree
    let merkle_tree: MerkleTree = MerkleTree {
        levels: msg.merkletree_levels,
        current_root_index: 0,
        next_index: 1,
    };
    // Initialize the Mixer
    let mixer: Mixer = Mixer {
        initialized: true,
        deposit_size: Uint256::from(msg.deposit_size.u128()),
        merkle_tree: merkle_tree,
    };
    MIXER.save(deps.storage, &mixer)?;

    // Initialize the poseidon hasher
    POSEIDON.save(deps.storage, &Poseidon::new())?;

    // Initialize the Mixer_Verifier
    MIXERVERIFIER.save(deps.storage, &MixerVerifier::new())?;

    for i in 0..msg.merkletree_levels {
        save_subtree(deps.storage, i as u32, &zeroes(i))?;
    }

    save_root(deps.storage, 0_u32, &zeroes(msg.merkletree_levels))?;

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
    use cosmwasm_std::{coins, from_binary, Uint128, attr};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let env = mock_env();
        let info = mock_info("anyone", &[]);
        let instantiate_msg = InstantiateMsg {
            merkletree_levels: 8,
            deposit_size: Uint128::from(1_000_000_u128),
        };

        // Should pass this "unwrap" if success.
        let response = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();

        assert_eq!(response.attributes, vec![
            attr("method", "instantiate"),
            attr("owner", "anyone"),
        ]);
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
