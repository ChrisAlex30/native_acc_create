import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { serialize, deserialize } from 'borsh';
import { CounterState, Schema } from './counter';

// === CONFIG ===
const PROGRAM_ID = new PublicKey('8HtMDonYeM4q9Jz2BEjS872chETQd9mrNYeZwknrjRCz'); 
const CLUSTER_URL = 'http://127.0.0.1:8899';

const payer = Keypair.generate()
const connection = new Connection(CLUSTER_URL);

// === ACCOUNTS ===
const counterAccount = Keypair.generate(); 

async function main() {
  console.log('Requesting airdrop...');
  await connection.requestAirdrop(payer.publicKey, 2e9); // 2 SOL
  await new Promise((res) => setTimeout(res, 3000));

  const counterPubkey = counterAccount.publicKey;

  const instruction = new TransactionInstruction({
    keys: [
      { pubkey: counterPubkey, isSigner: true, isWritable: true },
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    programId: PROGRAM_ID,
    data: Buffer.from([0]), 
  });

  const tx = new Transaction().add(instruction);

  console.log('Sending transaction...');
  await sendAndConfirmTransaction(
    connection,
    tx,
    [payer, counterAccount],
    { commitment: 'confirmed' }
  );

  console.log('✅ Account created at:', counterPubkey.toBase58());

  //await new Promise((res) => setTimeout(res, 10000)); // 3 seconds delay
  const accInfo = await connection.getAccountInfo(counterAccount.publicKey, {
    commitment: "confirmed",
  });
  console.log("accInfo : ",accInfo)

  if (accInfo) {
    
    const counter = deserialize(
      Schema,
      CounterState,
      accInfo.data
    );
    console.log('📦 Stored counter value:', counter.count);
  } else {
    console.error('❌ Could not fetch account data');
  }
}

main().catch(console.error);
