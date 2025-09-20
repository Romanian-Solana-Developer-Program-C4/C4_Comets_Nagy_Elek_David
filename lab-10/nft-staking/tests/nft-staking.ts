import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { LiteSVM } from "litesvm";

const programId = new PublicKey("AESehE9tH7VxNKfcBspX7ZkE3F5zupjHNFMhn9edrw5t");

describe("nft-staking", () => {
  // Configure the client to use the local cluster.

  // anchor.setProvider(anchor.AnchorProvider.env());

  const svm = new LiteSVM();
  // svm.addProgramFromFile(programId, "./target/deploy/nft_staking.so");
  // svm.addProgramFromFile(
  //   new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
  //   "metaplex_token_metadata_program.so"
  // );

  let user = Keypair.generate();

  svm.setAccount(user.publicKey, {
    lamports: 100 * LAMPORTS_PER_SOL,
    data: Buffer.alloc(0),
    owner: SystemProgram.programId,
    executable: false,
  });

  let userData = svm.getAccount(user.publicKey);

  let userTokenAccount = Keypair.generate();
  // Correctly get the rent-exempt lamports using svm.getRent()
  const rentExemption = Number(svm.minimumBalanceForRentExemption(BigInt(165)));

  const tokenaccount = Buffer.alloc(165);

  // serialize token account data
  let userTokenAccountData = svm.setAccount(userTokenAccount.publicKey, {
    lamports: rentExemption,
    data: tokenaccount,
    owner: TOKEN_PROGRAM_ID,
    executable: false,
  });

  // set chain clock
  let chain_clock = svm.getClock();
  chain_clock.unixTimestamp += BigInt(1000);
  svm.setClock(chain_clock);

  // const program = anchor.workspace.nftStaking as Program<NftStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    console.log(
      `User: ${user.publicKey.toString()} has ${userData.lamports} lamports`
    );
  });
});
