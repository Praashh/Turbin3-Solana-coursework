import { Keypair } from "@solana/web3.js";

let kp = Keypair.generate() // generating a new keygen a/c cmd -> solana-keygen new 

console.log(`You ve generated a new Solana wallet:  ${kp.publicKey.toBase58()} [${kp.secretKey}]`) // public key-> 8wW2ezBbcjdc8xpF42pDUQ4h21Dv3nbySkdGuSmg7ocD 
