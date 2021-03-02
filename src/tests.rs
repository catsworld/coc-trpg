
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
