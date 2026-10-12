// Local-validator only: generated test identities, no mainnet NFT or user wallet.
const assert=require('node:assert/strict');
const crypto=require('node:crypto');
const fs=require('node:fs');
const path=require('node:path');
const {Connection,Keypair,PublicKey,SystemProgram,Ed25519Program,Transaction,TransactionInstruction,SYSVAR_INSTRUCTIONS_PUBKEY,sendAndConfirmTransaction}=require('@solana/web3.js');
const PROGRAM=new PublicKey('FJGxh6SKgNoTVzHj98oBsC2oaEy8ovadVJf8rDUNaEHb');
const COLLECTION=new PublicKey('53UVubjHQpC4RmUnDGU1PV3f2bYFk6GcWb3SgtYFMHTb');
const rpc=new Connection('http://127.0.0.1:8899','confirmed');
const directory=process.env.UFO_TEST_FIXTURE_DIR;
if(!directory)throw new Error('An explicit cloud test fixture directory is required.');
const preparing=process.argv.includes('--prepare');
const saved=preparing?null:JSON.parse(fs.readFileSync(path.join(directory,'identities.json'),'utf8'));
const signer=name=>saved?Keypair.fromSecretKey(Uint8Array.from(saved[name])):Keypair.generate();
const admin=signer('admin'), commander=signer('commander'), other=signer('other'), issuer=signer('issuer');
const asset=Keypair.generate().publicKey, holder=Keypair.generate().publicKey;
const disc=name=>crypto.createHash('sha256').update(`global:${name}`).digest().subarray(0,8);
const pda=(seed,key)=>PublicKey.findProgramAddressSync([Buffer.from(seed),...(key?[key.toBuffer()]:[])],PROGRAM)[0];
const key=(pubkey,isWritable=false,isSigner=false)=>({pubkey,isWritable,isSigner});
const i64=value=>{const bytes=Buffer.alloc(8);bytes.writeBigInt64LE(BigInt(value));return bytes;};
const u64=value=>{const bytes=Buffer.alloc(8);bytes.writeBigUInt64LE(BigInt(value));return bytes;};
const instruction=(name,keys,args=Buffer.alloc(0))=>new TransactionInstruction({programId:PROGRAM,keys,data:Buffer.concat([disc(name),args])});
if(preparing) {
  fs.mkdirSync(directory,{recursive:true});
  fs.writeFileSync(path.join(directory,'identities.json'),JSON.stringify({admin:[...admin.secretKey],commander:[...commander.secretKey],other:[...other.secretKey],issuer:[...issuer.secretKey]}),{mode:0o600});
  const [config,bump]=PublicKey.findProgramAddressSync([Buffer.from('game_config')],PROGRAM);
  const data=Buffer.concat([crypto.createHash('sha256').update('account:GameConfig').digest().subarray(0,8),admin.publicKey.toBuffer(),Keypair.generate().publicKey.toBuffer(),Buffer.from([bump])]);
  fs.writeFileSync(path.join(directory,'game-config.json'),JSON.stringify({pubkey:config.toBase58(),account:{lamports:10000000,data:[data.toString('base64'),'base64'],owner:PROGRAM.toBase58(),executable:false,rentEpoch:0}}));
  console.log(config.toBase58());
  process.exit(0);
}
const send=(signer,instructions)=>sendAndConfirmTransaction(rpc,new Transaction().add(...instructions),[signer],{commitment:'confirmed'});
async function rejected(operation,label) { await assert.rejects(operation); console.log(`PASS: ${label}`); }
async function profile(signer) {
  const ciphertext=Buffer.from([1,0,0,0,1]);
  const args=Buffer.concat([signer.publicKey.toBuffer(),i64(0),Buffer.from([1]),ciphertext,Buffer.alloc(12+16+16)]);
  await send(signer,[instruction('initialize_player',[key(signer.publicKey,true,true),key(pda('player_profile',signer.publicKey),true),key(pda('authorized_vault',signer.publicKey),true),key(pda('vault_backup',signer.publicKey),true),key(SystemProgram.programId)],args)]);
}
async function state(signer=commander) {
  const account=await rpc.getAccountInfo(pda('ufo_loadout',signer.publicKey));
  if (!account) return null;
  return {nonce:account.data.readBigUInt64LE(40),status:account.data[178],stage:account.data[179],booster:account.data[181]===1,next: Number(account.data.readBigInt64LE(182)),completed:account.data.readBigUInt64LE(198),equipped:account.data[206],loot:account.data.readUInt32LE(207)};
}
async function attested(signer,action,choice=0,booster=false,override={}) {
  const current=await state(signer);
  const accessory=Buffer.alloc(32); Buffer.from('Frown').copy(accessory);
  const hair=Buffer.alloc(32); Buffer.from('Tiara').copy(hair);
  const nonce=override.nonce??current?.nonce??0n;
  const proof=Buffer.concat([holder.toBuffer(),asset.toBuffer(),u64(nonce),Buffer.from([1,5]),accessory,hair,i64(Math.floor(Date.now()/1000)+90)]);
  const message=Buffer.concat([Buffer.from('gamesol:ufo:devnet:v1'),PROGRAM.toBuffer(),COLLECTION.toBuffer(),signer.publicKey.toBuffer(),proof]);
  const signing=override.issuer||issuer;
  const privateKey=crypto.createPrivateKey({key:Buffer.concat([Buffer.from('302e020100300506032b657004220420','hex'),Buffer.from(signing.secretKey.subarray(0,32))]),format:'der',type:'pkcs8'});
  const signature=crypto.sign(null,message,privateKey);
  const precompile=Ed25519Program.createInstructionWithPublicKey({publicKey:signing.publicKey.toBytes(),message,signature});
  const keys=[key(signer.publicKey,action==='start',true)];
  if(action==='start')keys.push(key(pda('player_profile',signer.publicKey)));
  keys.push(key(pda('ufo_config')),key(pda('ufo_loadout',signer.publicKey),true));
  if(action==='start')keys.push(key(pda('ufo_asset_use',asset),true),key(pda('ufo_owner_use',holder),true));
  keys.push(key(SYSVAR_INSTRUCTIONS_PUBKEY));
  if(action==='start')keys.push(key(SystemProgram.programId));
  const name={start:'start_ufo_expedition',advance:'advance_ufo_expedition',claim:'claim_ufo_loot'}[action];
  return [precompile,instruction(name,keys,Buffer.concat([proof,action==='advance'?Buffer.from([choice,booster?1:0]):Buffer.alloc(0)]))];
}
async function ready() {
  for(let attempt=0;attempt<40;attempt++) {
    const current=await state();
    const slot=await rpc.getSlot();const now=await rpc.getBlockTime(slot);
    if(now!==null&&now>=current.next)return;
    await new Promise(resolve=>setTimeout(resolve,1000));
  }
  throw new Error('Encounter did not become ready in the validator.');
}
async function main() {
  for(let attempt=0;attempt<60;attempt++) { try { await rpc.getVersion();break; }catch(error){if(attempt===59)throw error;await new Promise(resolve=>setTimeout(resolve,1000));} }
  for(const signer of [admin,commander,other]) { const signature=await rpc.requestAirdrop(signer.publicKey,3_000_000_000);await rpc.confirmTransaction(signature); }
  await profile(commander);await profile(other);
  // Genesis fixture supplies this test-only admin; production bootstrap stays restricted.
  const configure=signer=>instruction('configure_ufo',[key(signer.publicKey,true,true),key(pda('game_config')),key(pda('ufo_config'),true),key(SystemProgram.programId)],Buffer.concat([issuer.publicKey.toBuffer(),Buffer.from([1])]));
  await rejected(()=>send(other,[configure(other)]),'only game admin can configure attestor');
  await send(admin,[configure(admin)]);
  await rejected(async()=>send(commander,await attested(commander,'start',0,false,{issuer:Keypair.generate()})),'untrusted issuer rejected');
  await send(commander,await attested(commander,'start'));
  let current=await state();assert.equal(current.status,1);assert.equal(current.nonce,1n);assert.equal(current.booster,true);
  await rejected(async()=>send(other,await attested(other,'start')),'asset and holder cooldown cannot be bypassed by another commander');
  await rejected(async()=>send(commander,await attested(commander,'advance',0,false,{nonce:0n})),'replayed proof rejected');
  await rejected(async()=>send(commander,await attested(commander,'advance')),'early encounter rejected');
  await rejected(async()=>send(commander,await attested(commander,'claim')),'premature loot claim rejected');
  await ready();await send(commander,await attested(commander,'advance',0,true));
  current=await state();assert.equal(current.stage,1);assert.equal(current.booster,false);console.log('PASS: real SBF booster rescues an unsafe choice');
  await ready();
  await rejected(async()=>send(commander,await attested(commander,'advance',0,true)),'spent booster cannot be reused');
  await send(commander,await attested(commander,'advance',0,false));
  await ready();await send(commander,await attested(commander,'advance',1,false));
  current=await state();assert.equal(current.status,2);assert.equal(current.stage,3);
  const claim=await attested(commander,'claim');await send(commander,claim);
  current=await state();assert.equal(current.status,0);assert.equal(current.loot,1);assert.equal(current.completed,1n);
  await rejected(()=>send(commander,claim),'same proof cannot claim twice');
  await rejected(async()=>send(commander,await attested(commander,'claim')),'fresh proof cannot reclaim completed mission');
  const equip=index=>instruction('equip_ufo_loot',[key(commander.publicKey,false,true),key(pda('ufo_loadout',commander.publicKey),true)],Buffer.from([index]));
  await rejected(()=>send(commander,[equip(99)]),'unearned cosmetic cannot be equipped');
  await send(commander,[equip(0)]);assert.equal((await state()).equipped,0);
  await rejected(async()=>send(commander,await attested(commander,'start')),'commander daily cooldown remains after claim');
  console.log('PASS: on-chain expedition -> booster -> completion -> unique loot -> equip');
}
main().catch(error=>{console.error(error.message);if(error.logs)console.error(error.logs.join('\n'));process.exitCode=1;});
