use crate::{
    state::{CURRENT_NFT_SUPPLY, NFTS, NFT_OPERATORS},
    util::nft::humanize_approvals,
};
use cosmwasm_std::{
    Addr, BlockInfo, Deps, Empty, Env, Order, StdError, StdResult, Storage,
    Uint64,
};
use cw721::{
    AllNftInfoResponse, Approval, ApprovalResponse, ApprovalsResponse,
    ContractInfoResponse, NftInfoResponse, NumTokensResponse, OperatorResponse,
    OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cw_storage_plus::Bound;
use cw_utils::{maybe_addr, Expiration};
use osmosis_std::types::cosmos::bank::v1beta1::Metadata;

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;

pub fn query_nft_owner(
    storage: &dyn Storage,
    block: &BlockInfo,
    token_id: Uint64,
    include_expired: Option<bool>,
) -> StdResult<OwnerOfResponse> {
    let nft = NFTS().load(storage, token_id.u64())?;
    let approvals =
        humanize_approvals(&block, &nft, include_expired.unwrap_or(false));
    Ok(OwnerOfResponse {
        owner: nft.owner.to_string(),
        approvals,
    })
}

pub fn query_nft_approval(
    storage: &dyn Storage,
    block: &BlockInfo,
    token_id: Uint64,
    spender: String,
    include_expired: Option<bool>,
) -> StdResult<ApprovalResponse> {
    let nft = NFTS().load(storage, token_id.u64())?;

    // token owner has absolute approval
    if nft.owner == spender {
        let approval = cw721::Approval {
            spender: nft.owner.to_string(),
            expires: Expiration::Never {},
        };
        return Ok(ApprovalResponse { approval });
    }

    let filtered: Vec<_> = nft
        .approvals
        .into_iter()
        .filter(|t| t.spender == spender)
        .filter(|t| {
            include_expired.unwrap_or(false) || !t.expires.is_expired(block)
        })
        .map(|a| cw721::Approval {
            spender: a.spender,
            expires: a.expires,
        })
        .collect();

    if filtered.is_empty() {
        return Err(StdError::not_found("Approval not found"));
    }
    // we expect only one item
    let approval = filtered[0].clone();

    Ok(ApprovalResponse { approval })
}

pub fn query_nft_approvals(
    storage: &dyn Storage,
    block: &BlockInfo,
    token_id: Uint64,
    include_expired: Option<bool>,
) -> StdResult<ApprovalsResponse> {
    let nft = NFTS().load(storage, token_id.u64())?;
    let approvals: Vec<_> = nft
        .approvals
        .into_iter()
        .filter(|t| {
            include_expired.unwrap_or(false) || !t.expires.is_expired(block)
        })
        .map(|a| cw721::Approval {
            spender: a.spender,
            expires: a.expires,
        })
        .collect();

    Ok(ApprovalsResponse { approvals })
}

pub fn query_nft_operator(
    storage: &dyn Storage,
    block: &BlockInfo,
    owner_addr: &Addr,
    operator_addr: &Addr,
    include_expired: Option<bool>,
) -> StdResult<OperatorResponse> {
    let info =
        NFT_OPERATORS.may_load(storage, (&owner_addr, &operator_addr))?;

    if let Some(expires) = info {
        if !include_expired.unwrap_or(false) && expires.is_expired(block) {
            return Err(StdError::not_found("Approval not found"));
        }

        return Ok(OperatorResponse {
            approval: cw721::Approval {
                spender: operator_addr.to_string(),
                expires,
            },
        });
    }

    Err(StdError::not_found("Approval not found"))
}

pub fn query_all_nfts_operators(
    deps: Deps,
    block: &BlockInfo,
    owner: String,
    include_expired: Option<bool>,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<OperatorsResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_addr.as_ref().map(Bound::exclusive);

    let owner_addr = deps.api.addr_validate(&owner)?;
    let res: StdResult<Vec<_>> = NFT_OPERATORS
        .prefix(&owner_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .filter(|r| {
            include_expired.unwrap_or(false)
                || r.is_err()
                || !r.as_ref().unwrap().1.is_expired(block)
        })
        .take(limit)
        .map(|item| {
            item.map(|(spender, expires)| Approval {
                spender: spender.to_string(),
                expires,
            })
        })
        .collect();
    Ok(OperatorsResponse { operators: res? })
}

pub fn query_nft_num_tokens(
    storage: &dyn Storage,
) -> StdResult<NumTokensResponse> {
    let count = CURRENT_NFT_SUPPLY.load(storage)?.u64();
    Ok(NumTokensResponse { count })
}

pub fn query_nft_contract_info(
    metadata: Metadata,
) -> StdResult<ContractInfoResponse> {
    Ok(ContractInfoResponse {
        name: metadata.name,
        symbol: metadata.symbol,
    })
}

pub fn query_nft_info(
    deps: Deps,
    token_id: Uint64,
) -> StdResult<NftInfoResponse<Empty>> {
    let nft = NFTS().load(deps.storage, token_id.u64())?;
    Ok(NftInfoResponse {
        token_uri: nft.token_uri,
        extension: Empty {},
    })
}

pub fn query_all_nft_infos(
    deps: Deps,
    env: Env,
    token_id: Uint64,
    include_expired: Option<bool>,
) -> StdResult<AllNftInfoResponse<Empty>> {
    let nft = NFTS().load(deps.storage, token_id.u64())?;
    Ok(AllNftInfoResponse {
        access: OwnerOfResponse {
            owner: nft.owner.to_string(),
            approvals: humanize_approvals(
                &env.block,
                &nft,
                include_expired.unwrap_or(false),
            ),
        },
        info: NftInfoResponse {
            token_uri: nft.token_uri,
            extension: Empty {},
        },
    })
}

pub fn query_nfts(
    deps: Deps,
    owner_addr: &Addr,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let nft_ids: Vec<String> = NFTS()
        .idx
        .owner
        .prefix(owner_addr.clone())
        .keys(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|k| k.to_string()))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(TokensResponse { tokens: nft_ids })
}

pub fn query_all_nfts(
    storage: &dyn Storage,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let nft_ids: Vec<String> = NFTS()
        .range(storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|(k, _)| k.to_string()))
        .collect::<StdResult<Vec<_>>>()?;

    Ok(TokensResponse { tokens: nft_ids })
}
