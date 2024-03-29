use cosmwasm_std::{StdError, Uint128};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Duplicate mint group {name:?}")]
    DuplicateMintGroup { name: String },

    #[error("Mint group {name:?} not started")]
    MintGroupNotStarted { name: String },

    #[error("Mint group {name:?} ended")]
    MintGroupEnded { name: String },

    #[error("Mint amount {mint_amount:?} exceeds max amount per mint {max_base_denom_amount_per_mint:?} for mint group: {name:?}")]
    MintAmountExceedsMaxAmountPerMint {
        name: String,
        mint_amount: Uint128,
        max_base_denom_amount_per_mint: Uint128,
    },

    #[error(
        "Insufficient funds to mint, required: {required:?}, paid (already deducted fee paid to launchpad): {paid:?}"
    )]
    InsufficientFundsToMint { required: Uint128, paid: Uint128 },

    #[error("Merkle proof required for mint group {name:?}")]
    MerkleProofRequiredForMintGroup { name: String },

    #[error("Invalid merkle proof for mint group {name:?}")]
    InvalidMerkleProofForMintGroup { name: String },

    #[error("Only admin or minter can mint")]
    OnlyAdminOrMinterCanMint {},

    #[error("Only admin can call this function: {function:?}")]
    OnlyAdminCanCallThisFunction { function: String },

    #[error("Only admin can call this function: {function:?} but contract has no admin set")]
    OnlyAdminCanCallThisFunctionButContractHasNoAdmin { function: String },

    #[error("Max base denom (FT in smallest unit) supply reached: current supply {current_base_denom_supply:?}, max supply {max_base_denom_supply:?}, mint amount {mint_amount:?}")]
    MaxBaseDenomSupplyReached {
        current_base_denom_supply: Uint128,
        max_base_denom_supply: Uint128,
        mint_amount: Uint128,
    },

    #[error("NFT Token ID {nft_token_id:?} already in use")]
    NftTokenIdAlreadyInUse { nft_token_id: Uint128 },

    #[error("No access to send NFT because grant expired")]
    NoAccessToSendNftCauseGrantExpired {},

    #[error("No access to send NFT because grant not found")]
    NoAccessToSendNftCauseGrantNotFound {},

    #[error("No access to approve NFT because grant expired")]
    NoAccessToApproveNftCauseGrantExpired {},

    #[error("No access to approval NFT because grant not found")]
    NoAccessToApproveNftCauseGrantNotFound {},

    #[error("Cannot mint zero amount")]
    CannotMintZeroAmount {},

    #[error("Cannot burn more NFT than owned, available: {available:?}, try to burn: {try_to_burn:?}")]
    CannotBurnMoreNftThanOwned {
        available: Uint128,
        try_to_burn: Uint128,
    },

    #[error("Expired")]
    Expired {},

    #[error(
        "Error getting contract address from CW404 contract instantiation"
    )]
    ErrorGettingContractAddrFromInstantiateReplyOfCw404Contract {},

    #[error(
        "Error getting collection creator address from CW404 contract instantiation"
    )]
    ErrorGettingCreatorAddrFromInstantiateReplyOfCw404Contract {},

    #[error("Cannot parse token id {value:?} from string to Uint64")]
    CannotParseTokenIdFromStringToUint64 { value: String },

    #[error("Error getting event from CW404 contract instantiation")]
    ErrorGettingEventFromInstantiateReplyOfCw404Contract {},

    #[error("Mint group not found, name: {name:?}")]
    MintGroupNotFound { name: String },

    #[error("Insufficient funds to create collection through launchpad, paid: {paid:?}, launchpad required: {required:?}")]
    FundsMisMatchToCreateCw404Collection { paid: Uint128, required: Uint128 },

    #[error("Insufficient funds to create coin through launchpad, paid: {paid:?}, launchpad required: {required:?}")]
    InsufficientFundsToCreateCoin { paid: Uint128, required: Uint128 },

    #[error("Insufficient funds to mint NFT through launchpad, paid: {paid:?}, launchpad required: {required:?}")]
    InsufficientFundsToMintNft { paid: Uint128, required: Uint128 },

    #[error("Collection already exists {collection_addr:?}")]
    CollectionAlreadyExists { collection_addr: String },

    #[error("Coin already exists {coin_addr:?}")]
    CoinAlreadyExists { coin_addr: String },

    #[error("Cannot convert base denom to base subdenom {base_denom:?}")]
    CannotConvertBaseDenomToBaseSubdenom { base_denom: String },

    #[error("Error create pair in astroport {err:?}")]
    ErrorCreatePairInAstroport { err: String },

    #[error("Cannot find create pair event from astroport reply")]
    CannotFindCreatePairEventFromAstroportReply {},

    #[error("Cannot find pair from astroport reply")]
    CannotFindPairFromAstroportReply {},

    #[error("Cannot find created token from astroport reply")]
    CannotFindCreatedTokenFromAstroportReply {},

    #[error("Cannot find subdenom in tokens {subdenom:?}")]
    CannotFindSubdenomToken { subdenom: String },

    #[error("Cannot find seed liquidity config")]
    CannotFindSeedLiquidityConfig {},

    #[error("Unknown reply ID {reply_id:?}")]
    UnknownReplyId { reply_id: u64 },
}
