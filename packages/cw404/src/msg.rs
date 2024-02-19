use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Coin, Empty, Uint128, Uint64};
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
    pub subdenom: String,
    pub max_denom_supply: Uint64,
    pub denom_metadata: Metadata,
}

// ========== execute ==========

#[cw_serde]
pub enum ExecuteMsg {
    // ========== FT functions ==========
    ChangeAdmin {
        new_admin_addr: String,
    },
    // Can only mint token to admin account
    MintTokens {
        amount: Uint128,
    },
    // Can only burn token from admin account
    BurnTokens {
        amount: Uint128,
    },
    SendTokens {
        amount: Uint128,
        recipient_addr: String,
    },
    ForceTransfer {
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
pub struct FullDenomResponse {
    pub full_denom: String,
}

#[cw_serde]
pub struct AdminResponse {
    pub admin_addr: String,
}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    // ========== FT functions ==========
    #[returns(FullDenomResponse)]
    FullDenom {},
    #[returns(AdminResponse)]
    Admin {},
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
// TODO: implement this message
#[cw_serde]
pub enum MigrateMsg {}

// ========== sudo ==========
// TODO: implement this message
#[cw_serde]
pub enum SudoMsg {
    TrackBeforeSend {
        from: String,
        to: String,
        amount: Coin,
    },
}
