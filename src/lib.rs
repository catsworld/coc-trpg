#[cfg(test)]
mod tests;

use rand::Rng;

pub enum RollRes {
    Success(RollPoint, SuccessLevel, Critical),
    Fail(RollPoint, Fumble),
}

pub type RollPoint = i32;

#[derive(PartialEq)]
pub enum SuccessLevel {
    Regular,
    Hard,
    Extreme,
}

pub type Critical = bool;
pub type Fumble = bool;

pub fn roll(rate: i32) -> RollRes {
    use RollRes::*;
    use SuccessLevel::*;

    let point = rand::thread_rng().gen_range(1..=100);

    if point <= rate {
        Success(
            point,
            if point <= rate / 5 {
                Extreme
            } else if point <= rate / 2 {
                Hard
            } else {
                Regular
            },
            (1..=100).contains(&rate) && point == 1,
        )
    } else {
        Fail(
            point,
            ((1..50).contains(&rate) && point > 95) || ((51..=100).contains(&rate) && point == 100),
        )
    }
}
