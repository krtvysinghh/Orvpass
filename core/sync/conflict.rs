use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ConflictStrategy {

    LatestWriteWins,
    ManualMerge,
    KeepBoth,

}

pub fn resolve(
strategy: ConflictStrategy
)->ConflictStrategy {

strategy

}
