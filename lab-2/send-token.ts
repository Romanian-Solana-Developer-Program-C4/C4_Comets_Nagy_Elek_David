import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
import { getOrCreateAssociatedTokenAccount, transfer } from '@solana/spl-token';
import { clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import 'dotenv/config';

const url = clusterApiUrl('devnet');
const connection = new Connection(url);

const user = getKeypairFromEnvironment('SECRET_KEY');

const recipient = new PublicKey('6aD15RVjeKv24miSd4GrJo4W9YQAkrsKGC9MmkopX8jJ');

const tokenMintAccount = new PublicKey('AMPbidFwtECYNhkWB45JnaKpWXyfoch4n8MPDa1f2dLA');

const MINOR_UNITS_PER_MAJOR = 10 ** 6;

const sourceATA = await getOrCreateAssociatedTokenAccount(
  connection,
  user,
  tokenMintAccount,
  user.publicKey
);

const recipientATA = await getOrCreateAssociatedTokenAccount(
  connection,
  user,
  tokenMintAccount,
  recipient
);

const signature = await transfer(
  connection,
  user,
  sourceATA.address,
  recipientATA.address,
  user.publicKey,
  10 * MINOR_UNITS_PER_MAJOR
);

const link = getExplorerLink('tx', signature, 'devnet');

console.log('Transaction confirmed, explorer link:', link);
