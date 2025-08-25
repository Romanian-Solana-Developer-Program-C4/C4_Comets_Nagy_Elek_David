import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import { clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import 'dotenv/config';

const url = clusterApiUrl('devnet');
const connection = new Connection(url);

const MINOR_UNITS_PER_MAJOR = 10 ** 6;

const user = getKeypairFromEnvironment('SECRET_KEY');

const tokenMintAccount = new PublicKey('AMPbidFwtECYNhkWB45JnaKpWXyfoch4n8MPDa1f2dLA');

const recipientATA = await getOrCreateAssociatedTokenAccount(
  connection,
  user,
  tokenMintAccount,
  user.publicKey
);

const transactionSignature = await mintTo(
  connection,
  user,
  tokenMintAccount,
  recipientATA.address,
  user.publicKey,
  10 * MINOR_UNITS_PER_MAJOR
);

const link = getExplorerLink('tx', transactionSignature, 'devnet');

console.log('Explorer link:', link);
