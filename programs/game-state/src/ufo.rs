use crate::{GameConfig, PlayerProfile};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    ed25519_program,
    sysvar::instructions::{load_current_index_checked, load_instruction_at_checked},
};

pub const UFO_COLLECTION: Pubkey = Pubkey::new_from_array([
    60, 17, 0, 121, 183, 68, 235, 72, 50, 114, 37, 199, 2, 200, 207, 253, 55, 230, 229, 0, 119,
    230, 72, 1, 150, 1, 194, 91, 219, 93, 61, 230,
]);
pub const UFO_DOMAIN: &[u8] = b"gamesol:ufo:devnet:v1";
pub const COMMANDER_DOMAIN: &[u8] = b"gamesol:commander:devnet:v1";
pub const UFO_COOLDOWN: i64 = 86_400;
pub const UFO_STAGE_DELAY: i64 = 15;
pub const UFO_MAX_LOOT: usize = 24;

#[account]
#[derive(InitSpace)]
pub struct UfoConfig {
    pub attestor: Pubkey,
    pub enabled: bool,
}

#[account]
#[derive(InitSpace)]
pub struct UfoUse {
    pub last_started: i64,
}

#[account]
#[derive(InitSpace, Default)]
pub struct CommanderIdentity {
    pub commander: Pubkey,
    pub nonce: u64,
    pub owner: Pubkey,
    pub asset: Pubkey,
    pub verified_at: i64,
}

#[derive(Accounts)]
pub struct SetCommanderIdentity<'info> {
    #[account(mut)]
    pub commander: Signer<'info>,
    #[account(seeds=[b"player_profile", commander.key().as_ref()], bump=player_profile.bump, constraint=player_profile.authority==commander.key())]
    pub player_profile: Account<'info, PlayerProfile>,
    #[account(seeds=[b"ufo_config"], bump)]
    pub config: Account<'info, UfoConfig>,
    #[account(init_if_needed, payer=commander, space=8+CommanderIdentity::INIT_SPACE, seeds=[b"commander_identity", commander.key().as_ref()], bump)]
    pub identity: Account<'info, CommanderIdentity>,
    /// CHECK: fixed instructions sysvar; ownership proof uses a separate domain.
    #[account(address=anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClearCommanderIdentity<'info> {
    pub commander: Signer<'info>,
    #[account(mut, seeds=[b"commander_identity", commander.key().as_ref()], bump, has_one=commander)]
    pub identity: Account<'info, CommanderIdentity>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, PartialEq, Eq, Debug)]
pub struct UfoLoot {
    pub kind: u8,
    pub doctrine: u8,
    pub trait_name: [u8; 32],
}

#[account]
#[derive(InitSpace)]
pub struct UfoLoadout {
    pub commander: Pubkey,
    pub nonce: u64,
    pub owner: Pubkey,
    pub asset: Pubkey,
    pub doctrine: u8,
    pub route: u8,
    pub accessory: [u8; 32],
    pub hair: [u8; 32],
    pub status: u8,
    pub stage: u8,
    pub mistakes: u8,
    pub booster: bool,
    pub next_action_at: i64,
    pub last_started: i64,
    pub completed: u64,
    pub equipped: u8,
    #[max_len(24)]
    pub loot: Vec<UfoLoot>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UfoProof {
    pub owner: Pubkey,
    pub asset: Pubkey,
    pub nonce: u64,
    pub doctrine: u8,
    pub route: u8,
    pub accessory: [u8; 32],
    pub hair: [u8; 32],
    pub expires: i64,
}

#[derive(Accounts)]
pub struct ConfigureUfo<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds=[b"game_config"], bump=game_config.bump, has_one=admin)]
    pub game_config: Account<'info, GameConfig>,
    #[account(init_if_needed, payer=admin, space=8+UfoConfig::INIT_SPACE, seeds=[b"ufo_config"], bump)]
    pub config: Account<'info, UfoConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proof: UfoProof)]
pub struct StartUfo<'info> {
    #[account(mut)]
    pub commander: Signer<'info>,
    #[account(seeds=[b"player_profile", commander.key().as_ref()], bump=player_profile.bump, constraint=player_profile.authority==commander.key())]
    pub player_profile: Account<'info, PlayerProfile>,
    #[account(seeds=[b"ufo_config"], bump)]
    pub config: Account<'info, UfoConfig>,
    #[account(init_if_needed, payer=commander, space=8+UfoLoadout::INIT_SPACE, seeds=[b"ufo_loadout", commander.key().as_ref()], bump)]
    pub loadout: Box<Account<'info, UfoLoadout>>,
    #[account(init_if_needed, payer=commander, space=8+UfoUse::INIT_SPACE, seeds=[b"ufo_asset_use", proof.asset.as_ref()], bump)]
    pub asset_use: Account<'info, UfoUse>,
    #[account(init_if_needed, payer=commander, space=8+UfoUse::INIT_SPACE, seeds=[b"ufo_owner_use", proof.owner.as_ref()], bump)]
    pub owner_use: Account<'info, UfoUse>,
    /// CHECK: fixed instructions sysvar; only validated Ed25519 precompiles are read.
    #[account(address=anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UfoAction<'info> {
    pub commander: Signer<'info>,
    #[account(seeds=[b"ufo_config"], bump)]
    pub config: Account<'info, UfoConfig>,
    #[account(mut, seeds=[b"ufo_loadout", commander.key().as_ref()], bump, has_one=commander)]
    pub loadout: Box<Account<'info, UfoLoadout>>,
    /// CHECK: fixed instructions sysvar, checked in verify_proof.
    #[account(address=anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct ManageUfo<'info> {
    pub commander: Signer<'info>,
    #[account(mut, seeds=[b"ufo_loadout", commander.key().as_ref()], bump, has_one=commander)]
    pub loadout: Box<Account<'info, UfoLoadout>>,
}

#[error_code(offset = 8000)]
pub enum UfoError {
    #[msg("UFO expeditions are disabled.")]
    Disabled,
    #[msg("Invalid or expired ownership attestation.")]
    InvalidProof,
    #[msg("Ownership proof already used. Refresh the loadout.")]
    Replay,
    #[msg("This commander, holding wallet or UFO is on expedition cooldown.")]
    Cooldown,
    #[msg("No matching active expedition.")]
    Mission,
    #[msg("The next encounter is not ready yet.")]
    NotReady,
    #[msg("Invalid encounter choice or booster unavailable.")]
    Choice,
    #[msg("Cosmetic inventory is full.")]
    InventoryFull,
    #[msg("That cosmetic has not been earned.")]
    Unearned,
}

pub fn proof_message(commander: Pubkey, proof: &UfoProof) -> Vec<u8> {
    scoped_proof_message(UFO_DOMAIN, commander, proof)
}

fn scoped_proof_message(domain: &[u8], commander: Pubkey, proof: &UfoProof) -> Vec<u8> {
    let mut message = domain.to_vec();
    message.extend_from_slice(crate::ID.as_ref());
    message.extend_from_slice(UFO_COLLECTION.as_ref());
    message.extend_from_slice(commander.as_ref());
    message.extend_from_slice(proof.owner.as_ref());
    message.extend_from_slice(proof.asset.as_ref());
    message.extend_from_slice(&proof.nonce.to_le_bytes());
    message.extend_from_slice(&[proof.doctrine, proof.route]);
    message.extend_from_slice(&proof.accessory);
    message.extend_from_slice(&proof.hair);
    message.extend_from_slice(&proof.expires.to_le_bytes());
    message
}

fn canonical_attestation(data: &[u8], attestor: Pubkey, message: &[u8]) -> bool {
    if data.len() != 112 + message.len() || data[0] != 1 || data[1] != 0 {
        return false;
    }
    let u16_at = |offset| u16::from_le_bytes([data[offset], data[offset + 1]]);
    u16_at(2) == 48
        && u16_at(4) == u16::MAX
        && u16_at(6) == 16
        && u16_at(8) == u16::MAX
        && u16_at(10) == 112
        && usize::from(u16_at(12)) == message.len()
        && u16_at(14) == u16::MAX
        && data[16..48] == attestor.to_bytes()
        && data[112..] == *message
}

fn verify_proof(
    config: &UfoConfig,
    commander: Pubkey,
    proof: &UfoProof,
    nonce: u64,
    ix: &AccountInfo,
    now: i64,
) -> Result<()> {
    verify_scoped_proof(UFO_DOMAIN, config, commander, proof, nonce, ix, now)
}

fn verify_scoped_proof(
    domain: &[u8],
    config: &UfoConfig,
    commander: Pubkey,
    proof: &UfoProof,
    nonce: u64,
    ix: &AccountInfo,
    now: i64,
) -> Result<()> {
    require!(config.enabled, UfoError::Disabled);
    require!(proof.nonce == nonce, UfoError::Replay);
    require!(
        proof.expires > now
            && proof.expires <= now.saturating_add(120)
            && (1..=6).contains(&proof.doctrine)
            && proof.route <= 5,
        UfoError::InvalidProof
    );
    require!(
        proof.owner != Pubkey::default() && proof.asset != Pubkey::default(),
        UfoError::InvalidProof
    );
    let message = scoped_proof_message(domain, commander, proof);
    let current = load_current_index_checked(ix)?;
    for index in 0..usize::from(current) {
        let instruction = load_instruction_at_checked(index, ix)?;
        if instruction.program_id == ed25519_program::ID
            && instruction.accounts.is_empty()
            && canonical_attestation(&instruction.data, config.attestor, &message)
        {
            return Ok(());
        }
    }
    err!(UfoError::InvalidProof)
}

fn update_identity(
    identity: &mut CommanderIdentity,
    commander: Pubkey,
    proof: &UfoProof,
    now: i64,
) -> Result<()> {
    require!(
        identity.commander == Pubkey::default() || identity.commander == commander,
        UfoError::InvalidProof
    );
    require!(identity.nonce == proof.nonce, UfoError::Replay);
    let next_nonce = identity.nonce.checked_add(1).ok_or(UfoError::Replay)?;
    identity.commander = commander;
    identity.nonce = next_nonce;
    identity.owner = proof.owner;
    identity.asset = proof.asset;
    identity.verified_at = now;
    Ok(())
}

pub fn set_commander_identity(ctx: Context<SetCommanderIdentity>, proof: UfoProof) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    verify_scoped_proof(
        COMMANDER_DOMAIN,
        &ctx.accounts.config,
        ctx.accounts.commander.key(),
        &proof,
        ctx.accounts.identity.nonce,
        &ctx.accounts.instructions.to_account_info(),
        now,
    )?;
    update_identity(
        &mut ctx.accounts.identity,
        ctx.accounts.commander.key(),
        &proof,
        now,
    )
}

fn clear_identity(identity: &mut CommanderIdentity) -> Result<()> {
    identity.nonce = identity.nonce.checked_add(1).ok_or(UfoError::Replay)?;
    identity.owner = Pubkey::default();
    identity.asset = Pubkey::default();
    identity.verified_at = 0;
    Ok(())
}

pub fn clear_commander_identity(ctx: Context<ClearCommanderIdentity>) -> Result<()> {
    // Keep the account and nonce so clearing cannot revive old selection proofs.
    clear_identity(&mut ctx.accounts.identity)
}

pub fn configure(ctx: Context<ConfigureUfo>, attestor: Pubkey, enabled: bool) -> Result<()> {
    require!(attestor != Pubkey::default(), UfoError::InvalidProof);
    ctx.accounts
        .config
        .set_inner(UfoConfig { attestor, enabled });
    Ok(())
}

fn cooldown_ready(last: i64, now: i64) -> bool {
    last == 0 || now >= last.saturating_add(UFO_COOLDOWN)
}

pub fn start(ctx: Context<StartUfo>, proof: UfoProof) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let loadout = &mut ctx.accounts.loadout;
    verify_proof(
        &ctx.accounts.config,
        ctx.accounts.commander.key(),
        &proof,
        loadout.nonce,
        &ctx.accounts.instructions.to_account_info(),
        now,
    )?;
    require!(loadout.status == 0, UfoError::Mission);
    require!(
        cooldown_ready(loadout.last_started, now)
            && cooldown_ready(ctx.accounts.asset_use.last_started, now)
            && cooldown_ready(ctx.accounts.owner_use.last_started, now),
        UfoError::Cooldown
    );
    if loadout.commander == Pubkey::default() {
        loadout.commander = ctx.accounts.commander.key();
        loadout.equipped = 255;
    }
    require_keys_eq!(
        loadout.commander,
        ctx.accounts.commander.key(),
        UfoError::Mission
    );
    loadout.nonce = loadout.nonce.checked_add(1).ok_or(UfoError::Replay)?;
    loadout.owner = proof.owner;
    loadout.asset = proof.asset;
    loadout.doctrine = proof.doctrine;
    loadout.route = proof.route;
    loadout.accessory = proof.accessory;
    loadout.hair = proof.hair;
    loadout.status = 1;
    loadout.stage = 0;
    loadout.mistakes = 0;
    loadout.booster = true;
    loadout.last_started = now;
    loadout.next_action_at = now.saturating_add(UFO_STAGE_DELAY);
    ctx.accounts.asset_use.last_started = now;
    ctx.accounts.owner_use.last_started = now;
    Ok(())
}

fn matching_mission(loadout: &UfoLoadout, proof: &UfoProof) -> Result<()> {
    require!(
        loadout.owner == proof.owner
            && loadout.asset == proof.asset
            && loadout.doctrine == proof.doctrine
            && loadout.route == proof.route,
        UfoError::Mission
    );
    Ok(())
}

// Each doctrine has a different safe action; Eyes rotate the route encounters.
pub fn safe_choice(doctrine: u8, route: u8, stage: u8) -> u8 {
    (doctrine - 1 + route + stage) % 3
}

pub fn advance_state(
    loadout: &mut UfoLoadout,
    choice: u8,
    use_booster: bool,
    now: i64,
) -> Result<()> {
    require!(loadout.status == 1 && loadout.stage < 3, UfoError::Mission);
    require!(now >= loadout.next_action_at, UfoError::NotReady);
    require!(
        choice < 3 && (!use_booster || loadout.booster),
        UfoError::Choice
    );
    let safe = choice == safe_choice(loadout.doctrine, loadout.route, loadout.stage);
    if use_booster {
        loadout.booster = false;
    }
    if safe || use_booster {
        loadout.stage += 1;
    } else {
        loadout.mistakes += 1;
    }
    loadout.status = if loadout.stage == 3 {
        2
    } else if loadout.mistakes >= 3 {
        3
    } else {
        1
    };
    loadout.next_action_at = now.saturating_add(UFO_STAGE_DELAY);
    Ok(())
}

pub fn advance(
    ctx: Context<UfoAction>,
    proof: UfoProof,
    choice: u8,
    use_booster: bool,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let loadout = &mut ctx.accounts.loadout;
    verify_proof(
        &ctx.accounts.config,
        ctx.accounts.commander.key(),
        &proof,
        loadout.nonce,
        &ctx.accounts.instructions.to_account_info(),
        now,
    )?;
    matching_mission(loadout, &proof)?;
    advance_state(loadout, choice, use_booster, now)?;
    loadout.nonce = loadout.nonce.checked_add(1).ok_or(UfoError::Replay)?;
    Ok(())
}

pub fn award_loot(loadout: &mut UfoLoadout) -> Result<()> {
    require!(loadout.status == 2 && loadout.stage == 3, UfoError::Mission);
    let candidates = [
        UfoLoot {
            kind: 0,
            doctrine: loadout.doctrine,
            trait_name: [0; 32],
        },
        UfoLoot {
            kind: 1,
            doctrine: 0,
            trait_name: loadout.accessory,
        },
        UfoLoot {
            kind: 2,
            doctrine: 0,
            trait_name: loadout.hair,
        },
    ];
    let next = candidates.into_iter().find(|item| {
        (item.kind == 0 || item.trait_name != [0; 32]) && !loadout.loot.contains(item)
    });
    if let Some(item) = next {
        require!(loadout.loot.len() < UFO_MAX_LOOT, UfoError::InventoryFull);
        loadout.loot.push(item);
    }
    loadout.completed = loadout.completed.checked_add(1).ok_or(UfoError::Replay)?;
    loadout.status = 0;
    Ok(())
}

pub fn claim(ctx: Context<UfoAction>, proof: UfoProof) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let loadout = &mut ctx.accounts.loadout;
    verify_proof(
        &ctx.accounts.config,
        ctx.accounts.commander.key(),
        &proof,
        loadout.nonce,
        &ctx.accounts.instructions.to_account_info(),
        now,
    )?;
    matching_mission(loadout, &proof)?;
    award_loot(loadout)?;
    loadout.nonce = loadout.nonce.checked_add(1).ok_or(UfoError::Replay)?;
    Ok(())
}

pub fn equip(ctx: Context<ManageUfo>, item: u8) -> Result<()> {
    require!(
        item == 255 || usize::from(item) < ctx.accounts.loadout.loot.len(),
        UfoError::Unearned
    );
    ctx.accounts.loadout.equipped = item;
    Ok(())
}

pub fn abort(ctx: Context<ManageUfo>) -> Result<()> {
    require!(ctx.accounts.loadout.status != 0, UfoError::Mission);
    ctx.accounts.loadout.status = 0;
    ctx.accounts.loadout.nonce = ctx
        .accounts
        .loadout
        .nonce
        .checked_add(1)
        .ok_or(UfoError::Replay)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn identity_proof() -> UfoProof {
        UfoProof {
            owner: Pubkey::new_unique(),
            asset: Pubkey::new_unique(),
            nonce: 0,
            doctrine: 1,
            route: 0,
            accessory: [0; 32],
            hair: [0; 32],
            expires: 120,
        }
    }

    #[test]
    fn commander_proof_is_bound_to_action_and_game_account() {
        let commander = Pubkey::new_unique();
        let proof = identity_proof();
        let message = scoped_proof_message(COMMANDER_DOMAIN, commander, &proof);
        assert_ne!(message, proof_message(commander, &proof));
        assert_ne!(
            message,
            scoped_proof_message(COMMANDER_DOMAIN, Pubkey::new_unique(), &proof)
        );
        let mut changed = proof.clone();
        changed.asset = Pubkey::new_unique();
        assert_ne!(
            message,
            scoped_proof_message(COMMANDER_DOMAIN, commander, &changed)
        );
        changed = proof.clone();
        changed.owner = Pubkey::new_unique();
        assert_ne!(
            message,
            scoped_proof_message(COMMANDER_DOMAIN, commander, &changed)
        );
    }

    #[test]
    fn commander_clear_preserves_replay_protection() {
        let commander = Pubkey::new_unique();
        let mut identity = CommanderIdentity::default();
        let mut proof = identity_proof();
        update_identity(&mut identity, commander, &proof, 1).unwrap();
        assert_eq!(identity.asset, proof.asset);
        assert_eq!(identity.verified_at, 1);
        assert!(update_identity(&mut identity, commander, &proof, 2).is_err());
        clear_identity(&mut identity).unwrap();
        assert_eq!(identity.commander, commander);
        assert_eq!(identity.nonce, 2);
        assert_eq!(identity.asset, Pubkey::default());
        assert!(update_identity(&mut identity, commander, &proof, 3).is_err());
        proof.nonce = 2;
        assert!(update_identity(&mut identity, Pubkey::new_unique(), &proof, 3).is_err());
        update_identity(&mut identity, commander, &proof, 3).unwrap();
        assert_eq!(identity.nonce, 3);
    }

    #[test]
    fn commander_nonce_exhaustion_does_not_modify_selection() {
        let commander = Pubkey::new_unique();
        let mut identity = CommanderIdentity {
            commander,
            nonce: u64::MAX,
            ..Default::default()
        };
        let mut proof = identity_proof();
        proof.nonce = u64::MAX;
        assert!(update_identity(&mut identity, commander, &proof, 1).is_err());
        assert_eq!(identity.asset, Pubkey::default());
        assert!(clear_identity(&mut identity).is_err());
        assert_eq!(identity.nonce, u64::MAX);
    }

    fn mission() -> UfoLoadout {
        UfoLoadout {
            commander: Pubkey::new_unique(),
            nonce: 0,
            owner: Pubkey::new_unique(),
            asset: Pubkey::new_unique(),
            doctrine: 1,
            route: 0,
            accessory: [0; 32],
            hair: [0; 32],
            status: 1,
            stage: 0,
            mistakes: 0,
            booster: true,
            next_action_at: 15,
            last_started: 1,
            completed: 0,
            equipped: 255,
            loot: vec![],
        }
    }
    #[test]
    fn booster_rescues_one_encounter_and_cannot_stack() {
        let mut state = mission();
        assert!(advance_state(&mut state, 1, true, 14).is_err());
        advance_state(&mut state, 1, true, 15).unwrap();
        assert_eq!(state.stage, 1);
        assert!(!state.booster);
        assert!(advance_state(&mut state, 2, true, 30).is_err());
    }
    #[test]
    fn loot_requires_completion_and_is_not_duplicated() {
        let mut state = mission();
        assert!(award_loot(&mut state).is_err());
        for stage in 0..3 {
            advance_state(&mut state, stage, false, 15 * (i64::from(stage) + 1)).unwrap();
        }
        award_loot(&mut state).unwrap();
        assert_eq!(state.loot.len(), 1);
        assert!(award_loot(&mut state).is_err());
        state.status = 2;
        award_loot(&mut state).unwrap();
        assert_eq!(state.loot.len(), 1);
    }
    #[test]
    fn transfer_limits_use_rolling_cooldowns() {
        assert!(!cooldown_ready(100, 101));
        assert!(!cooldown_ready(100, 86_499));
        assert!(cooldown_ready(100, 86_500));
    }
    #[test]
    fn attestation_parser_rejects_cross_instruction_offsets() {
        let signer = Pubkey::new_unique();
        let message = b"test";
        let mut data = vec![0; 116];
        data[0] = 1;
        for (offset, value) in [
            (2, 48u16),
            (4, u16::MAX),
            (6, 16),
            (8, u16::MAX),
            (10, 112),
            (12, 4),
            (14, u16::MAX),
        ] {
            data[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
        }
        data[16..48].copy_from_slice(signer.as_ref());
        data[112..].copy_from_slice(message);
        assert!(canonical_attestation(&data, signer, message));
        data[4] = 0;
        assert!(!canonical_attestation(&data, signer, message));
    }
}
