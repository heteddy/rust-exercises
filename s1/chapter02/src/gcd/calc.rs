

pub fn gcd_calc_function(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        // 交换让m为大数
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd_calc_function(14, 15), 1);
        assert_eq!(gcd_calc_function(2 * 3 * 7, 11 * 7), 7);
    }
}
