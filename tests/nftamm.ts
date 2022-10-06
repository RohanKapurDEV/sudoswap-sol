import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Nftamm } from "../target/types/nftamm";
import { LOCALHOST, Amman } from "@metaplex-foundation/amman-client";
import {
  CreateNftOutput,
  keypairIdentity,
  Metaplex,
} from "@metaplex-foundation/js";
import { Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import {} from "@metaplex-foundation/mpl-token-metadata";
import { assert } from "chai";

describe("nftamm", () => {
  // Configure the anchor client to use the local cluster
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Nftamm as Program<Nftamm>;

  // Set up accounts
  const protocolAuthority = anchor.web3.Keypair.generate(); // PairAuthority owner
  const poolCreator = anchor.web3.Keypair.generate();
  const poolUser = anchor.web3.Keypair.generate();

  const pairAuthorityAccount = anchor.web3.Keypair.generate();
  const feeForPairAuthority = 100;

  let collectionNft: CreateNftOutput;
  let firstNft: CreateNftOutput;
  let secondNft: CreateNftOutput;
  let thirdNft: CreateNftOutput;
  let fourthNft: CreateNftOutput;
  let fifthNft: CreateNftOutput;

  // Configure the metaplex client to use the local cluster
  const metaplexConnection = new Connection(
    "http://localhost:8899",
    "confirmed"
  );
  const metaplex = new Metaplex(metaplexConnection).use(
    keypairIdentity(protocolAuthority)
  );

  before("Set up accounts", async () => {
    // Fund protocol authority account
    const protocolAuthorityAirdropSig =
      await provider.connection.requestAirdrop(
        protocolAuthority.publicKey,
        10 * LAMPORTS_PER_SOL
      );

    const latestBlockhash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      signature: protocolAuthorityAirdropSig,
    });

    // Fund pool creator account
    const poolCreatorAirdropSig = await provider.connection.requestAirdrop(
      protocolAuthority.publicKey,
      10 * LAMPORTS_PER_SOL
    );

    await provider.connection.confirmTransaction({
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      signature: poolCreatorAirdropSig,
    });

    // Fund pool user account
    const poolUserAirdropSig = await provider.connection.requestAirdrop(
      protocolAuthority.publicKey,
      10 * LAMPORTS_PER_SOL
    );

    await provider.connection.confirmTransaction({
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      signature: poolUserAirdropSig,
    });
  });

  it("Mint an MCC verified collection", async () => {
    // create collection of NFTs
    // NOTE: The protocol authority is the owner of the collection
    const collectionNftResponse = await metaplex
      .nfts()
      .create({
        name: "Base collection NFT",
        sellerFeeBasisPoints: 0,
        uri: "lol",
        isCollection: true,
      })
      .run();

    collectionNft = collectionNftResponse;

    const firstNftResponse = await metaplex
      .nfts()
      .create({
        name: "First NFT",
        sellerFeeBasisPoints: 0,
        uri: "lol",
        collection: collectionNft.mintAddress,
        collectionAuthority: protocolAuthority,
      })
      .run();

    const secondNftResponse = await metaplex
      .nfts()
      .create({
        name: "Second NFT",
        sellerFeeBasisPoints: 0,
        uri: "lol",
        collection: collectionNft.mintAddress,
        collectionAuthority: protocolAuthority,
      })
      .run();

    const thirdNftResponse = await metaplex
      .nfts()
      .create({
        name: "Third NFT",
        sellerFeeBasisPoints: 0,
        uri: "lol",
        collection: collectionNft.mintAddress,
        collectionAuthority: protocolAuthority,
      })
      .run();

    const fourthNftResponse = await metaplex
      .nfts()
      .create({
        name: "Fourth NFT",
        sellerFeeBasisPoints: 0,
        uri: "lol",
        collection: collectionNft.mintAddress,
        collectionAuthority: protocolAuthority,
      })
      .run();

    const fifthNftResponse = await metaplex
      .nfts()
      .create({
        name: "Fifth NFT",
        sellerFeeBasisPoints: 0,
        uri: "lol",
        collection: collectionNft.mintAddress,
        collectionAuthority: protocolAuthority,
      })
      .run();

    firstNft = firstNftResponse;
    secondNft = secondNftResponse;
    thirdNft = thirdNftResponse;
    fourthNft = fourthNftResponse;
    fifthNft = fifthNftResponse;

    let firstMint = await metaplex
      .nfts()
      .findByMint({ mintAddress: firstNft.mintAddress })
      .run();

    let secondMint = await metaplex
      .nfts()
      .findByMint({ mintAddress: secondNft.mintAddress })
      .run();

    let thirdMint = await metaplex
      .nfts()
      .findByMint({ mintAddress: thirdNft.mintAddress })
      .run();

    let fourthMint = await metaplex
      .nfts()
      .findByMint({ mintAddress: fourthNft.mintAddress })
      .run();

    let fifthMint = await metaplex
      .nfts()
      .findByMint({ mintAddress: fifthNft.mintAddress })
      .run();

    // Assert that NFTs minted actually belong to the correct collection
    assert(collectionNft.mintAddress.equals(firstMint.collection.address));
    assert(collectionNft.mintAddress.equals(secondMint.collection.address));
    assert(collectionNft.mintAddress.equals(thirdMint.collection.address));
    assert(collectionNft.mintAddress.equals(fourthMint.collection.address));
    assert(collectionNft.mintAddress.equals(fifthMint.collection.address));
  });

  it("Initialize pair authority", async () => {
    const tx = await program.methods
      .initializePairAuthority(feeForPairAuthority)
      .accounts({
        pairAuthority: pairAuthorityAccount.publicKey,
        payer: protocolAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([pairAuthorityAccount, protocolAuthority])
      .rpc();

    // const pairAuthority = await program.account.pairAuthority.fetch(
    //   pairAuthorityAccount.publicKey
    // );

    // console.log("Pair authority fees: ", pairAuthority.fees);
    // console.log(
    //   "Pair authority current authority: ",
    //   pairAuthority.currentAuthority.toString()
    // );
  });

  it("initialize a pool", async () => {});
});
