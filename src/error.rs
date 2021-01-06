#[derive(Debug)]
pub enum RunLimit {
    TooManyFacts,
    TooManyIterations,
    Timeout,
}
