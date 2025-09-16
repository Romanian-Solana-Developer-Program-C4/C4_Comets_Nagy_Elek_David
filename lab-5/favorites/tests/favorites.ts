import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Favorites } from "../target/types/favorites";

const web3 = anchor.web3;

describe("Favorites", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const user = (provider.wallet as anchor.Wallet).payer;
  const program = anchor.workspace.Favorites as Program<Favorites>;

  before(async () => {
    const balance = await provider.connection.getBalance(user.publicKey);
    const balanceinSol = balance / web3.LAMPORTS_PER_SOL;
    const formatedBalance = new Intl.NumberFormat().format(balanceinSol);
    // console.log(`User balance: ${formatedBalance} SOL`);
  });

  it("Saves a user favorites to the blockchain!", async () => {
    // Add your test here.
    const favoriteNumber = new anchor.BN(42);
    const favoriteColor = "blue";
    const favoriteHobbies = ["coding", "reading"];
    await program.methods.setFavorites(favoriteNumber, favoriteColor, favoriteHobbies).signers([user]).rpc();

    const favoritesPdaAndBump = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favorites"), user.publicKey.toBuffer()],
      program.programId
    );

    const favoritesPda = favoritesPdaAndBump[0];
    const dataFromPda = await program.account.favorites.fetch(favoritesPda);

    assert.equal(dataFromPda.number.toString(), favoriteNumber.toString());
    assert.equal(dataFromPda.color, favoriteColor);
    assert.deepEqual(dataFromPda.hobbies, favoriteHobbies);
  });

  it("Doesn't let people write to favorites for other users", async () => {
    const otherUser = anchor.web3.Keypair.generate();
    try {
      const favoriteNumber = new anchor.BN(42);
      const favoriteColor = "blue";
      const favoriteHobbies = ["coding", "reading"];
      await program.methods.setFavorites(favoriteNumber, favoriteColor, favoriteHobbies).signers([otherUser]).rpc();
    } catch (error) {
      const errorString = (error as Error).message;
      assert.isTrue(errorString.includes("unknown signe"));
    }

  })
});
