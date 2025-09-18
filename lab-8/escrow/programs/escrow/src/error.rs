use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid offer")]
    InvalidOffer,
    #[msg("Invalid amount")]
    InvalidAmount,
}
