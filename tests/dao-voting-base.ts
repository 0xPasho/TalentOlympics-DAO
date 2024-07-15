import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DaoVoting } from "../target/types/talent_olympics_dao_voting";
import { assert } from "chai";
import dayjs from "dayjs";

describe("dao-voting", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .DaoVoting as Program<DaoVoting>;

  const [user1, user2, user3, user4] = [
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
    anchor.web3.Keypair.generate(),
  ];

  const id = new anchor.BN(1);
  const [proposalAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("proposal"), id.toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  const maximumVote = new anchor.BN(2);
  const endTime = dayjs().add(5, "s").unix();

  before(async () => {
    {
      const tx = await provider.connection.requestAirdrop(
        user1.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(tx);
      const tx2 = await provider.connection.requestAirdrop(
        user2.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(tx2);
      const tx3 = await provider.connection.requestAirdrop(
        user3.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(tx3);
      const tx4 = await provider.connection.requestAirdrop(
        user4.publicKey,
        5 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(tx3);
    }
  });

  it("Should init a proposal successfully", async () => {
    const tx = await program.methods
      .initProposal(
        id,
        "Give 1000 USDC to the winner",
        maximumVote,
        new anchor.BN(endTime)
      )
      .accountsPartial({
        signer: user1.publicKey,
        proposal: proposalAccount,
      })
      .signers([user1])
      .rpc();

    assert.ok(tx);

    const proposal = await program.account.proposal.fetch(proposalAccount);
    assert.equal(proposal.id.toNumber(), id.toNumber());
    assert.equal(proposal.maxVotes.toNumber(), maximumVote.toNumber());
    assert.equal(proposal.votesFor.toNumber(), 0);
    assert.equal(proposal.votesAgainst.toNumber(), 0);

    console.log("Init proposal tx", tx);
  });

  it("Should vote on a proposal successfully", async () => {
    const tx1 = await program.methods
      .vote(id, true)
      .accountsPartial({
        voter: user2.publicKey,
        proposal: proposalAccount,
      })
      .signers([user2])
      .rpc();

    assert.ok(tx1);

    console.log("user2 Vote for", tx1);

    const tx2 = await program.methods
      .vote(id, false)
      .accountsPartial({ voter: user3.publicKey, proposal: proposalAccount })
      .signers([user3])
      .rpc();

    assert.ok(tx2);

    console.log("user3 Vote agains", tx2);

    const proposal = await program.account.proposal.fetch(proposalAccount);

    assert.equal(proposal.votesFor.toNumber(), 1);
    assert.equal(proposal.votesAgainst.toNumber(), 1);
  });

  it("Should vote fail if maximum votes reached", async () => {
    try {
      await program.methods
        .vote(id, true)
        .accountsPartial({
          voter: user4.publicKey,
          proposal: proposalAccount,
        })
        .signers([user4])
        .rpc();

      assert.ok(false);
    } catch (error) {
      assert.isNotNull(error);
    }
  });

  it("Should creator close proposal fail when proposal not expired", async () => {
    try {
      const tx = await program.methods
        .closeProposal(id)
        .accountsPartial({
          signer: user1.publicKey,
          proposal: proposalAccount,
        })
        .signers([user1])
        .rpc();
      assert.ok(false);
    } catch (error) {
      assert.isNotNull(error);
    }
  });

  it("Should creator close proposal successfully when proposal expired", async () => {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    const tx = await program.methods
      .closeProposal(id)
      .accountsPartial({
        signer: user1.publicKey,
        proposal: proposalAccount,
      })
      .signers([user1])
      .rpc();
    assert.ok(tx);

    console.log("Close proposal tx", tx);
  });
});
