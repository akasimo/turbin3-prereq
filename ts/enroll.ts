import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, WbaPrereq } from "./programs/wba_prereq";
import wallet from "./dev-wallet.json"

// Create a Keypair from the secret key stored in the dev-wallet.json file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Establish a connection to the Solana devnet
const connection = new Connection("https://api.devnet.solana.com");

// Convert the GitHub username to a Buffer
const github = Buffer.from("akasimo", "utf8");

// Create an AnchorProvider using the connection, wallet, and confirmation settings
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed" });

// Create a Program instance using the IDL and provider
const program: Program<WbaPrereq> = new Program(IDL, provider);

// Generate the enrollment seeds using the "prereq" string and the user's public key
const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];

// Find the program address for the enrollment account using the enrollment seeds and program ID
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(enrollment_seeds, program.programId);

// Define an async function to handle the enrollment process
(async () => {
    try {
        // Call the "complete" method of the program, passing the GitHub username
        const txhash = await program.methods
            .complete(github)
            .accounts({
                signer: keypair.publicKey,
            })
            .signers([
                keypair
            ]).rpc();
        console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        // Log an error message if something goes wrong
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
