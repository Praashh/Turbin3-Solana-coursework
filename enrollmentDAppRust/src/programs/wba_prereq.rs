use solana_idlgen::idlgen;

idlgen!({
    "version": "0.1.0",
    "name": "wba_prereq",
    "instructions": [
        {
            "name": "complete",
            "accounts": [
                {
                    "name": "signer",
                    "isMut": true,
                    "isSigner": true
                },
                {
                    "name": "prereq",
                    "isMut": true,
                    "isSigner": false
                },
                {
                    "name": "systemProgram",
                    "isMut": false,
                    "isSigner": false
                }
            ],
            "args": [
                {
                    "name": "github",
                    "type": "bytes"
                }
            ]
        }
    ],
    "metadata": {
        "address": "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1"
    }
});