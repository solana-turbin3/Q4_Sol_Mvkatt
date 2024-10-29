import wallet from "../../wallets/Turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const imageUri = "https://devnet.irys.xyz/B1z4k8iu9uUVquDmz3va13d5JRx4Kr9qHQqXnttbX7AU";

        const metadata = {
            name: "Mv Test",
            symbol: "MVT",
            description: "This is my test NFT",
            image: imageUri,
            attributes: [
                { trait_type: 'color', value: 'purple' },
                { trait_type: 'material', value: 'rock'},
                { trait_type: 'size', value: '50' }
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imageUri
                    },
                ]
            },
            creators: [keypair.publicKey]
        };

        const myUri = await umi.uploader.uploadJson(metadata);

        console.log("Your metadata URI: ", myUri);
        // Your metadata URI:  https://arweave.net/G3E1dGhmm2Vidt8uCGuvUBahJdVjAnxcYbggbyd8dyMS
        // Your metadata URI: devnet: https://devnet.irys.xyz/G3E1dGhmm2Vidt8uCGuvUBahJdVjAnxcYbggbyd8dyMS
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
