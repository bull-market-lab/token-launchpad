use super::assert_helper::{
    assert_can_send, assert_can_update_approvals,
    assert_max_base_denom_supply_not_reached,
};
use crate::state::{
    CURRENT_NFT_SUPPLY, MAX_NFT_SUPPLY, MINT_GROUPS, NFTS, RECYCLED_NFTS,
    RECYCLED_NFT_IDS,
};
use cosmwasm_std::{
    Addr, BlockInfo, Order, QuerierWrapper, StdError, StdResult, Storage,
    Uint128,
};
use cw721::Approval as Cw721Approval;
use cw721_base::state::{
    Approval as Cw721BaseApproval, TokenInfo as NftTokenInfo,
};
use cw721_metadata_onchain::{
    Extension as NftExtension, Metadata as NftMetadata,
};
use cw_utils::Expiration;
use sha3::{Digest, Keccak256};
use shared_pkg::error::ContractError;

fn humanize_approval(approval: &Cw721BaseApproval) -> Cw721Approval {
    Cw721Approval {
        spender: approval.spender.to_string(),
        expires: approval.expires,
    }
}
pub fn humanize_approvals(
    block: &BlockInfo,
    nft: &NftTokenInfo<NftExtension>,
    include_expired: bool,
) -> Vec<Cw721Approval> {
    nft.approvals
        .iter()
        .filter(|apr| include_expired || !apr.expires.is_expired(block))
        .map(humanize_approval)
        .collect()
}

pub fn parse_token_id_from_string_to_uint128(
    token_id: String,
) -> StdResult<u128> {
    let token_id_in_u128 = token_id
        .parse::<u128>()
        .map_err(|_| StdError::generic_err("token_id is not a valid u128"))?;
    Ok(token_id_in_u128)
}

pub fn assert_can_mint(
    storage: &mut dyn Storage,
    querier: QuerierWrapper,
    block: &BlockInfo,
    mint_amount: Uint128,
    one_denom_in_base_denom: Uint128,
    base_denom: &str,
    recipient_addr: &Addr,
    user_paid_amount: Uint128,
    mint_group_name: &str,
    merkle_proof: Option<Vec<Vec<u8>>>,
) -> Result<(), ContractError> {
    let mint_group = MINT_GROUPS.may_load(storage, mint_group_name)?;
    match mint_group {
        Some(mg) => {
            if block.time.seconds() < mg.start_time.u64() {
                return Err(ContractError::MintGroupNotStarted {
                    name: mint_group_name.to_string(),
                });
            }
            if block.time.seconds() > mg.end_time.u64() {
                return Err(ContractError::MintGroupEnded {
                    name: mint_group_name.to_string(),
                });
            }
            if mint_amount > mg.max_base_denom_amount_per_mint {
                return Err(ContractError::MintAmountExceedsMaxAmountPerMint {
                    name: mint_group_name.to_string(),
                    mint_amount,
                    max_base_denom_amount_per_mint: mg
                        .max_base_denom_amount_per_mint,
                });
            }
            let required_paid_amount = mg.price_per_base_denom * mint_amount;
            if user_paid_amount < required_paid_amount {
                return Err(ContractError::InsufficientFundsToMint {
                    required: required_paid_amount,
                    paid: user_paid_amount,
                });
            }
            if mg.merkle_root.is_some() {
                if merkle_proof.is_none() {
                    return Err(
                        ContractError::MerkleProofRequiredForMintGroup {
                            name: mint_group_name.to_string(),
                        },
                    );
                }
                let mut hasher = Keccak256::new();
                hasher.update(recipient_addr.to_string().as_bytes());
                let recipient_hash = hasher.finalize().to_vec();
                let mut calculated_root_hash: Vec<u8> = recipient_hash;
                for proof_hash in merkle_proof.unwrap().into_iter() {
                    let mut hasher = Keccak256::new();
                    if calculated_root_hash < proof_hash {
                        hasher.update(&calculated_root_hash);
                        hasher.update(&proof_hash);
                    } else {
                        hasher.update(&proof_hash);
                        hasher.update(&calculated_root_hash);
                    }
                    calculated_root_hash = hasher.finalize().to_vec();
                }
                if calculated_root_hash != mg.merkle_root.unwrap() {
                    return Err(
                        ContractError::InvalidMerkleProofForMintGroup {
                            name: mint_group_name.to_string(),
                        },
                    );
                }
            }
        }
        None => {
            return Err(ContractError::MintGroupNotFound {
                name: mint_group_name.to_string(),
            });
        }
    }
    let current_base_denom_supply = querier.query_supply(base_denom)?.amount;
    let max_nft_supply = MAX_NFT_SUPPLY.load(storage)?;
    assert_max_base_denom_supply_not_reached(
        current_base_denom_supply,
        max_nft_supply * one_denom_in_base_denom,
        mint_amount,
    )?;
    Ok(())
}

pub fn calculate_nft_to_mint_for_ft_mint(
    querier: QuerierWrapper,
    owner_addr: &Addr,
    base_denom: &str,
    ft_mint_amount: Uint128,
    one_denom_in_base_denom: Uint128,
) -> Result<Uint128, ContractError> {
    let before_ft_balance =
        querier.query_balance(owner_addr, base_denom)?.amount;
    let before_nft_balance = before_ft_balance / one_denom_in_base_denom;
    let after_ft_balance = before_ft_balance + ft_mint_amount;
    let after_nft_balance = after_ft_balance / one_denom_in_base_denom;
    let mint_amount = after_nft_balance - before_nft_balance;
    Ok(mint_amount)
}

pub fn calculate_nft_to_burn_for_ft_burn(
    querier: QuerierWrapper,
    owner_addr: &Addr,
    base_denom: &str,
    ft_burn_amount: Uint128,
    one_denom_in_base_denom: Uint128,
) -> Result<Uint128, ContractError> {
    let before_ft_balance =
        querier.query_balance(owner_addr, base_denom)?.amount;
    let before_nft_balance = before_ft_balance / one_denom_in_base_denom;
    let after_ft_balance = before_ft_balance - ft_burn_amount;
    let after_nft_balance = after_ft_balance / one_denom_in_base_denom;
    let burn_amount = before_nft_balance - after_nft_balance;
    Ok(burn_amount)
}

pub fn batch_mint_nft(
    storage: &mut dyn Storage,
    base_uri: &str,
    owner_addr: &Addr,
    amount: Uint128,
) -> Result<(), ContractError> {
    let current_nft_supply = CURRENT_NFT_SUPPLY.load(storage)?;
    for i in 0..amount.u128() {
        let (nft_token_id, nft) = if RECYCLED_NFT_IDS.is_empty(storage)? {
            let nft_token_id =
                (current_nft_supply + Uint128::from(1 + i)).u128();
            let nft = NftTokenInfo {
                owner: owner_addr.clone(),
                approvals: vec![],
                token_uri: Some(format!(
                    "{}/{}",
                    base_uri,
                    current_nft_supply + Uint128::from(1 + i)
                )),
                extension: Some(NftMetadata {
                    // TODO: add metadata
                    ..NftMetadata::default()
                }),
            };
            (nft_token_id, nft)
        } else {
            let recycled_nft_id = RECYCLED_NFT_IDS.pop_front(storage)?.unwrap();
            let recycled_nft = RECYCLED_NFTS.load(storage, recycled_nft_id)?;
            RECYCLED_NFTS.remove(storage, recycled_nft_id);
            (recycled_nft_id, recycled_nft)
        };
        NFTS().update(storage, nft_token_id, |old| match old {
            Some(_) => Err(ContractError::NftTokenIdAlreadyInUse {
                nft_token_id: Uint128::from(nft_token_id),
            }),
            None => Ok(nft),
        })?;
    }
    let updated_nft_supply = current_nft_supply + amount;
    CURRENT_NFT_SUPPLY.save(storage, &updated_nft_supply)?;
    Ok(())
}

pub fn batch_burn_nft(
    storage: &mut dyn Storage,
    owner_addr: &Addr,
    amount: Uint128,
) -> Result<(), ContractError> {
    let current_nft_supply = CURRENT_NFT_SUPPLY.load(storage)?;
    let token_ids: Vec<u128> = NFTS()
        .idx
        .owner
        .prefix(owner_addr.clone())
        .keys(storage, None, None, Order::Ascending)
        .take(amount.u128() as usize)
        .collect::<StdResult<Vec<_>>>()?;
    if token_ids.len() != amount.u128() as usize {
        return Err(ContractError::CannotBurnMoreNftThanOwned {
            available: Uint128::from(token_ids.len() as u128),
            try_to_burn: amount,
        });
    }
    for token_id in token_ids {
        RECYCLED_NFT_IDS.push_back(storage, &token_id)?;
        let recycled_nft = NFTS().load(storage, token_id)?;
        RECYCLED_NFTS.save(storage, token_id, &recycled_nft)?;
        NFTS().remove(storage, token_id)?;
    }
    let updated_nft_supply: Uint128 = current_nft_supply - amount;
    CURRENT_NFT_SUPPLY.save(storage, &updated_nft_supply)?;
    Ok(())
}

pub fn update_approvals(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    sender_addr: &Addr,
    spender_addr: &Addr,
    token_id: u128,
    // if add == false, remove. if add == true, remove then set with this expiration
    add: bool,
    expires: Option<Expiration>,
) -> Result<NftTokenInfo<NftExtension>, ContractError> {
    let mut nft = NFTS().load(storage, token_id)?;
    // ensure we have permissions
    assert_can_update_approvals(storage, block, &nft.owner, sender_addr)?;
    // update the approval list (remove any for the same spender before adding)
    nft.approvals.retain(|apr| apr.spender != *spender_addr);
    // only difference between approve and revoke
    if add {
        // reject expired data as invalid
        let expires = expires.unwrap_or_default();
        if expires.is_expired(block) {
            return Err(ContractError::Expired {});
        }
        let approval = Cw721BaseApproval {
            spender: spender_addr.clone(),
            expires,
        };
        nft.approvals.push(approval);
    }
    NFTS().save(storage, token_id, &nft)?;
    Ok(nft)
}

pub fn transfer_nft_helper(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    sender_addr: &Addr,
    recipient_addr: &Addr,
    token_id: u128,
) -> Result<NftTokenInfo<NftExtension>, ContractError> {
    let mut nft = NFTS().load(storage, token_id)?;
    // ensure we have permissions
    assert_can_send(storage, block, sender_addr, token_id)?;
    // set owner and remove existing approvals
    nft.owner = recipient_addr.clone();
    nft.approvals = vec![];
    NFTS().save(storage, token_id, &nft)?;
    Ok(nft)
}
