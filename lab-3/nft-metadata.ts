import { createSignerFromKeypair, signerIdentity } from '@metaplex-foundation/umi';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';
import { getKeypairFromEnvironment } from '@solana-developers/helpers';
import { clusterApiUrl } from '@solana/web3.js';
import 'dotenv/config';

// load keypair from environment
const user = getKeypairFromEnvironment('SECRET_KEY');

const umi = createUmi(clusterApiUrl('devnet'));

const keypair = umi.eddsa.createKeypairFromSecretKey(user.secretKey);
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

const IMG_URI = 'https://gateway.irys.xyz/3SBicEhPgMEfBMraZ4ChKdhC5HgWS27zX7aQtKbZBo1h';

async function uploadMetadata() {
  try {
    console.log('Uploading metadata...');

    const metadata = {
      name: 'Commets C4 NFT',
      symbol: 'C4NFT',
      description: 'This is a Commets C4 NFT',
      image: IMG_URI,
      attributes: [
        { trait_type: 'Background', value: 'Dark Blue' },
        { trait_type: 'Body', value: 'Blue' },
        { trait_type: 'Eyes', value: 'White' },
        { trait_type: 'Mouth', value: 'No' },
      ],
      properties: {
        files: [
          {
            uri: IMG_URI,
            type: 'image/jpeg',
          },
        ],
      },
    };

    const metadataUri = await umi.uploader.uploadJson(metadata);

    console.log('Metadata uploaded successfully:', metadataUri);
  } catch (err) {
    console.error('[UploadMetadata] Failed with:', err);
  }
}

uploadMetadata();
