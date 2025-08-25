import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
import { createMint } from '@solana/spl-token';
import { clusterApiUrl, Connection } from '@solana/web3.js';
import 'dotenv/config';

const url = clusterApiUrl('devnet');

const connection = new Connection(url);

const user = getKeypairFromEnvironment('SECRET_KEY');

const tokenMint = await createMint(connection, user, user.publicKey, null, 6);

console.log('Token mint created at:', tokenMint.toBase58());
console.log('Explorer link:', getExplorerLink('address', tokenMint.toBase58(), 'devnet'));
