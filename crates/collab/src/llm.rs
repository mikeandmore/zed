pub mod db;
mod token;

pub use token::*;

pub const AGENT_EXTENDED_TRIAL_FEATURE_FLAG: &str = "agent-extended-trial";

/// The name of the feature flag that bypasses the account age check.
pub const BYPASS_ACCOUNT_AGE_CHECK_FEATURE_FLAG: &str = "bypass-account-age-check";

/// The minimum account age an account must have in order to use the LLM service.
pub const MIN_ACCOUNT_AGE_FOR_LLM_USE: chrono::Duration = chrono::Duration::days(30);
