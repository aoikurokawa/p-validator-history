use quasar_lang::prelude::*;

#[error_code]
pub enum ValidatorHistoryError {
    NotEnoughVotingHistory,
}
