use anchor_lang::prelude::*;

declare_id!("Gm1CFRsZHsyhqd1mYZSoVZvLrssPM4kNj2NdHShx1rn7");

#[program]
pub mod brick {
    use super::*;

    pub fn fallback(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> Result<()> {
        Err(ErrorCode::ProgramDisabled.into())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("This program is temporarily disabled.")]
    ProgramDisabled,
}
