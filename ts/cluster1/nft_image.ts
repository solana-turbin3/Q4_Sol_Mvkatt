import wallet from "../../wallets/Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        const imageBuffer = await readFile("/Users/katekhremuchkova/RustroverProjects/solana-starter/ts/nft_img.png");
        //2. Convert image to generic file.
        const genericImage = createGenericFile(imageBuffer, "nft_image", {
            contentType: "image/png"
        });
        //3. Upload image
        const [uri] = await umi.uploader.upload([genericImage]);

        console.log("Your image URI: ", uri);
        // mainnet: https://arweave.net/BtkV2xXUU9PovfmFQhbKm8qmyMFByrXeroKbkwzoAAC7
        // devnet: https://devnet.irys.xyz/BtkV2xXUU9PovfmFQhbKm8qmyMFByrXeroKbkwzoAAC7
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
