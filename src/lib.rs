use rand::Rng;

pub type RollPoint = i32;

pub enum RollRes {
    Success(RollPoint, SuccessLevel, Critical),
    Fail(RollPoint, Fumble),
}

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

#[cfg(test)]
mod tests {
    use crate::*;
    use RollRes::*;
    use SuccessLevel::*;

    #[test]
    fn rate_46() {
        for _ in 0..10000 {
            let roll_res = roll(46);

            match roll_res {
                Success(point, level, critical) => {
                    assert!(point >= 1 && point <= 100);
                    assert!(point <= 46);

                    if point <= 9 {
                        assert!(level == Extreme);
                    } else if point <= 23 {
                        assert!(level == Hard);
                    } else {
                        assert!(level == Regular);
                    }

                    if point == 1 {
                        assert!(critical);
                    } else {
                        assert!(!critical);
                    }
                }
                Fail(point, fumble) => {
                    assert!(point >= 1 && point <= 100);
                    assert!(point > 46);

                    if point > 95 {
                        assert!(fumble);
                    } else {
                        assert!(!fumble);
                    }
                }
            };
        }
    }

    #[test]
    fn rate_53() {
        for _ in 0..10000 {
            match roll(53) {
                Success(point, level, critical) => {
                    assert!(point >= 1 && point <= 100);
                    assert!(point <= 53);

                    if point <= 10 {
                        assert!(level == Extreme);
                    } else if point <= 26 {
                        assert!(level == Hard);
                    } else {
                        assert!(level == Regular);
                    }

                    if point == 1 {
                        assert!(critical);
                    } else {
                        assert!(!critical);
                    }
                }
                Fail(point, fumble) => {
                    assert!(point >= 1 && point <= 100);
                    assert!(point > 53);

                    if point == 100 {
                        assert!(fumble);
                    } else {
                        assert!(!fumble);
                    }
                }
            };
        }
    }

    #[test]
    fn rate_100() {
        for _ in 0..10000 {
            if let Success(point, level, critical) = roll(100) {
                assert!(point >= 1 && point <= 100);

                if point <= 20 {
                    assert!(level == Extreme);
                } else if point <= 50 {
                    assert!(level == Hard);
                } else {
                    assert!(level == Regular);
                }

                if point == 1 {
                    assert!(critical);
                } else {
                    assert!(!critical);
                }
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn rate_0() {
        for _ in 0..10000 {
            if let Fail(point, fumble) = roll(0) {
                assert!(point >= 1 && point <= 100);
                assert!(!fumble);
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn rate_over_100() {
        for _ in 0..10000 {
            if let Success(point, level, critical) = roll(246) {
                assert!(point >= 1 && point <= 100);

                if point <= 49 {
                    assert!(level == Extreme);
                } else {
                    assert!(level == Hard);
                }

                assert!(!critical);
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn rate_below_1() {
        for _ in 0..10000 {
            if let Fail(point, fumble) = roll(-35) {
                assert!(point >= 1 && point <= 100);
                assert!(!fumble);
            } else {
                assert!(false);
            }
        }
    }
}
