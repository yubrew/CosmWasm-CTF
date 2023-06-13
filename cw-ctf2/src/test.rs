#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, BalanceResponse, Coin, Uint128};

    #[test]
    #[should_panic(expected = "Invalid instantiation")]
    fn invalid_init() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(0, "uosmo".to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    fn deposit_success() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "uosmo".to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // user able to deposit uosmo
        let info = mock_info("alice", &coins(100, "uosmo"));
        let msg = ExecuteMsg::Deposit {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // verify deposit succeeded
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetBalance {
                address: "alice".to_string(),
            },
        )
        .unwrap();
        let value: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(Uint128::from(100_u64), value.amount.amount);
    }

    #[test]
    #[should_panic(expected = "Invalid deposit!")]
    fn deposit_failure() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "uosmo".to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // invalid deposit
        let info = mock_info("bob", &coins(10, "uluna".to_string()));
        let msg = ExecuteMsg::Deposit {};
        let _err = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid deposit!")]
    fn exploit_fail() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "uosmo".to_string()));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // we send a vector of coins to trick the system we deposited OSMO
        let malicious_funds: Vec<Coin> = vec![
            Coin {
                denom: "umyr".to_string(),
                amount: Uint128::from(1000_u64),
            },
            Coin {
                denom: "uosmo".to_string(),
                amount: Uint128::from(0_u64),
            },
        ];
        let info = mock_info("hacker", &malicious_funds);
        let msg = ExecuteMsg::Deposit {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }
}
