#[derive(Clone, Copy, Debug)]
pub enum RoundMethod {
    ToNearest,
    Stochastic,
}

pub trait Round {
    const ROUND_METHOD: RoundMethod;
}

impl RoundMethod {
    pub fn round<T: Round>(num: T) {
        match T::ROUND_METHOD {
            RoundMethod::ToNearest => {},
            RoundMethod::Stochastic => {},
        }
    }
}