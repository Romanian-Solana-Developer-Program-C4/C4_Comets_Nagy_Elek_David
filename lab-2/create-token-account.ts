import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
import { getOrCreateAssociatedTokenAccount } from '@solana/spl-token';
import { clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import 'dotenv/config';

const url = clusterApiUrl('devnet');

const connection = new Connection(url);

const user = getKeypairFromEnvironment('SECRET_KEY');

const tokenMintAccount = new PublicKey('AMPbidFwtECYNhkWB45JnaKpWXyfoch4n8MPDa1f2dLA');
const recipient = new PublicKey('6aD15RVjeKv24miSd4GrJo4W9YQAkrsKGC9MmkopX8jJ');

const recipeintATA = await getOrCreateAssociatedTokenAccount(
  connection,
  user,
  tokenMintAccount,
  recipient
);

console.log('Associated token account created at:', recipeintATA.address.toBase58());

const link = getExplorerLink('address', recipeintATA.address.toBase58(), 'devnet');

console.log('Explorer link:', link);
