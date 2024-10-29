import base58 from "bs58";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi";
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../../wallets/Turbin3-wallet.json";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi, {
        mint,
        name: "Mv NFT Test",
        symbol: "MVT",
        uri: "https://devnet.irys.xyz/335TRugnTzfi7MMrXmi2khN9Z4zb2aBnNbayeMFwXstQ",
        sellerFeeBasisPoints: percentAmount(1)
    });

    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);

    // Succesfully Minted! Check out your TX here:
    // https://explorer.solana.com/address/GwkzURzyffGLv1KWyfP1uH8LUKpBMMSVmfiZVh1u9Cad?cluster=devnet
    // Mint Address:  GwkzURzyffGLv1KWyfP1uH8LUKpBMMSVmfiZVh1u9Cad

})();