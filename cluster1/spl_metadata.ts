import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey, base58 } from "@metaplex-foundation/umi";
import { PublicKey } from "@solana/web3.js";

// Define our Mint address
const mint = new  PublicKey("BamJPzbY9icNMu6gUozgbvXtfigyEX51MnZL2PQcQtV3")

// Add the Token Metadata Program
const token_metadata_program_id = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

// Create PDA for token metadata
const metadata_seeds = [
    Buffer.from("metadata"),
    token_metadata_program_id.toBuffer(),
    mint.toBuffer(),
];
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(
    metadata_seeds,
    token_metadata_program_id
);

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        let myTransaction = createMetadataAccountV3(umi, {
            //accounts
            metadata: publicKey(metadata_pda.toString()),
            mint: publicKey(mint.toString()),
            mintAuthority: signer,
            payer: signer,
            updateAuthority: keypair.publicKey,
            data: {
                name: "Dogukan",
                symbol: "$DAG",
                uri: "dogukan.com",
                sellerFeeBasisPoints: 0,
                creators: null,
                collection: null,
                uses: null,
            },
            isMutable: true,
            collectionDetails: null,
        });

        let result = await myTransaction.sendAndConfirm(umi);
        console.log(result.signature);
            // Deserialize signature
        const signature = base58.deserialize(result.signature);
        console.log(signature[0]);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();