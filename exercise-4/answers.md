## Check Balances

**Balance of own account**
`$ solana balance`

**Balance of another account**
`$ solana balance 76BMVEPmdpVhwamxe3gSYPVx8NPB3SJR2h2qFziGpPtX `

## Transfer SOL via CLI

`$ solana transfer 76BMVEPmdpVhwamxe3gSYPVx8NPB3SJR2h2qFziGpPtX 0.1 --allow-unfunded-recipient `

Note: Use `--allow-unfunded-recipient` flag when the destination account does not have any SOL (new account).