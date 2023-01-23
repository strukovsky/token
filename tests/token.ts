import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Token } from "../target/types/token";

describe("token", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Token as Program<Token>;
  const totalSupply = new anchor.BN(10000000);
  const token = anchor.web3.Keypair.generate();
  const adminBalance = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize(totalSupply)
        .accounts({
          token: token.publicKey,
          adminBalance: adminBalance.publicKey,
        })
        .signers([token, adminBalance])
        .rpc();
    console.log("Your transaction signature", tx);
  });
});
