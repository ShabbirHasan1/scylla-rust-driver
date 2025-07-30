//! Policies to decide whether to retry a request and how to do so.

mod default;
mod downgrading_consistency;
mod fallthrough;
mod retry_policy;

pub use default::{DefaultRetryPolicy, DefaultRetrySession};
pub use downgrading_consistency::{
    DowngradingConsistencyRetryPolicy, DowngradingConsistencyRetrySession,
};
pub use fallthrough::{FallthroughRetryPolicy, FallthroughRetrySession};
pub use retry_policy::{RequestInfo, RetryDecision, RetryPolicy, RetrySession};
