import { airdropIfRequired, getKeypairFromEnvironment } from "@solana-developers/helpers";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import "dotenv/config";

const url = clusterApiUrl("devnet");

const connection = new Connection(url);

const keypair = getKeypairFromEnvironment("SECRET_KEY");

await airdropIfRequired(connection, keypair.publicKey, 1 * LAMPORTS_PER_SOL, 0.5 * LAMPORTS_PER_SOL);

const balanceinLamports = await connection.getBalance(keypair.publicKey);
const balanceinSol = balanceinLamports / LAMPORTS_PER_SOL;

console.log(`url is ${url}`);
console.log(`balance in lamports is ${balanceinLamports}`);
console.log(`balance in sol is ${balanceinSol}`);
