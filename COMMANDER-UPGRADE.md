# Commander identity upgrade

Status: deployed and finalized on devnet on 2026-10-14. Not enabled in the frontend.

## Deployment evidence

- Source commit: `562c6cb69de089af2c775ba8022892acc4bbd8bf`.
- Successful Rust/SBF/isolated-validator run:
  https://github.com/SrMessiSOL/GAMESOL/actions/runs/38653841491
- Binary SHA256: `61d2043125e62db902a7b62f925be360ab2d2540871ad77c912a4cbc0002c164`.
- Finalized upgrade transaction:
  `q1sHk9UCyYJwn1pqqX8i5hTkESRf9K4DG726mzmB4gfyvAtMcYhGAz6HD6U2AGTtx44R9jSEGVXhMKWHk7AeWEf`.
- Upgrade authority remains `EhT4UUmsHyd41bZSGzj1W72yu2EptHk8sEvnK7aaNuBu`.
- Program extension: 23,384 bytes; temporary buffer closed after success.
- Fee payer net debit: 135,820,720 lamports (0.13582072 devnet SOL), within
  the approved 8.1 devnet SOL funding cap. No USDC or mainnet funds used.
- Existing commander loadout `EjqHypWAD6NWdUfXcwS4fkU7K8zsTDY75uDJWBPXiGLh`
  was unchanged before/after deployment, SHA256:
  `a3cd7c3acd249398c9a520be2dc7ba7f62ce257c9395c0ba706dbdb0f575972f`.
- The public RPC upload exhausted its retries. The same funded buffer was
  resumed successfully through the existing game's devnet RPC proxy; no second
  buffer deposit was made.

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

1. Completed: Rust unit tests, SBF build and isolated validator suite in the
   existing bounded cloud workflow. No heavy local Windows build was used.
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
6. Completed: verified live upgrade authority, artifact hash and account order;
   obtained explicit deployment approval. Any subsequent upgrade needs its own review.
7. After approval/deployment: real wallet select, refresh/new-browser restore,
   clear/reselect, changed ownership rejection, replay rejection, and unchanged
   active expedition/earned loot. No expedition launch needed to save an avatar.

## Current verification boundary

Rust unit tests, SBF build, and isolated-validator tests passed. The deployed
binary matches the tested artifact at finalized commitment. Frontend/server
integration helpers passed TypeScript and 159 local regression tests but have
not been published or wired into the live avatar picker. Real-player selection
and cross-browser persistence remain untested until that integration is complete.
