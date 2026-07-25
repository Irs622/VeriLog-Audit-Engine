use anchor_lang::prelude::*;

// Replace with your actual program ID when deployed
declare_id!("VeriLog111111111111111111111111111111111111");

#[program]
pub mod verilog_audit {
    use super::*;

    pub fn record_audit_log(
        ctx: Context<RecordAuditLog>, 
        merkle_root: [u8; 32], 
        service_id: String,
        batch_id: String
    ) -> Result<()> {
        let log_account = &mut ctx.accounts.log_account;
        
        log_account.merkle_root = merkle_root;
        log_account.timestamp = Clock::get()?.unix_timestamp;
        log_account.service_id = service_id;
        log_account.batch_id = batch_id;
        log_account.authority = *ctx.accounts.authority.key;
        
        msg!("Audit log recorded for service: {} | batch: {}", log_account.service_id, log_account.batch_id);
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(merkle_root: [u8; 32], service_id: String, batch_id: String)]
pub struct RecordAuditLog<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + AuditLogState::INIT_SPACE,
        seeds = [b"audit", authority.key().as_ref(), service_id.as_bytes(), batch_id.as_bytes()],
        bump
    )]
    pub log_account: Account<'info, AuditLogState>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct AuditLogState {
    pub merkle_root: [u8; 32],
    pub timestamp: i64,
    #[max_len(64)]
    pub service_id: String,
    #[max_len(64)]
    pub batch_id: String,
    pub authority: Pubkey,
}
