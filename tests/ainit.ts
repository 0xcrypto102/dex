import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Dex } from "../target/types/dex";
import { SystemProgram, Keypair } from "@solana/web3.js";

describe("Init", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.Dex as Program<Dex>;
  // GqywbRHXZjDdRTxqTDYp9J4RSvJBwUxpGZojjYJgzMCG
  let owner = Keypair.fromSecretKey(
    Uint8Array.from([93,241,149,127,150,75,40,131,222,198,214,225,84,78,102,157,181,245,231,106,49,111,65,167,50,214,55,136,120,176,205,183,235,107,145,1,68,46,115,54,118,167,44,241,173,67,177,80,0,131,118,118,218,31,93,138,157,168,128,60,50,7,130,21])
  );


  it("Initialize", async () => {
    const [globalState, bump] = await anchor.web3.PublicKey.findProgramAddress(
        [
            Buffer.from("global-state")
        ],
        program.programId
    );

    await program.rpc.initialize(
        {
            accounts: {
                owner: owner.publicKey,
                globalState,
                systemProgram: SystemProgram.programId
            },
            signers: [owner]
        }
    )
  });
});