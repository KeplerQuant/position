use anchor_lang::AccountDeserialize;
use anyhow::Result;
use raydium_amm_v3::states::PersonalPositionState;
use solana_account_decoder::{
    parse_token::{TokenAccountType, UiAccountState},
    UiAccountData,
};
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::pubkey::Pubkey;

use super::types::PositionNftTokenInfo;

pub fn get_nft_account_and_position_by_owner(
    client: &RpcClient,
    owner: &Pubkey,
    token_program: Pubkey,
    raydium_amm_v3_program: &Pubkey,
) -> Result<Vec<PositionNftTokenInfo>> {
    let all_tokens =
        client.get_token_accounts_by_owner(owner, TokenAccountsFilter::ProgramId(token_program))?;
    let mut position_nft_accounts = Vec::new();
    for keyed_account in all_tokens {
        if let UiAccountData::Json(parsed_account) = keyed_account.account.data {
            if parsed_account.program == "spl-token" || parsed_account.program == "spl-token-2022" {
                if let Ok(TokenAccountType::Account(ui_token_account)) =
                    serde_json::from_value(parsed_account.parsed)
                {
                    let _frozen = ui_token_account.state == UiAccountState::Frozen;

                    let token = ui_token_account.mint.parse::<Pubkey>()?;
                    let token_account = keyed_account.pubkey.parse::<Pubkey>()?;
                    let token_amount = ui_token_account.token_amount.amount.parse::<u64>()?;

                    let _close_authority = ui_token_account.close_authority.map_or(*owner, |s| {
                        s.parse::<Pubkey>()
                            .unwrap_or_else(|err| panic!("Invalid close authority: {}", err))
                    });

                    if ui_token_account.token_amount.decimals == 0 && token_amount == 1 {
                        let (position_pda, _) = Pubkey::find_program_address(
                            &[
                                raydium_amm_v3::states::POSITION_SEED.as_bytes(),
                                token.to_bytes().as_ref(),
                            ],
                            &raydium_amm_v3_program,
                        );
                        position_nft_accounts.push(PositionNftTokenInfo {
                            key: token_account,
                            program: token_program,
                            position: position_pda,
                            mint: token,
                            amount: token_amount,
                            decimals: ui_token_account.token_amount.decimals,
                        });
                    }
                }
            }
        }
    }

    Ok(position_nft_accounts)
}

pub fn get_all_nft_and_position_by_owner(
    client: &RpcClient,
    owner: &Pubkey,
    raydium_amm_v3_program: &Pubkey,
) -> Result<Vec<PositionNftTokenInfo>> {
    let mut spl_nfts = get_nft_account_and_position_by_owner(
        client,
        owner,
        spl_token::id(),
        raydium_amm_v3_program,
    )?;
    let spl_2022_nfts = get_nft_account_and_position_by_owner(
        client,
        owner,
        spl_token_2022::id(),
        raydium_amm_v3_program,
    )?;
    spl_nfts.extend(spl_2022_nfts);
    Ok(spl_nfts)
}

pub fn get_positions_by_owner(
    rpc_client: &RpcClient,
    user_wallet: &Pubkey,
    raydium_amm_v3_program: &Pubkey,
) -> Result<Vec<PersonalPositionState>> {
    let position_nft_infos =
        get_all_nft_and_position_by_owner(rpc_client, user_wallet, raydium_amm_v3_program)?;
    let positions: Vec<Pubkey> = position_nft_infos
        .iter()
        .map(|item| item.position)
        .collect();
    let rsps = rpc_client.get_multiple_accounts(&positions)?;
    let mut user_positions = Vec::new();
    for rsp in rsps {
        match rsp {
            None => continue,
            Some(rsp) => {
                let mut data: &[u8] = &rsp.data;
                let position =
                    raydium_amm_v3::states::PersonalPositionState::try_deserialize(&mut data)?;
                let (personal_position_key, __bump) = Pubkey::find_program_address(
                    &[
                        raydium_amm_v3::states::POSITION_SEED.as_bytes(),
                        position.nft_mint.to_bytes().as_ref(),
                    ],
                    &raydium_amm_v3_program,
                );
                println!("id:{}, lower:{}, upper:{}, liquidity:{}, fees_owed_0:{}, fees_owed_1:{}, fee_growth_inside_0:{}, fee_growth_inside_1:{}", personal_position_key, position.tick_lower_index, position.tick_upper_index, position.liquidity, position.token_fees_owed_0, position.token_fees_owed_1, position.fee_growth_inside_0_last_x64, position.fee_growth_inside_1_last_x64);
                user_positions.push(position);
            }
        }
    }

    Ok(user_positions)
}
