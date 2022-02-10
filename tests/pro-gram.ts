import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { ProGram } from "../target/types/pro_gram";
import { TestProg } from "../target/types/test_prog";

describe("pro-gram", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const proProgram = anchor.workspace.ProGram as Program<ProGram>;
  const testProgram = anchor.workspace.TestProg as Program<TestProg>;

  it("Is initialized!", async () => {
    const [gram, gramBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("gram"), testProgram.programId.toBuffer()],
      proProgram.programId
    );

    const [gramAuthority, gramAuthorityBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from("gram_authority")],
        testProgram.programId
      );

    const initTx = await proProgram.rpc.initGram("t-prog", "t-desc", "t-repo", {
      accounts: {
        gram: gram,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        prog: testProgram.programId,
      },
    });

    await provider.connection.confirmTransaction(initTx);

    const updateTx = await testProgram.rpc.changeAuth({
      accounts: {
        gram: gram,
        gramAuthority: gramAuthority,
        newAuthority: provider.wallet.publicKey,
        prog: testProgram.programId,
        proGram: proProgram.programId,
      },
    });

    await provider.connection.confirmTransaction(updateTx);

    const updateMetaTx = await proProgram.rpc.updateGramMeta(
      "Uniswap",
      "swap description",
      "github.com/heheswap",
      {
        accounts: {
          gram: gram,
          prog: testProgram.programId,
          gramAuthority: provider.wallet.publicKey,
        },
      }
    );

    await provider.connection.confirmTransaction(updateMetaTx);

    const [like, likeBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("like"),
        provider.wallet.publicKey.toBuffer(),
        gram.toBuffer(),
      ],
      proProgram.programId
    );

    const likeTx = await proProgram.rpc.likeGram("wowie!!!", {
      accounts: {
        gram: gram,
        liker: provider.wallet.publicKey,
        like: like,
        prog: testProgram.programId,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    await provider.connection.confirmTransaction(likeTx);

    const gramAccount = await proProgram.account.gram.fetch(gram);
    console.log(gramAccount);

    const likeAccounts = await proProgram.account.like.all();
    console.log(likeAccounts);
  });
});
