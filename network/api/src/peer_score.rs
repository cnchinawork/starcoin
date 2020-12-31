pub trait Score<Entry> {
    fn execute(&self, entry: Entry) -> i64;
}

#[derive(Clone)]
pub struct InverseScore {
    k: i64,
}

impl InverseScore {
    pub fn new(x: u32, y: u32) -> Self {
        assert!(y < 100);
        Self { k: (x * y) as i64 }
    }
}

impl Score<u32> for InverseScore {
    fn execute(&self, time: u32) -> i64 {
        assert!(time > 0);
        self.k / (time as i64)
    }
}

pub enum HandleState {
    Future,
    Succ,
    Fail,
}

pub struct BlockBroadcastEntry {
    new: bool,
    state: HandleState,
}

#[derive(Clone)]
pub struct LinearScore {
    base: i64,
}

impl LinearScore {
    pub fn new(base: i64) -> Self {
        assert!(base > 0);
        Self { base }
    }

    pub fn linear(&self) -> i64 {
        self.base
    }

    pub fn percentage(&self, percent: usize) -> i64 {
        assert!(percent <= 100);
        self.base * (percent as i64) / 100
    }
}

impl Score<BlockBroadcastEntry> for LinearScore {
    fn execute(&self, entry: BlockBroadcastEntry) -> i64 {
        match entry.state {
            HandleState::Future => {
                if entry.new {
                    self.percentage(50)
                } else {
                    self.percentage(5)
                }
            }
            HandleState::Succ => {
                if entry.new {
                    self.linear()
                } else {
                    self.percentage(10)
                }
            }
            HandleState::Fail => -self.base,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::peer_score::{InverseScore, Score};
    use starcoin_logger::prelude::*;

    #[test]
    fn test_inverse_score() {
        let inverse_score = InverseScore::new(100, 90);
        let mut score = inverse_score.execute(50);
        info!("{:?}", score);
        assert!(score > 90);
        score = inverse_score.execute(100);
        info!("{:?}", score);
        assert_eq!(score, 90);
        score = inverse_score.execute(200);
        info!("{:?}", score);
        assert!(score < 90);
    }
}
