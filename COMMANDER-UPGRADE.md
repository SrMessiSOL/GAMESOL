# Commander identity upgrade - preparation only

Status: local draft. Not deployed, not proposed, not enabled in the frontend.

Target program: J6rNikvM7XLs6REtbbhMhe8NVd6Nfqa1C7PDfgHSDFkq (devnet).
Do not use the legacy FJGx program or its governance configuration.
Read the live ProgramData upgrade authority before preparing any executor payload;
historical Squads membership is not proof of current program authority.

## Contract

- `set_commander_identity(UfoProof)` requires the game commander signer, its
  player profile, enabled UFO attestor configuration, and a prior canonical
  Ed25519 ownership attestation.
- PDA: `["commander_identity", commander]`. Stores commander, independent nonce,
  holding wallet, asset ID, and last verified timestamp. Total account size: 120 bytes.
- Proof domain: `gamesol:commander:devnet:v1`. Existing collection, program ID,
  commander, owner, asset, nonce, traits and expiry remain signed. Maximum age
  is governed by a future expiry no more than 120 seconds away.
- `clear_commander_identity` needs only the matching commander signer. It keeps
  the account and increments the nonce, preventing replay after clearing.
- Neither instruction accesses expedition loadouts, inventory, loot or cooldowns.
  Existing layouts and expedition proof domain stay unchanged.
- This record is a verified-at-selection identity, not perpetual ownership proof.
  Fresh mainnet ownership must still gate utility after an NFT transfer.

## Release gates

1. Run Rust unit tests, SBF build and isolated validator suite in the existing
   bounded cloud workflow. No heavy local Windows build is needed.
2. Extend the server ownership challenge/attestation flow with an explicit
   commander-selection action, the new domain and identity nonce. Recheck mainnet
   collection membership and ownership; never issue an expedition proof instead.
3. Add a client decoder with discriminator, account-owner, length, PDA and
   commander checks. Restore the identity on login and then resolve real NFT
   metadata. A cleared record must not fall back to an old expedition avatar.
4. Wire explicit Save/Clear transactions and honest pending/error states. Do not
   silently save locally while presenting the selection as persisted on-chain.
5. Review sponsor/session instruction allowlists and transaction size. Do not
   grant broad Ed25519 or arbitrary signing permissions.
6. Verify live upgrade authority; prepare artifact hash, IDL and account-order
   review. Obtain separate permission before publishing, proposing or deploying.
7. After approval/deployment: real wallet select, refresh/new-browser restore,
   clear/reselect, changed ownership rejection, replay rejection, and unchanged
   active expedition/earned loot. No expedition launch needed to save an avatar.

## Current verification boundary

Rust source parsing/formatting and JavaScript syntax checks can run locally.
New Rust unit and validator tests are prepared, not yet executed. No claim of
compiled SBF, validator success, live persistence or deployment is made.
