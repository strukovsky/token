import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {Token} from "../target/types/token";
import {expect} from "chai";

describe("token", () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Token as Program<Token>;
    const totalSupply = new anchor.BN(10000000);
    const token = anchor.web3.Keypair.generate();
    const adminOwnership = anchor.web3.Keypair.generate();
    const recipientOwnership = anchor.web3.Keypair.generate();
    const recipientKeypair = anchor.web3.Keypair.generate();

    it("should be initialized", async () => {
        const tx = await program.methods.initialize(totalSupply)
            .accounts({
                token: token.publicKey,
                adminOwnership: adminOwnership.publicKey,
            })
            .signers([token, adminOwnership])
            .rpc();
        console.log("Your transaction signature", tx);
    });


    it("should perform a transfer with non-initialized account", async () => {
        const transferAmount = new anchor.BN(10);

        await program.methods.transferToEmpty(recipientKeypair.publicKey, transferAmount)
            .accounts({
                from: adminOwnership.publicKey,
                to: recipientOwnership.publicKey,
            })
            .signers([recipientOwnership])
            .rpc();

        let balance = (await program.account.tokenOwnership.fetch(recipientOwnership.publicKey)).balance;
        expect(transferAmount.eq(balance)).to.be.true;
    });

    it("should perform a transfer with initialized account", async () => {
        const transferAmount = new anchor.BN(100);
        let recipientBalanceBefore = (await program.account.tokenOwnership.fetch(recipientOwnership.publicKey)).balance;
        await program.methods.transferToExisting(recipientKeypair.publicKey, transferAmount)
            .accounts({
                from: adminOwnership.publicKey,
                to: recipientOwnership.publicKey,
            })
            .rpc();
        let recipientBalanceAfter = (await program.account.tokenOwnership.fetch(recipientOwnership.publicKey)).balance;
        expect(recipientBalanceAfter.sub(recipientBalanceBefore).eq(transferAmount)).to.be.true;
    });
});
