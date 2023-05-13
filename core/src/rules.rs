#[derive(PartialEq, Eq, Copy, Clone)]

/// The different ways to stack plus cards in Uno
pub enum PlusStacking {
    /// Plus Stacking is never allowed
    Banned,

    /// 1. Plus Twos stack on eachother
    /// 2. Plus Fours stack on Plus Twos or Plus Fours
    /// 3. Plus Twos DO NOT stack on Plus Fours
    Conservative,

    /// Similar to Conservative,
    /// except Plus Twos CAN stack on Plus Fours as long
    /// as the Plus Two has the same color as the Plus Four
    Liberal,
}

///
#[derive(Copy, Clone)]
pub struct Rules {
    pub plus_stacking: PlusStacking,
    must_play: bool,
    draw_until_match: bool,
}

impl Rules {
    pub fn default() -> Rules {
        Rules {
            plus_stacking: PlusStacking::Conservative,
            must_play: false,
            draw_until_match: false,
        }
    }
}
