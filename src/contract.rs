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
}

impl Default for ContractStatus {
    fn default() -> Self {
        ContractStatus::Trading
    }
}
