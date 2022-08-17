import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Nftamm } from "../target/types/nftamm";

describe("nftamm", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Nftamm as Program<Nftamm>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
