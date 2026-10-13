# UFO utility, devnet implementation

Mission state, cooldowns, booster charges, earned loot and equipped loot are stored in new game-program PDAs. No database or browser-stored reward ledger is used. Existing planet account layouts are unchanged.

## Trust boundary

Crypto UFOs is the mainnet collection `53UVubjHQpC4RmUnDGU1PV3f2bYFk6GcWb3SgtYFMHTb`; the game remains devnet. Mainnet DAS cannot be invoked from a devnet instruction. This version therefore uses an explicitly trusted, game-admin-configured Ed25519 ownership attestor. It is not trustless mainnet cNFT verification.

The holder explicitly signs a site-, commander-, asset-, nonce- and expiry-bound message. This authorizes attestations for one hour, not NFT transfers or delegation. The issuer rechecks current mainnet collection ownership on every privileged action. It signs a 90-second proof bound to this devnet domain, game program, collection, commander, holder, asset, on-chain nonce, doctrine and traits. No client-supplied traits or counters authorize rewards.

The program requires the game commander to sign the actual instruction. A Lazor Quick Play session can authorize it without another passkey prompt. It accepts only a canonical, already-executed, top-level Ed25519 precompile for the configured issuer and exact proof bytes. Proof age is capped at 120 seconds; an NFT transfer within this short attestation window is a known trust/freshness limitation.

Do not reuse this domain or issuer for a mainnet deployment. Mainnet activation needs a separate chain-bound verifier design and audit.

## Accounts And Rules

- `ufo_config`: issuer and enabled flag, managed only by the existing GameConfig admin. There is no resource or token authority in the issuer role.
- `ufo_loadout + commander`: monotonic proof nonce, one active mission, frozen launch traits, three encounter stages, one booster charge, cooldown, completion count, at most 24 unique cosmetics and equipped index.
- `ufo_asset_use + asset` and `ufo_owner_use + holder`: rolling 24-hour start limits alongside the commander limit. Moving an NFT or changing commanders does not reset these accounts. No account-close instruction exists.
- Successful choices progress the expedition. A single Body-specific booster can rescue an unsafe choice. Three unrescued hazards fail the mission. Each action waits 15 seconds of on-chain Clock time.
- Claim requires stage three, fresh ownership attestation, matching launch asset/holder/doctrine/route, and the next nonce. Claim closes the mission atomically. It cannot be claimed twice.
- Rewards progress through a Body cosmetic, actual Accessory insignia, and actual Hair crest. Duplicates do not increase inventory; rarity does not affect rewards. Inventory full fails rather than discarding an item.
- Earned cosmetics remain with the game commander after an NFT sale. New utility requires current ownership. Abort and cosmetic equip need only the game commander, so a transferred NFT does not strand the account.
- No resource production, combat, token payout, NFT mint, marketplace, ICO or treasury effects.

## Release Gates

Backend commit `85c2440` passed host tests, the cloud SBF/stack gate and the isolated-validator expedition/booster/claim/equip rehearsal. Its exact CI binary is now deployed to the existing devnet game ID. The user signed Squads v3 upgrade and activation proposals; all six transactions finalized successfully. The configured issuer is enabled on devnet. The game ID, planet layouts and upgrade authority are unchanged.

The real holding-wallet and Lazor CPI expedition loop is a separate release check; do not treat the isolated-validator rehearsal or configuration activation as proof of completed real-player gameplay. Mainnet remains disabled. Transaction evidence is in the frontend repository's `docs/UFO_DEVNET_ACTIVATION_PROOF.md` on branch `feat/crypto-ufos-wallet-link`.

The cloud SBF gate detected oversized frames in Anchor 0.30 account initialization and BLAKE3 1.8.4's digest 0.11 dependency. The workspace moves to Anchor 0.31.1's init-closure stack fix, with BLAKE3 pinned to 1.8.3. Default eight-byte discriminators and existing account layouts are unchanged. Both game and market host suites must pass; the market program is not deployed by this feature.

The frontend issuer requires server-only `GAMESOL_UFO_ATTESTOR_KEY` (base64 64-byte Ed25519 keypair), existing `GAMESOL_NFT_LINK_SECRET`, mainnet DAS RPC and a devnet game RPC. A server key authorizes only bounded cosmetic/PvE eligibility; progress and rewards remain on-chain. Do not paste keys in chat or commit them.
