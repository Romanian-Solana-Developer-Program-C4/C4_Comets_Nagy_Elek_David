import { getKeypairFromEnvironment } from '@solana-developers/helpers';
import { createMemoInstruction } from '@solana/spl-memo';
import {
  clusterApiUrl,
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from '@solana/web3.js';
import 'dotenv/config';

const url = clusterApiUrl('devnet');

const sender = getKeypairFromEnvironment('SECRET_KEY');

const connection = new Connection(url);

console.log(`Loaded sender keypair: ${sender.publicKey.toBase58()}`);

const recipient = new PublicKey('6aD15RVjeKv24miSd4GrJo4W9YQAkrsKGC9MmkopX8jJ');

console.log(
  `Atempting to send 0.01 SOL from ${sender.publicKey.toBase58()} to ${recipient.toBase58()}`
);

const transaction = new Transaction();

const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: recipient,
  lamports: 0.01 * LAMPORTS_PER_SOL,
});

transaction.add(sendSolInstruction);
const addMemoInstruction = createMemoInstruction('Hello from Solana World');

transaction.add(addMemoInstruction);

const signature = await sendAndConfirmTransaction(connection, transaction, [sender]);

console.log(`Transactrion confirmed, signature: ${signature}`);
