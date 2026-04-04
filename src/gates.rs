use ndarray::prelude::*;

pub fn and(x1: i32, x2: i32) -> i32 {
    let x = array![x1 as f64, x2 as f64];
    let w = array![0.5, 0.5];
    let b = -0.7;
    let tmp = (w * x).sum() + b;
    if tmp <= 0.0 { 0 } else { 1 }
}

pub fn nand(x1: i32, x2: i32) -> i32 {
    let x = array![x1 as f64, x2 as f64];
    let w = array![-0.5, -0.5];
    let b = 0.7;
    let tmp = (w * x).sum() + b;
    if tmp <= 0.0 { 0 } else { 1 }
}

pub fn or(x1: i32, x2: i32) -> i32 {
    let x = array![x1 as f64, x2 as f64];
    let w = array![0.5, 0.5];
    let b = -0.2;
    let tmp = (w * x).sum() + b;
    if tmp <= 0.0 { 0 } else { 1 }
}

pub fn xor(x1: i32, x2: i32) -> i32 {
    let s1 = nand(x1, x2);
    let s2 = or(x1, x2);
    and(s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        assert_eq!(and(0, 0), 0);
        assert_eq!(and(0, 1), 0);
        assert_eq!(and(1, 0), 0);
        assert_eq!(and(1, 1), 1);
    }

    #[test]
    fn test_nand() {
        assert_eq!(nand(0, 0), 1);
        assert_eq!(nand(0, 1), 1);
        assert_eq!(nand(1, 0), 1);
        assert_eq!(nand(1, 1), 0);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(0, 0), 0);
        assert_eq!(or(0, 1), 1);
        assert_eq!(or(1, 0), 1);
        assert_eq!(or(1, 1), 1);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(0, 0), 0);
        assert_eq!(xor(0, 1), 1);
        assert_eq!(xor(1, 0), 1);
        assert_eq!(xor(1, 1), 0);
    }
}
