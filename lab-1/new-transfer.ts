import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
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
const connection = new Connection(url);

const sender = getKeypairFromEnvironment('SECRET_KEY');
const receiver = new PublicKey('6aD15RVjeKv24miSd4GrJo4W9YQAkrsKGC9MmkopX8jJ');

const transaction = new Transaction();

const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: receiver,
  lamports: 0.05 * LAMPORTS_PER_SOL,
});

transaction.add(sendSolInstruction);

const signature = await sendAndConfirmTransaction(connection, transaction, [sender], {
  commitment: 'confirmed',
});

const link = getExplorerLink('tx', signature, 'devnet');

console.log(`Transaction confirmed, signature: ${signature}`);
console.log(`Transaction link: ${link}`);
