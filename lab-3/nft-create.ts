import { createNft, mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata';
import {
  createSignerFromKeypair,
  generateSigner,
  percentAmount,
  signerIdentity,
} from '@metaplex-foundation/umi';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { base58 } from '@metaplex-foundation/umi/serializers';
import { getExplorerLink, getKeypairFromEnvironment } from '@solana-developers/helpers';
import { clusterApiUrl } from '@solana/web3.js';
import 'dotenv/config';

const user = getKeypairFromEnvironment('SECRET_KEY');

const umi = createUmi(clusterApiUrl('devnet'));

const keypair = umi.eddsa.createKeypairFromSecretKey(user.secretKey);
const signer = createSignerFromKeypair(umi, keypair);

// add token metadata program
umi.use(mplTokenMetadata());
umi.use(signerIdentity(signer));

const IMG_URI = 'https://gateway.irys.xyz/3SBicEhPgMEfBMraZ4ChKdhC5HgWS27zX7aQtKbZBo1h';
const METADATA_URI = 'https://gateway.irys.xyz/2mdYrCY4S59SoccGC7THWpDjYtecxXnBTC16maf2jbTc';

async function createMyNFT() {
  try {
    console.log('Creating NFT...');

    // generate a new keypair for the mint account
    const mint = generateSigner(umi);

    // create NFT trasnsation
    let tx = createNft(umi, {
      name: 'Commets C4 NFT',
      mint, // the mint account signer
      authority: signer, // who has authority over this NFT
      sellerFeeBasisPoints: percentAmount(10), // 10%
      isCollection: false,
      uri: METADATA_URI,
    });

    // send transaction and wait for confirmation
    let result = await tx.sendAndConfirm(umi);
    const [signature] = base58.deserialize(result.signature);

    console.log('NFT created successfully:', signature);
    console.log('Mint address:', mint.publicKey);
    console.log('Explorer link:', getExplorerLink('tx', signature, 'devnet'));
  } catch (err) {
    console.error('[CreateMyNFT] Failed with:', err);
  }
}

createMyNFT();
