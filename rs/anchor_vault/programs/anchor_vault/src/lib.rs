use anchor_lang::prelude::*;
use anchor_lang::system_program::{ Transfer, transfer };

declare_id!("JEJMHyemEsbq6oDwnmZGr5WmNRubu1vb1ESDpdCEjurE");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;

        Ok(())
    }
    
    pub fn deposit(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        
        Ok(())
    }
    
    pub fn withdraw(ctx: Context<Operations>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        
        Ok(())
    }
    
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;

        Ok(())
    }
}

// 'info - lifetime specifier telling the compiler that every trait marked by this 
// will live for at least the same amount of time
// and will not be dropped one before the other
#[derive(Accounts)]
// create Initialize context and pass all the accounts used in any method of it
pub struct Initialize<'info> {
    // user account is mutable since we are going to deduct lamports from it
    #[account(mut)]
    pub user: Signer<'info>, // anchor makes sure that isSigner == true
    #[account(
        init, // System Program will init the account, and we will claim ownership on it
        payer = user, // user is going to pay to the initialization fees for the vault_state account
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    // vault is a system account since we are using native sol
    pub vault: SystemAccount<'info>,
    // system program is responsible for creation of every new account
    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    // init the canonical bumps
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Operations<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // withdraw into this account
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>, // withdraw from this account
    #[account(
        seeds = [b"state", user.key().as_ref()], // deriving from users pubkey, so the vault is derived from the user
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System> // for native token transfer we need the system program, for other tokens - token program
}

impl<'info> Operations<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };
        
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer_seeds
        );

        transfer(cpi_ctx, amount)
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        // TODO: check if the deposit amount is equal or bigger than the amount of rent exemption
        // check also if the vault already has enough lamports equal or bigger than the amount of rent exemption
        
        
        // calc min rent exemption : https://docs.rs/solana-program/latest/solana_program/rent/struct.Rent.html#method.minimum_balance
        // check if:
        // amount >= rent exemption amount
        // lamports inside the vault >= rent exemption amount

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user // will send the rent back to the specified account
    )]
    pub vault_state: Account<'info, VaultState>,
    pub system_program: Program<'info, System>
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };


        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            signer_seeds
        );

        // transfer all the lamports inside the vault, the system account will close it automatically 
        // when the balance reaches 0
        // if it were an ATA, we would have to close it manually
        transfer(cpi_ctx, self.vault.lamports())
    }
}

// create vault state account, initialize the bumps for PDAs
#[account]
// #[derive(InitSpace)] // does not take the 8 bytes into consideration, does the same as impl Space below
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

// space trait, identify the space the vault will take on-chain (rent exemption)
impl Space for VaultState {
    // 8 - anchor discriminator
    // 1 - vault_bump
    // 1 - state_bump
    const INIT_SPACE: usize = 8 + 1 + 1;
}