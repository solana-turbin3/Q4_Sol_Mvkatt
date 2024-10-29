import wallet from "../../wallets/Turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey, percentAmount } from "@metaplex-foundation/umi";
import { PublicKey } from "@solana/web3.js";
import bs58 from "bs58";

// Define our Mint address
const mint = publicKey(new PublicKey("3bnxmwySgXJH4HHphiWn8to4AWGQLUCqp1jbJgqnHw6S"));


// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
// Generate a new signer and immediately assign it to the identity and/or payer attributes
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
    try {
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            metadata: undefined,
            mint,
            mintAuthority: signer,
            payer: signer,
        }

        let data: DataV2Args = {
            name: "Test new Mint",
            symbol: "MV3",
            uri: "",
            sellerFeeBasisPoints: 1,
            collection: null,
            creators: null,
            uses: null
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: true,
            collectionDetails: null
        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
        // GkaiVNgAEDsCZT5f45TjoBieuprNVWdSWYxj5uZmYw1Jqup2Z2E2k1TuyN7sNCe3MyDymnYA3TgVBM3Y6YPGzhX
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
