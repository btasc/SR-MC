#[derive(Clone, Copy, Debug)]
pub enum RoundMethod {
    ToNearest,
    Stochastic,
}

pub enum RoundResult {
    Up, Down,
}

pub trait Round {
    const ROUND_METHOD: RoundMethod;
    type Width;
    
    // u32 represents fixed point 0-1 distance from lower bound to upper bound of the number
    fn get_fract_dist(&self, new_width: Self::Width) -> u32;
}

impl RoundMethod {
    pub fn round(fract_dist: T, method: RoundMethod) -> RoundResult {
        
        match method {
            RoundMethod::ToNearest => {
                
            },
            RoundMethod::Stochastic => {},
        }
    }
}