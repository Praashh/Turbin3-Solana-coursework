import { Transaction, SystemProgram, Connection, Keypair,
    LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from
    "@solana/web3.js"
import wallet from "./dev-wallet.json"
import dotenv from "dotenv"
dotenv.config();


const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection(process.env.RPC_CONNECTION!);
const to = new
PublicKey(process.env.TRANSFER_TO_PUBLIC_KEY!);
const solToTransfer = 0.1;



(async () => {
    try {
    const transaction = new Transaction().add(
    SystemProgram.transfer({
    fromPubkey: from.publicKey,
    toPubkey: to,
    lamports: LAMPORTS_PER_SOL* solToTransfer,
    })
    );
    transaction.recentBlockhash = (await
    
    connection.getLatestBlockhash('confirmed')).blockhash;
    
    transaction.feePayer = from.publicKey;
    // Sign transaction, broadcast, and confirm
    const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [from]
);
console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${signature}?cluster=devnet`);
} catch(e) {
console.error(`Oops, something went wrong: ${e}`)
}
})();