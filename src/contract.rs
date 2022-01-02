use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum ContractType {
    InversePerpetual,
    LinearPerpetual,
    InverseFutures,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum ContractStatus {
    Trading,
    Settling,
    Closed,
    // The `Pending` status doesn't appear anywhere in the API docs, however it has been spotted in the wild.
    Pending,
}

impl Default for ContractStatus {
    fn default() -> Self {
        ContractStatus::Trading
    }
}
