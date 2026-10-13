use anchor_lang::prelude::*;

use crate::state::{
    AllianceDecision, AllianceGovernanceConfig, AllianceJoinRequest, AllianceMembership,
    AllianceMetadata, AllianceState, AllianceTreasuryState, AuthorizedVault, FaucetClaim,
    GameConfig, GamesolGovernanceConfig, LazorPlayerProfile, PlanetCoordinates, PlanetOwnerIndex,
    PlanetOwnershipRegistry, PlanetState, PlayerProfile, PublicPlanetCoordinates,
    PublicPlanetState, QuestProgressState, QuestRewardTargetState, QuestState, StoreConfig,
    StorePurchaseState, SystemSovereignty, TutorialQuestRewardTargetState, VaultBackup,
};

pub const MAX_PLANET_NAME_LEN: usize = 32;
pub const MAX_MISSION_COLONY_NAME_LEN: usize = 32;
pub const MAX_ALLIANCE_NAME_LEN: usize = 32;
pub const MAX_ALLIANCE_TAG_LEN: usize = 3;
pub const MAX_ALLIANCE_IMAGE_URL_LEN: usize = 160;
pub const MAX_MISSIONS: usize = 4;
pub const MAX_COMBAT_ROUNDS: u8 = 6;
pub const MAX_RESOURCE_SETTLEMENT_SECONDS: i64 = 86_400;
pub const NEW_PLAYER_PROTECTION_SECONDS: i64 = 7 * 24 * 60 * 60;
pub const ATTACK_UNLOCK_SECONDS: i64 = 300;
pub const MARKET_UNLOCK_SECONDS: i64 = 300;
pub const ATTACK_LAUNCH_COOLDOWN_SECONDS: i64 = 60;
pub const TARGET_ATTACK_COOLDOWN_SECONDS: i64 = 30 * 60;
pub const MAX_PURCHASED_SHIELD_SECONDS: i64 = 7 * 24 * 60 * 60;
pub const DAILY_SHIELD_SECONDS: i64 = 6 * 60 * 60;
pub const WEEKLY_SHIELD_SECONDS: i64 = 24 * 60 * 60;
pub const MONTHLY_SHIELD_SECONDS: i64 = 3 * 24 * 60 * 60;
pub const MIN_ATTACK_COMBAT_POINTS: u64 = 1_000;
pub const MISSION_ATTACK: u8 = 1;
pub const MISSION_TRANSPORT: u8 = 2;
pub const MISSION_COLONIZE: u8 = 5;
pub const MISSION_ESPIONAGE: u8 = 6;
pub const ANTIMATTER_DECIMALS: u8 = 6;
pub const ANTIMATTER_SCALE: u64 = 1_000_000;
pub const ANTIMATTER_FAUCET_AMOUNT: u64 = 10_000 * ANTIMATTER_SCALE;
pub const ANTIMATTER_FAUCET_COOLDOWN_SECONDS: i64 = 24 * 60 * 60;
// Isolated sovereignty devnet economics. Production keeps the original
// 500 USDC / 100,000 ANTIMATTER alliance-creation threshold.
pub const ALLIANCE_CREATE_USDC_COST: u64 = 100_000_000;
pub const ALLIANCE_CREATE_ANTIMATTER_COST: u64 = 5_000 * ANTIMATTER_SCALE;
pub const ALLIANCE_CREATE_ANTIMATTER_BURN: u64 = ALLIANCE_CREATE_ANTIMATTER_COST / 2;
pub const ALLIANCE_CREATE_ANTIMATTER_TREASURY: u64 =
    ALLIANCE_CREATE_ANTIMATTER_COST - ALLIANCE_CREATE_ANTIMATTER_BURN;
pub const ALLIANCE_DEPOSIT_XP_PER_RESOURCE_UNIT: u64 = 1_000;
pub const ALLIANCE_DEPOSIT_XP_PER_ANTIMATTER_UNIT: u64 = ANTIMATTER_SCALE;
pub const ALLIANCE_DEPOSIT_VARIABLE_XP_MULTIPLIER: u64 = 10;
pub const ALLIANCE_BUILDING_MAX_LEVEL: u8 = 25;
pub const ALLIANCE_TRADE_DISCOUNT_BPS_PER_LEVEL: u64 = 100;
pub const ALLIANCE_DEFENSE_DISCOUNT_BPS_PER_LEVEL: u64 = 150;
pub const PROTOCOL_AUTHORITY: Pubkey = Pubkey::new_from_array([
    203, 132, 160, 224, 36, 85, 73, 7, 47, 237, 167, 190, 14, 187, 187, 93, 163, 241, 26, 147, 33,
    250, 136, 102, 231, 253, 14, 8, 203, 234, 209, 64,
]);
pub const PROTOCOL_ANTIMATTER_MINT: Pubkey = Pubkey::new_from_array([
    210, 124, 79, 139, 189, 97, 171, 121, 236, 30, 15, 224, 71, 28, 151, 137, 112, 205, 123, 216,
    200, 197, 217, 132, 30, 230, 156, 231, 135, 221, 136, 128,
]);
pub const STORE_USDC_MINT: Pubkey = Pubkey::new_from_array([
    59, 68, 44, 179, 145, 33, 87, 241, 58, 147, 61, 1, 52, 40, 45, 3, 43, 95, 254, 205, 1, 162,
    219, 241, 183, 121, 6, 8, 223, 0, 46, 167,
]);
pub const STARTING_METAL: u64 = 500;
pub const STARTING_CRYSTAL: u64 = 500;
pub const STARTING_DEUTERIUM: u64 = 100;
pub const BASE_STORAGE_CAP: u64 = 10_000;
pub const PLANET_COORDS_SPACE: usize = 8 + PlanetCoordinates::INIT_SPACE;
pub const PUBLIC_PLANET_COORDS_SPACE: usize = 8 + PublicPlanetCoordinates::INIT_SPACE;
pub const PLAYER_PROFILE_SPACE: usize = 8 + PlayerProfile::INIT_SPACE;
pub const LAZOR_PLAYER_PROFILE_SPACE: usize = 8 + LazorPlayerProfile::INIT_SPACE;
pub const LAZOR_PLAYER_SCHEMA_VERSION: u8 = 2;
pub const PLANET_OWNER_INDEX_SPACE: usize = 8 + PlanetOwnerIndex::INIT_SPACE;
pub const PLANET_OWNERSHIP_REGISTRY_SPACE: usize = 8 + PlanetOwnershipRegistry::INIT_SPACE;
pub const PLANET_STATE_SPACE: usize = 8 + PlanetState::INIT_SPACE;
pub const PUBLIC_PLANET_STATE_SPACE: usize = 8 + PublicPlanetState::INIT_SPACE;
pub const AUTHORIZED_VAULT_SPACE: usize = 8 + AuthorizedVault::INIT_SPACE;
pub const VAULT_BACKUP_SPACE: usize = 8 + VaultBackup::INIT_SPACE;
pub const GAME_CONFIG_SPACE: usize = 8 + GameConfig::INIT_SPACE;
pub const FAUCET_CLAIM_SPACE: usize = 8 + FaucetClaim::INIT_SPACE;
pub const STORE_CONFIG_SPACE: usize = 8 + StoreConfig::INIT_SPACE;
pub const STORE_PURCHASE_STATE_SPACE: usize = 8 + StorePurchaseState::INIT_SPACE;
pub const QUEST_STATE_SPACE: usize = 8 + QuestState::INIT_SPACE;
pub const QUEST_PROGRESS_STATE_SPACE: usize = 8 + QuestProgressState::INIT_SPACE;
pub const QUEST_REWARD_TARGET_STATE_SPACE: usize = 8 + QuestRewardTargetState::INIT_SPACE;
pub const TUTORIAL_QUEST_REWARD_TARGET_STATE_SPACE: usize =
    8 + TutorialQuestRewardTargetState::INIT_SPACE;
/// Upper bound for a voluntary rent contribution routed through the program.
/// The paymaster applies the stricter command- and simulation-level policy.
pub const MAX_SPONSORED_RENT_LAMPORTS: u64 = 50_000_000;
pub const ALLIANCE_STATE_SPACE: usize = 8 + AllianceState::INIT_SPACE;
pub const ALLIANCE_METADATA_SPACE: usize = 8 + AllianceMetadata::INIT_SPACE;
pub const ALLIANCE_TREASURY_SPACE: usize = 8 + AllianceTreasuryState::INIT_SPACE;
pub const ALLIANCE_MEMBERSHIP_SPACE: usize = 8 + AllianceMembership::INIT_SPACE;
pub const ALLIANCE_JOIN_REQUEST_SPACE: usize = 8 + AllianceJoinRequest::INIT_SPACE;
pub const SYSTEM_SOVEREIGNTY_SPACE: usize = 8 + SystemSovereignty::INIT_SPACE;
pub const GAMESOL_GOVERNANCE_CONFIG_SPACE: usize = 8 + GamesolGovernanceConfig::INIT_SPACE;
pub const ALLIANCE_GOVERNANCE_CONFIG_SPACE: usize = 8 + AllianceGovernanceConfig::INIT_SPACE;
pub const ALLIANCE_DECISION_SPACE: usize = 8 + AllianceDecision::INIT_SPACE;
pub const SYSTEM_SOVEREIGNTY_VERSION: u8 = 1;
pub const GAMESOL_GOVERNANCE_VERSION: u8 = 1;
// Version 2 certifies that the alliance configuration was initialized from or
// synchronized with the shared GAMESOL governance root. The instruction
// account layouts remain compatible with already-created Squads transactions.
pub const ALLIANCE_GOVERNANCE_VERSION: u8 = 2;
pub const ALLIANCE_DECISION_VERSION: u8 = 1;
pub const ALLIANCE_DECISION_CLAIM_SYSTEM: u8 = 0;
pub const ALLIANCE_DECISION_FUND_UPKEEP: u8 = 1;
pub const ALLIANCE_DECISION_SET_SPECIALIZATION: u8 = 2;
pub const MAX_ALLIANCE_DECISION_ACTION: u8 = ALLIANCE_DECISION_SET_SPECIALIZATION;
pub const MAX_ALLIANCE_DECISION_LIFETIME_SECONDS: i64 = 30 * 24 * 60 * 60;
pub const SYSTEM_SOVEREIGNTY_UPKEEP_SECONDS: i64 = 7 * 24 * 60 * 60;
// Devnet sovereignty experiment: keep every debit real while making the full
// proposal -> market -> execution loop testable from a fresh homeworld.
// Production retains the original 100k/50k/25k economy thresholds.
pub const SYSTEM_SOVEREIGNTY_CLAIM_METAL: u64 = 100;
pub const SYSTEM_SOVEREIGNTY_CLAIM_CRYSTAL: u64 = 50;
pub const SYSTEM_SOVEREIGNTY_CLAIM_DEUTERIUM: u64 = 25;
pub const SYSTEM_SOVEREIGNTY_UPKEEP_METAL: u64 = 25;
pub const SYSTEM_SOVEREIGNTY_UPKEEP_CRYSTAL: u64 = 15;
pub const SYSTEM_SOVEREIGNTY_UPKEEP_DEUTERIUM: u64 = 10;
pub const SYSTEM_SOVEREIGNTY_SPECIALIZATION_METAL: u64 = 20;
pub const SYSTEM_SOVEREIGNTY_SPECIALIZATION_CRYSTAL: u64 = 20;
pub const SYSTEM_SOVEREIGNTY_SPECIALIZATION_DEUTERIUM: u64 = 10;
pub const MAX_SYSTEM_SPECIALIZATION: u8 = 3;
pub const SYSTEM_SPECIALIZATION_INDUSTRIAL: u8 = 1;
pub const SYSTEM_SPECIALIZATION_RESEARCH: u8 = 2;
pub const SYSTEM_SPECIALIZATION_MILITARY: u8 = 3;
pub const INDUSTRIAL_BUILD_SPEED_BONUS_BPS: u16 = 1_000;
pub const RESEARCH_SPEED_BONUS_BPS: u16 = 1_500;
pub const MILITARY_BUILD_SPEED_BONUS_BPS: u16 = 1_500;
pub const BASE_ALLIANCE_MAX_MEMBERS: u16 = 5;
pub const ALLIANCE_MEMBERS_PER_LEVEL: u16 = 3;
pub const ALLIANCE_XP_UNIT: u64 = 1_000;
pub const MARKET_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    84, 163, 170, 61, 4, 172, 20, 4, 48, 103, 146, 235, 200, 253, 197, 56, 244, 102, 208, 21,
    151, 224, 4, 142, 30, 137, 144, 19, 11, 22, 57, 77,
]);
