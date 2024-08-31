import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID,ASSOCIATED_TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token';
import { PublicKey,sendAndConfirmTransaction,SystemProgram ,Connection,ComputeBudgetProgram,Keypair} from "@solana/web3.js";
import { Treasure } from "./require"
import { SimpleMint } from "../target/types/simple_mint";
import { readFileSync ,writeFileSync } from 'fs';
import { BN } from "bn.js";
const info = {
  programId: new PublicKey("11111111111111111111111111111111"),
  TOKEN_METADATA_PROGRAM_ID: new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
}

describe("seternia", () => {
  const con = new Connection("https://api.devnet.solana.com");
  //const con = new Connection("https://devnet.sonic.game/");
  const provider = anchor.AnchorProvider.env();
  const wallet = //anchor.Wallet.local().payer;
  readPrivateKeys("private_keys.txt")[0];
  info.programId = new PublicKey(readContracts("contract.txt")[0]);
  anchor.setProvider(provider);
  const name = "SOLClaimr";
  const uri = "https://aquamarine-elaborate-cuckoo-625.mypinata.cloud/ipfs/QmQfKsFZ9XigM6X3wTGVhKfNE7PpvWWaPfCN7VkdjPXLaN";
  const symbol = "SOLC";
  const program = new Program(require("../target/idl/simple_mint.json"), info.programId,provider);
  //const program = anchor.workspace.simple_mint as Program<SimpleMint>
  const exe_sys_prog = anchor.web3.SystemProgram;
  //const con = new Connection("http://127.0.0.1:8899");
  it("Is deploy!", async () => {
    const v = (await con.getLatestBlockhash());
    console.log(v.blockhash,v.lastValidBlockHeight)
    const contractAddressFilePath = "contract.txt";
    writeFileSync(contractAddressFilePath, program.programId.toString());
    const contractAddressFile = readContracts("contract.txt")[0];
    if (program.programId.toString() == contractAddressFile){
      console.log("File Contract address: ",new PublicKey(contractAddressFile).toString());
    }
});
  it("Is initialized!", async () => {
    let transaction = new anchor.web3.Transaction();
    const [TreasuryKey] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("TRESURE_SEED")],
      program.programId
    );
    if(await con.getAccountInfo(TreasuryKey)==null){
      const tx = await program.methods.initialize(
        new BN(1000),
        name,
        uri,
        symbol,
      ).accounts({
        admin:wallet.publicKey,
        treasure:TreasuryKey,
        systemProgram:exe_sys_prog.programId
      }).instruction();
      transaction.add(tx);
      const txSignature = await sendAndConfirmTransaction(con,transaction,[wallet], { skipPreflight: true });
      
      console.log("Your transaction signature: ", txSignature);
      const treasury_data = await program.account.treasure.fetch(TreasuryKey) as Treasure;
      console.log("treasury data:", treasury_data);
    }
  });
  it("should mint collection successfully!", async () => {
    let transaction = new anchor.web3.Transaction();
    let cump_limit = ComputeBudgetProgram.setComputeUnitLimit({ units: 800_000 });
    const [TreasuryKey,bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("TRESURE_SEED")],
      program.programId
    );
    const [CollectionKey] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("collection")],
      program.programId
    );
    const CollectionTokenAccount = await getAssociatedTokenAddress(
      CollectionKey,
      wallet.publicKey
    );

    const [metadataAddress] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("metadata"),info.TOKEN_METADATA_PROGRAM_ID.toBuffer(),CollectionKey.toBuffer()],
      info.TOKEN_METADATA_PROGRAM_ID
    );
    const [masterEdition] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        info.TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        CollectionKey.toBuffer(),
        Buffer.from("edition")
      ],
      info.TOKEN_METADATA_PROGRAM_ID
    );
    const [delegate] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        info.TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        CollectionKey.toBuffer(),
        Buffer.from("collection_authority"),
        TreasuryKey.toBuffer()
      ],
      info.TOKEN_METADATA_PROGRAM_ID
    );

    if(await con.getAccountInfo(TreasuryKey)!=null && await con.getAccountInfo(CollectionKey)==null){
      const tx = await program.methods.mintCollection(bump).accounts({
        admin:wallet.publicKey,
        treasure:TreasuryKey,
        mint:CollectionKey,
        tokenAccount:CollectionTokenAccount,
        masterEditionAccount:masterEdition,
        nftMetadata:metadataAddress,
        delegate:delegate,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: exe_sys_prog.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataProgram: info.TOKEN_METADATA_PROGRAM_ID,
      }).instruction();
      transaction.add(tx).add(cump_limit);
      const txSignature = await sendAndConfirmTransaction(con,transaction,[wallet], { skipPreflight: true });
      
      console.log("Your transaction signature: ", txSignature);
    }
  });

  it("should mint and active NFT successfully!", async () => {
    let transaction = new anchor.web3.Transaction();
    let cump_limit = ComputeBudgetProgram.setComputeUnitLimit({ units: 1800_000 });
    const [TreasuryKey,bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("TRESURE_SEED")],
      program.programId
    );

    const [CollectionKey] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("collection")],
      program.programId
    );
    const [CmetadataAddress] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("metadata"),info.TOKEN_METADATA_PROGRAM_ID.toBuffer(),CollectionKey.toBuffer()],
      info.TOKEN_METADATA_PROGRAM_ID
    );
    const [CmasterEdition] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        info.TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        CollectionKey.toBuffer(),
        Buffer.from("edition")
      ],
      info.TOKEN_METADATA_PROGRAM_ID
    );
    const [MintKey] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("mint"),wallet.publicKey.toBuffer()],
      program.programId
    );
    const [metadataAddress] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("metadata"),info.TOKEN_METADATA_PROGRAM_ID.toBuffer(),MintKey.toBuffer()],
      info.TOKEN_METADATA_PROGRAM_ID
    );
    const [masterEdition] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        info.TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        MintKey.toBuffer(),
        Buffer.from("edition")
      ],
      info.TOKEN_METADATA_PROGRAM_ID
    );
    const [delegate] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        info.TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        CollectionKey.toBuffer(),
        Buffer.from("collection_authority"),
        TreasuryKey.toBuffer()
      ],
      info.TOKEN_METADATA_PROGRAM_ID
    );
    const MintTokenAccount = await getAssociatedTokenAddress(
      MintKey,
      wallet.publicKey
    );
    if(await con.getAccountInfo(TreasuryKey)!=null){
      const tx = await program.methods.mint(
        bump
      ).accounts({
        payer:wallet.publicKey,
        treasure:TreasuryKey,
        mint:MintKey,
        collectionMint:CollectionKey,
        tokenAccount:MintTokenAccount,
        masterEditionAccount:masterEdition,
        collectionMasterEdition:CmasterEdition,
        nftMetadata:metadataAddress,
        collectionMetadata:CmetadataAddress,
        delegate:delegate,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        systemProgram: exe_sys_prog.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataProgram: info.TOKEN_METADATA_PROGRAM_ID,
      }).instruction();
      transaction.add(tx).add(cump_limit);
      const txSignature = await sendAndConfirmTransaction(con,transaction,[wallet], { skipPreflight: true });
      
      console.log("Your transaction signature: ", txSignature);
    }
  });
});
export function readContracts(filePath: string): string[] {
  const privateKeysString = readFileSync(filePath, 'utf-8');
  return privateKeysString.split('\n').map(line => line.trim()).filter(line => line.length > 0);
}
export function readPrivateKeys(filePath: string): Keypair[] {
  const privateKeysString = readFileSync(filePath, 'utf-8');
  const privateKeys = privateKeysString.split('\n').map(line => line.trim()).filter(line => line.length > 0);
  return privateKeys.map(privateKey => {
    const privateKeyBytes = Buffer.from(privateKey, 'hex');
    return Keypair.fromSecretKey(privateKeyBytes);
  });
}
