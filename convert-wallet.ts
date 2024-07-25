import bs58 from 'bs58';
import prompt from 'prompt-sync';

const ps = prompt();

// Function to convert base58 encoded private key to a byte array
function base58ToWallet() {
    const base58 = ps('Enter your base58 encoded private key: ');
    try {
        const wallet = bs58.decode(base58);
        console.log('Byte array:', wallet);
    } catch (error) {
        console.error('Error decoding base58:', error);
    }
}

// Function to convert a byte array to base58 encoded private key
function walletToBase58() {
    const walletStr = ps('Enter your wallet byte array (comma separated): ');
    const wallet = walletStr.split(',').map(Number);
    try {
        const base58 = bs58.encode(Buffer.from(wallet));
        console.log('Base58 encoded private key:', base58);
    } catch (error) {
        console.error('Error encoding to base58:', error);
    }
}

const operation = ps('Choose operation (1: base58 to wallet, 2: wallet to base58): ');
if (operation === '1') {
    base58ToWallet();
} else if (operation === '2') {
    walletToBase58();
} else {
    console.log('Invalid operation chosen.');
}
