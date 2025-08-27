import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from '@metaplex-foundation/umi';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';
import { getKeypairFromEnvironment } from '@solana-developers/helpers';
import { clusterApiUrl } from '@solana/web3.js';
import { readFile } from 'fs/promises';

import 'dotenv/config';

// load keypair from environment
const user = getKeypairFromEnvironment('SECRET_KEY');

// create UMI instance connected to Solana devenet
const umi = createUmi(clusterApiUrl('devnet'));

// convert keypair and create signer
const keypair = umi.eddsa.createKeypairFromSecretKey(user.secretKey);
const signer = createSignerFromKeypair(umi, keypair);

// configure UMI with Irys uploader and signer identity
umi.use(irysUploader());
umi.use(signerIdentity(signer));

const IMAGE_FILE = './image.jpg';

async function uploadImage() {
  try {
    console.log('Uploading image...');

    // read image file
    const img = await readFile(IMAGE_FILE);

    // convert to UMI-compatilbe format
    const imgConverted = createGenericFile(new Uint8Array(img), 'image.jpg');

    // upload image to Irys and gt the URI
    const [myURI] = await umi.uploader.upload([imgConverted]);

    console.log('Image uploaded successfully:', myURI);
  } catch (err) {
    console.error('[UploadImage] Failed with:', err);
  }
}

uploadImage();
