import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OwnBlog } from "../target/types/own_blog";

describe("ownBlog", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OwnBlog as Program<OwnBlog>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
