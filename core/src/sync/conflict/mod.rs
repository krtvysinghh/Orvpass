#[derive(Debug)]

pub enum ConflictStrategy {
    LatestWins,

    Manual,

    Merge,
}

pub fn resolve(strategy: ConflictStrategy) -> ConflictStrategy {
    strategy
}
