pub fn to_binary(n: u32) -> Vec<u8> {
    let mut n = n;
    let mut bits = vec![];

    if n == 0 {
        return vec![0];
    }

    while n > 0 {
        let r = (n % 2) as u8;
        n = n / 2;

        bits.push(r);
    }

    bits.reverse();
    bits
}

pub fn to_decimal(bits: &[u8]) -> Option<u32> {
    if bits.len() == 0 {
        return None;
    }

    let mut n = 0;
    let mut i = bits.len() - 1;

    while i > 0 {
        n += bits[i as usize] as u32 * (2 ^ i) as u32;
        i -= 1;
    }

    Some(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_binary() {
        assert_eq!(to_binary(0), vec![0]);
        assert_eq!(to_binary(4), vec![1, 0, 0]);
        assert_eq!(to_binary(54), vec![1, 1, 0, 1, 1, 0]);
    }

    #[test]
    fn test_to_decimal() {
        assert_eq!(to_decimal(&[]), None);
        assert_eq!(to_decimal(&[0]), Some(0));
        assert_eq!(to_decimal(&[1, 0, 0]), Some(4));
    }
}
