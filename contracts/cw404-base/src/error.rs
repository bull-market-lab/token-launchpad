use cosmwasm_std::{StdError, Uint128, Uint64};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    // ========================== ADMIN ==========================
    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Only admin can enable contract")]
    OnlyAdminCanEnableContract {},

    #[error("Only admin can disable contract")]
    OnlyAdminCanDisableContract {},

    #[error("Only admin can update config")]
    OnlyAdminCanUpdateConfig {},

    #[error("Only admin can call this function: {function:?}")]
    OnlyAdminCanCallThisFunction { function: String },

    #[error("Only member contract can setup distribution for new membership")]
    OnlyMembershipContractCanSetupDistributionForNewMembership {},

    #[error("Only member contract can setup distribution for new member")]
    OnlyMembershipContractCanSetupDistributionForNewMember {},

    #[error("Only distribute allowlist addresses can distribute")]
    OnlyDistributeAllowlistAddressesCanDistribute {},

    #[error("Only member contract can update user pending reward")]
    OnlyMembershipContractCanUpdateUserPendingReward {},

    #[error("Cannot distribute before setup distribution")]
    CannotDistributeBeforeSetupDistribution {},

    #[error("Cannot update pending reward before setup distribution")]
    CannotUpdatePendingRewardBeforeSetupDistribution {},

    #[error("Only admin can update distribution caller allowlist")]
    OnlyAdminCanAddUpdateDistributionCallerAllowlist {},

    #[error("Address not in distribution caller allowlist")]
    AddressNotInDistributionCallerAllowlist {},

    #[error("Contract disabled for non admin")]
    ContractDisabledForNonAdmin {},

    // ========================== USER ==========================
    #[error("Distribution already setup for membership issuer")]
    DistributionAlreadySetupForMembershipIssuer {},

    #[error("Global indices and effective total weight already setup for membership issuer")]
    GlobalIndicesAlreadySetupForMembershipIssuer {},

    #[error("Only membership issuer can update its trading fee config")]
    OnlyMembershipIssuerCanUpdateItsTradingFeeConfig {},

    #[error("Only membership issuer can update its trading fee percentage of membership")]
    OnlyUserCanUpdateItsOwnConfig {},

    #[error("Only membership issuer can update its ask fee percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsAskFeePercentageOfMembership {},

    #[error("Only membership issuer can update its ask fee to creator percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsAskFeeToCreatorPercentageOfMembership {},

    #[error("Only membership issuer can update its reply fee percentage of membership")]
    OnlyMembershipIssuerCanUpdateItsReplyFeePercentageOfMembership {},

    #[error("User not exist")]
    UserNotExist {},

    #[error("User already registered membership")]
    UserAlreadyEnabledMembership {},

    #[error("User already verified social media")]
    UserAlreadyVerifiedSocialMedia {},

    #[error("User has not registered membership")]
    UserHasNotEnabledMembership {},

    #[error("User cannot register membership before its social media handle is verified")]
    UserCannotEnableMembershipBeforeSocialMediaHandleVerified {},

    #[error("Cannot claim reward before setup distribution")]
    CannotClaimRewardBeforeSetupDistribution {},
    // ========================== OTHERS ==========================
    #[error("Exceed query limit: given {given:?}, limit {limit:?}")]
    ExceedQueryLimit { given: Uint64, limit: Uint64 },

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Expect exactly one denom unit, received: {count:?}")]
    ExpectExactlyOneDenomUnit { count: Uint64 },

    #[error("Max base denom (FT in smallest unit) supply reached: current supply {current_base_denom_supply:?}, max supply {max_base_denom_supply:?}, mint amount {mint_amount:?}")]
    MaxBaseDenomSupplyReached {
        current_base_denom_supply: Uint128,
        max_base_denom_supply: Uint128,
        mint_amount: Uint128,
    },

    #[error("Max NFT supply reached: current supply {current_nft_supply:?}, max supply {max_nft_supply:?}, mint amount {mint_amount:?}")]
    MaxNftSupplyReached {
        current_nft_supply: Uint64,
        max_nft_supply: Uint64,
        mint_amount: Uint64,
    },

    #[error("Token ID {token_id:?} already in use")]
    TokenIdAlreadyInUse { token_id: Uint64 },

    #[error("No access to send NFT")]
    NoAccessToSend {},

    #[error("No access to approval NFT")]
    NoAccessToApproval {},

    #[error("Cannot mint zero amount")]
    CannotMintZeroAmount {},

    #[error("Cannot burn more NFT than owned, available: {available:?}, try to burn: {try_to_burn:?}")]
    CannotBurnMoreNftThanOwned {
        available: Uint64,
        try_to_burn: Uint64,
    },

    #[error("Expired")]
    Expired {},

    #[error("Cannot parse token id {value:?} from string to Uint64")]
    CannotParseTokenIdFromStringToUint64 { value: String },
}
