import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Beeraf } from "../target/types/beeraf";
import { Ed25519Program, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SYSVAR_INSTRUCTIONS_PUBKEY, Transaction } from "@solana/web3.js";
import { randomBytes } from "crypto";

const coreProgram = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d")

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

  const [house, maker, userA, userB, userC, raffle, mintRaffle, ticketA] = Array.from({ length: 8 }, () =>
    Keypair.generate()
  );
  
  let treasuryPDA = PublicKey.findProgramAddressSync([Buffer.from("treasury"), house.publicKey.toBuffer()], program.programId)[0];
  let configPDA = PublicKey.findProgramAddressSync([Buffer.from("config"), treasuryPDA.toBuffer()], program.programId)[0];
  let raffleConfigPDA = PublicKey.findProgramAddressSync([
    Buffer.from("raffle"),
    house.publicKey.toBuffer(),
    raffle.publicKey.toBuffer()
  ], program.programId)[0];
  let vaultPDA = PublicKey.findProgramAddressSync([Buffer.from("vault"), maker.publicKey.toBuffer()], program.programId)[0];

  console.log('treasuryPDA', treasuryPDA);
  console.log('configPDA', configPDA);
  console.log('raffleConfigPDA', raffleConfigPDA);
  
  // We will have to pay 1 SOL to create a Raffle for that house
  const fee = new BN(1 * LAMPORTS_PER_SOL);

  // For each ticket, the raffle maker will get a 10%
  const raffleFee = new BN(100);

  // Each ticket will cost 1 SOL
  const ticketPrice = new BN(1 * LAMPORTS_PER_SOL);

  const slotInterval = new BN(2);

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
    const tx = await program.methods.initialize(fee)
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
    const createRaffleArgs  = {
      name: "Raffle Test Collection",
      uri: "https://example.com",
      ticketPrice,
      raffleFee,
      slotInterval
    };

    const tx = await program.methods.createRaffle(createRaffleArgs)
    .accountsPartial({
      maker: maker.publicKey,
      house: house.publicKey,
      treasury: treasuryPDA,
      config: configPDA,
      raffle: raffle.publicKey,
      raffleConfig: raffleConfigPDA,
      vault: vaultPDA,
      mplCoreProgram: coreProgram,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([maker, raffle])
    .rpc()
    .then(confirm)
    .then(log);

    const raffleConfigData = await program.account.raffleConfig.fetch(raffleConfigPDA);
    console.log(raffleConfigData);

    const treasuryBalance = await connection.getBalance(treasuryPDA);
    console.log(treasuryBalance);
  });

  it('should be able to buy a ticket', async () => {
    const buyTicketArgs  = {
      name: "Raffle Test Ticket",
      uri: "https://example.com",
    };

    let makerBalance = await connection.getBalance(maker.publicKey);
    console.log('makerBalance: ', makerBalance);

    const tx = await program.methods.buyTicket(buyTicketArgs)
    .accountsPartial({
      buyer: userA.publicKey,
      house: house.publicKey,
      maker: maker.publicKey,
      treasury: treasuryPDA,
      config: configPDA,
      raffle: raffle.publicKey,
      raffleConfig: raffleConfigPDA,
      vault: vaultPDA,
      ticket: ticketA.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      mplCoreProgram: coreProgram,
    })
    .signers([userA, ticketA])
    .rpc()
    .then(confirm)
    .then(log);

    makerBalance = await connection.getBalance(maker.publicKey);
    console.log('makerBalance: ', makerBalance);

    const vaultBalance = await connection.getBalance(vaultPDA);
    console.log('vaultBalance: ', vaultBalance);
  });

  it('should be able to buy many tickets', async () => {
    const buyTicketArgs  = {
      name: "Raffle Test Ticket",
      uri: "https://example.com",
    };

    for(let i = 0; i < 5; i++) {
      const ticket = Keypair.generate(); 

      const tx = await program.methods.buyTicket(buyTicketArgs)
      .accountsPartial({
        buyer: userA.publicKey,
        house: house.publicKey,
        maker: maker.publicKey,
        treasury: treasuryPDA,
        config: configPDA,
        raffle: raffle.publicKey,
        raffleConfig: raffleConfigPDA,
        vault: vaultPDA,
        ticket: ticket.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCoreProgram: coreProgram,
      })
      .signers([userA, ticket])
      .rpc()
      .then(confirm)
      .then(log);
    }

    const makerBalance = await connection.getBalance(maker.publicKey);
    console.log('makerBalance: ', makerBalance);

    const vaultBalance = await connection.getBalance(vaultPDA);
    console.log('vaultBalance: ', vaultBalance);

  });

  it('should be able to resolve the raffle and save the winner number', async () => {
    let raffleConfigAccount = await connection.getAccountInfo(raffleConfigPDA, "confirmed");
    
    const message = Buffer.concat([
      raffleConfigAccount.data.slice(8, 40), // authority (32 bytes)
      raffleConfigAccount.data.slice(40, 72), // collection (32 bytes)
      raffleConfigAccount.data.slice(72, 80), // slot (8 bytes)
      raffleConfigAccount.data.slice(80, 88), // ticket_price (8 bytes)
      raffleConfigAccount.data.slice(88, 96), // raffle_fee (8 bytes)
      raffleConfigAccount.data.slice(96, 97), // raffle_config_bump (1 byte)
      raffleConfigAccount.data.slice(97, 98), // vault_bump (1 byte) 
    ]);
  
    let sig_ix = Ed25519Program.createInstructionWithPrivateKey({
      privateKey: maker.secretKey,
      message // : raffleConfigAccount.data.subarray(8) // It will slice the data to get all data after the `discriminator`!? 
    });

    const solve_ix = await program.methods.solveRaffle(Buffer.from(sig_ix.data.buffer.slice(16+32, 16+32+64)))    
    .accountsPartial({
      maker: maker.publicKey,
      house: house.publicKey,
      treasury: treasuryPDA,
      config: configPDA,
      raffle: raffle.publicKey,
      raffleConfig: raffleConfigPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
      instructionSysvar: SYSVAR_INSTRUCTIONS_PUBKEY,
      mplCoreProgram: coreProgram,
    })
    .signers([maker])
    .instruction();

    const tx = new Transaction().add(sig_ix).add(solve_ix);

    let evenListener: number; 

    try {

      let betResult: number; 

      evenListener = program.addEventListener('rafEvent', (event) => {
        betResult = event.winner;
      });

      await sendAndConfirmTransaction(
        program.provider.connection,
        tx,
        [maker]
      ).then(log);

      console.log(betResult);
    }catch(err) {
      console.log(err);
      throw Error("It should not fail the program!");
    } finally {
      program.removeEventListener(evenListener);
    }
  });

  it('should be able to scratch the ticket and see if Im the winner', async () => {
    try {
      let userABalance = await connection.getBalance(userA.publicKey);
      console.log('userABalance: ', userABalance);

      const scratch_tx = await program.methods.scratchTicket()
        .accountsPartial({
          buyer: userA.publicKey,
          house: house.publicKey,
          maker: maker.publicKey,
          treasury: treasuryPDA,
          config: configPDA,
          raffle: raffle.publicKey,
          raffleConfig: raffleConfigPDA,
          ticket: ticketA.publicKey,
          vault: vaultPDA,
          systemProgram: anchor.web3.SystemProgram.programId,
          mplCoreProgram: coreProgram,
        })
        .signers([userA, ticketA])
        .rpc()
        .then(confirm)
        .then(log);

        userABalance = await connection.getBalance(userA.publicKey);
      console.log('userABalance: ', userABalance);
    } catch(err) {
      console.log(err);
      throw new Error(err);
    }
  });
});
