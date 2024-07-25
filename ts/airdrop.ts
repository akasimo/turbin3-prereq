import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js"
import wallet from "./temp-wallet.json"

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
console.log(`Public key: ${keypair.publicKey.toBase58()}`);

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
    // We're going to claim 2 devnet SOL tokens
    const txhash = await
    connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);
    console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
    console.error(`Oops, something went wrong: ${e}`)
    }
    })();
    