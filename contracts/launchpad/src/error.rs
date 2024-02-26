use cosmwasm_std::{StdError, Uint128};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Only admin can call this function: {function:?}")]
    OnlyAdminCanCallThisFunction { function: String },

    #[error("Cannot mint zero amount")]
    CannotMintZeroAmount {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Error getting event from CW404 contract instantiation")]
    ErrorGettingEventFromInstantiateReplyOfCw404Contract {},

    #[error(
        "Error getting contract address from CW404 contract instantiation"
    )]
    ErrorGettingContractAddrFromInstantiateReplyOfCw404Contract {},

    #[error(
        "Error getting collection creator address from CW404 contract instantiation"
    )]
    ErrorGettingCreatorAddrFromInstantiateReplyOfCw404Contract {},

    #[error("Unknown reply ID {reply_id:?}")]
    UnknownReplyId { reply_id: u64 },

    #[error("Collection already exists {collection_addr:?}")]
    CollectionAlreadyExists { collection_addr: String },

    #[error("Insufficient funds to create collection through launchpad, paid: {paid:?}, launchpad required: {required:?}")]
    InsufficientFundsToCreateCollection { paid: Uint128, required: Uint128 },

    #[error("Insufficient funds to mint NFT through launchpad, paid: {paid:?}, launchpad required: {required:?}")]
    InsufficientFundsToMintNft { paid: Uint128, required: Uint128 },
}
