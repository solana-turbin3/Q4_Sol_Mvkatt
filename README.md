# Q4 2024 Builders Cohort

---

### Accounts
 - everything on-chain is an Account
 - space is allocated on creation and can grow over time
 - rent is paid for space
 - can only be created by the System Program

### Accounts flags
- writable
- read only
- signer
- executable

```
{
    key: number,
    lamports: number,
    is_executable: boolean,
    owner: PublicKey
}
```

### Programs
- marked as executable
- stateless, only hold compiled code
- owned by loaders
- can own non-executable accounts
- accessible via their program_id
- Native Programs are provided by Solana
- User Programs are written by us

### Rent
- rent is paid for account creation
- pay 2 years up-front for Rent-Exemption
- closing an account allows rent to be reclaimed
- resizing a program costs rent

### Transactions
- list of instruction that is sent over the rpc - remote procedure call
- must include all accounts that your transaction will reference
- made up of one or more instructions - interface to Solana Programs
- atomic, if any of the instructions fails the transaction fails, the fee will be lost and no change of state occurs

```
{
    message: {
        instructions: Array<Instruction>,
        recent_blockhash: number,
        fee_payer: PublicKey
    },
    signers: Array<Uint8Array>
}
```

### Compute
- on-chain actions require compute units
- Solana has a max numer of compute units per block
- you can request extra compute units if needed

### PDA
- made up of seeds and a bump
- can be deterministic is seeds are chosen well
- can't collide with PDAs or Accounts created by other programs
- can be used as hashmap (key/value)
- PDA ids look similar to addresses but have no corresponding private key
- can sign on behalf of a Program

### IDL
- Interface Design Language
- many on-chain programs have an IDL
- makes interacting with on-chain programs much easier

### SPL Token
- used to create Fungible and Non-Fungible Tokens
- must first create the Mint - specific type of token
- spl-token library provides a function createMint

### Associated Token Account
- one of the most used PDAs on Solana
- creates deterministic token account
- other people can create token account for you
- make easier you tell if someone has an account to accept a particular token

---

## Blinks
- embedded dApp widgets, injected natively into the website
- blinks == frontend, detect actions with browser extensions
- ways to integrate
  - solana-action: uri scheme
  - actions.json: maps normal urls on your website to Action API endpoints
  - interstitial website: dial.to, allows to build Applications without building FE UI and wallet connect UX
- actions == backend, public API URLs on the internet (REST)
  - accept GET and POST request
  - return signable transactions and eventually messages



