import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Beeraf } from "../target/types/beeraf";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { randomBytes } from "crypto";

describe("beeraf", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const connection = provider.connection;

  const program = anchor.workspace.Beeraf as Program<Beeraf>;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  const [house, maker, userA, userB, userC, mintRaffle] = Array.from({ length: 6 }, () =>
    Keypair.generate()
  );

  // Generating a big number random to create a unique seed
  const seed = new BN(randomBytes(8));
  
  let treasuryPDA = PublicKey.findProgramAddressSync([Buffer.from("treasury"), house.publicKey.toBuffer()], program.programId)[0];
  let configPDA = PublicKey.findProgramAddressSync([Buffer.from("config"), treasuryPDA.toBuffer()], program.programId)[0];
  let raffleConfigPDA = PublicKey.findProgramAddressSync([
    Buffer.from("raffle_config"), 
    house.publicKey.toBuffer(),
    seed.toArrayLike(Buffer, "le", 8),
  ], program.programId)[0];

  console.log('treasuryPDA', treasuryPDA);
  console.log('configPDA', configPDA);
  console.log('raffleConfigPDA', raffleConfigPDA);

  
  const ticketPrice = new BN(1 * LAMPORTS_PER_SOL);
  const raffleFee = new BN(100);

  it("Airdrop", async () => {
    await Promise.all([house, maker, userA, userB, userC, mintRaffle].map(async (k) => {
      return await connection.requestAirdrop(
        k.publicKey, 
        1000 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirm);
    }));
  });


  // We will generate a few different keypair to test the program 
  // The roles are:
  // - house: the person who controls the "house"
  // - maker: Person who created a raffle
  // - userA: PErson who buy ticket
  // - userB: Person who buy ticket
  // - userC: Person who buy ticket
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new BN(150))
      .accounts({
        house: house.publicKey,
      })
      .signers([house])
      .rpc()
      .then(confirm)
      .then(log);

    
    const treasuryBalance = await connection.getBalance(treasuryPDA);
    console.log(treasuryBalance);
    
    const configData = await program.account.config.fetch(configPDA);
    console.log(configData);
  });

  it('is created the raffle', async () => {    
    const tx = await program.methods.createRaffle(
      seed,
      ticketPrice,
      raffleFee,
    ).accounts({
      maker: maker.publicKey,
      house: house.publicKey,
      treasury: treasuryPDA,
      config: configPDA,
    })
    .signers([maker])
    .rpc()
    .then(confirm)
    .then(log);

    const raffleConfigData = await program.account.raffleConfig.fetch(raffleConfigPDA);
    console.log(raffleConfigData);
  });


});
