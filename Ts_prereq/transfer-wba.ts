import { Transaction, SystemProgram, Connection, Keypair,
    LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from
    "@solana/web3.js"
import wallet from "./dev-wallet.json"
import dotenv from "dotenv"
dotenv.config();


const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection(process.env.RPC_CONNECTION!);
const to = new
PublicKey(process.env.WBA_PUBLIC_KEY!);
const solToTransfer = 0.1;


async function transferSolToWbaWallet () {
    try {
    const transaction = new Transaction().add(
    SystemProgram.transfer({
    fromPubkey: from.publicKey,
    toPubkey: to,
    lamports: LAMPORTS_PER_SOL*solToTransfer,
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
};

// transferSolToWbaWallet(); uncomment you wanna transfer sol to WBA wallet

async function EmptyWallet() {
    try {
    // Get balance of dev wallet
    const balance = await connection.getBalance(from.publicKey)
    // Create a test transaction to calculate fees
    const transaction = new Transaction().add(
    SystemProgram.transfer({
    fromPubkey: from.publicKey,
    toPubkey: to,
    lamports: balance,
    })
    );
    transaction.recentBlockhash = (await
    
    connection.getLatestBlockhash('confirmed')).blockhash;
    
    transaction.feePayer = from.publicKey;

    
    const fee = (await
    
    connection.getFeeForMessage(transaction.compileMessage(),
    'confirmed')).value || 0;
    
    // Remove our transfer instruction to replace it
    transaction.instructions.pop();
    
    
    transaction.add(
    SystemProgram.transfer({
    fromPubkey: from.publicKey,
    toPubkey: to,
    lamports: balance - fee,
    })
    );
    // Sign transaction, broadcast, and confirm
    const signature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [from]
    );
    console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${signature}?cluster=devnet`)
    } catch(e) {
    console.error(`Oops, something went wrong: ${e}`)
    }
};


// EmptyWallet(); uncomment when you wanna empty your devnet WBA wallet