use anchor_lang::AccountDeserialize;
use anyhow::Result;
use raydium_amm_v3::states::PersonalPositionState;
use solana_account_decoder::{parse_token::TokenAccountType, UiAccountData};
use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Debug, PartialEq, Eq)]
struct PositionNftTokenInfo {
    key: Pubkey,
    program: Pubkey,
    position: Pubkey,
    mint: Pubkey,
    amount: u64,
    decimals: u8,
}

fn get_nft_account_and_position_by_owner(
    client: &RpcClient,
    owner: &Pubkey,
    token_program: Pubkey,
    program_id: &Pubkey,
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
                    let token = ui_token_account.mint.parse::<Pubkey>()?;
                    let token_account = keyed_account.pubkey.parse::<Pubkey>()?;
                    let token_amount = ui_token_account.token_amount.amount.parse::<u64>()?;

                    if ui_token_account.token_amount.decimals == 0 && token_amount == 1 {
                        let position_pda = get_position_pda(&token, program_id);
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

fn get_all_nft_and_position_by_owner(
    client: &RpcClient,
    owner: &Pubkey,
    program_id: &Pubkey,
) -> Result<Vec<PositionNftTokenInfo>> {
    let mut spl_nfts =
        get_nft_account_and_position_by_owner(client, owner, spl_token::id(), program_id)?;
    let spl_2022_nfts =
        get_nft_account_and_position_by_owner(client, owner, spl_token_2022::id(), program_id)?;
    spl_nfts.extend(spl_2022_nfts);
    Ok(spl_nfts)
}

pub fn get_position_pda(position_mint: &Pubkey, program_id: &Pubkey) -> Pubkey {
    let (personal_position_key, __bump) = Pubkey::find_program_address(
        &[
            raydium_amm_v3::states::POSITION_SEED.as_bytes(),
            position_mint.to_bytes().as_ref(),
        ],
        &program_id,
    );

    personal_position_key
}

pub fn get_positions_by_owner(
    client: &RpcClient,
    user_wallet: &Pubkey,
    program_id: &Pubkey,
) -> Result<Vec<PersonalPositionState>> {
    let position_nft_infos = get_all_nft_and_position_by_owner(client, user_wallet, program_id)?;
    let positions: Vec<Pubkey> = position_nft_infos
        .iter()
        .map(|item| item.position)
        .collect();
    let rsps = client.get_multiple_accounts(&positions)?;
    let mut user_positions = Vec::new();
    for rsp in rsps {
        match rsp {
            None => continue,
            Some(rsp) => {
                let position = raydium_amm_v3::states::PersonalPositionState::try_deserialize(
                    &mut rsp.data.as_ref(),
                )?;
                user_positions.push(position);
            }
        }
    }

    Ok(user_positions)
}
