use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Coin, Empty, Uint128};
use cw721::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse,
    ContractInfoResponse, NftInfoResponse, NumTokensResponse, OperatorResponse,
    OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cw_utils::Expiration;
use osmosis_std::types::cosmos::bank::v1beta1::Metadata;

// ========== instantiate ==========

#[cw_serde]
pub struct InstantiateMsg {
    pub admin_addr: String,
    pub max_nft_supply: Uint128,
    // e.g. subdenom = atom, then base subdenom is uatom,
    // denom is factory/contract_addr/atom, base denom is factory/contract_addr/uatom
    // 1 atom = 1_000_000 uatom, 1 atom = 1 atom NFT,
    pub subdenom: String,
    pub denom_description: String,
    pub denom_name: String,
    pub denom_symbol: String,
    pub denom_uri: String,
    pub denom_uri_hash: String,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    // ========== FT functions ==========
    ChangeAdmin {
        new_admin_addr: String,
    },
    // Can only mint token to admin account
    MintFt {
        /// amount is in base denom
        amount: Uint128,
    },
    // Can only burn token from admin account
    BurnFt {
        /// amount is in base denom
        amount: Uint128,
    },
    SendFt {
        /// amount is in base denom
        amount: Uint128,
        recipient_addr: String,
    },
    ForceTransferFt {
        /// amount is in base denom
        amount: Uint128,
        from: String,
        to: String,
    },
    // ========== NFT functions ==========
    // ========== cw721 ==========
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke {
        spender: String,
        token_id: String,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll {
        operator: String,
    },
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft {
        recipient: String,
        token_id: String,
    },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Burn an NFT the sender has access to
    Burn {
        token_id: String,
    },
}

// ========== query ==========

#[cw_serde]
pub struct AdminResponse {
    pub admin_addr: String,
}

#[cw_serde]
pub struct DenomMetadataResponse {
    pub metadata: Metadata,
}

#[cw_serde]
pub struct SupplyResponse {
    pub current_nft_supply: Uint128,
    pub max_nft_supply: Uint128,
    pub current_ft_supply: Uint128,
    pub max_ft_supply: Uint128,
}

#[cw_serde]
pub struct BalanceResponse {
    /// balance in NFT which is equal to 10 ** exponent base denom
    pub nft_balance: Uint128,
    /// balance in base denom
    pub ft_balance: Uint128,
}

#[cw_serde]
pub struct RecycledNftTokenIdsResponse {
    pub recycled_nft_token_ids: Vec<Uint128>,
}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    // ========== custom functions ==========
    #[returns(AdminResponse)]
    Admin {},
    #[returns(RecycledNftTokenIdsResponse)]
    RecycledNftTokenIds {},
    // ========== FT functions ==========
    #[returns(DenomMetadataResponse)]
    DenomMetadata {},
    #[returns(SupplyResponse)]
    Supply {},
    #[returns(BalanceResponse)]
    Balance { owner: String },
    // ========== NFT functions ==========
    /// Return the owner of the given token, error if token does not exist
    #[returns(OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    #[returns(ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    /// Return approvals that a token has
    #[returns(ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Return approval of a given operator for all tokens of an owner, error if not set
    #[returns(OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    #[returns(OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(NumTokensResponse)]
    NumTokens {},
    /// With MetaData Extension.
    /// Returns top-level metadata about the contract: `ContractInfoResponse`
    #[returns(ContractInfoResponse)]
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract: `NftInfoResponse`
    #[returns(NftInfoResponse<Empty>)]
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients: `AllNftInfo`
    #[returns(AllNftInfoResponse<Empty>)]
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

// ========== migrate ==========
#[cw_serde]
pub enum MigrateMsg {}

// ========== sudo ==========
#[cw_serde]
pub enum SudoMsg {
    TrackBeforeSend {
        from: String,
        to: String,
        amount: Coin,
    },
    BlockBeforeSend {
        from: String,
        to: String,
        amount: Coin,
    },
}
