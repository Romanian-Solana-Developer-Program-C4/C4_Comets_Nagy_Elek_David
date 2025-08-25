import { getKeypairFromEnvironment } from '@solana-developers/helpers';
import 'dotenv/config';

const keypair = getKeypairFromEnvironment('SECRET_KEY');

console.log(`public key: ${keypair.publicKey.toBase58()}`);

console.log(`secret key: ${keypair.secretKey}`);
