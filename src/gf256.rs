pub struct Gf256 {
    log: [u8; 256],
    exp: [u8; 256],
}

impl Gf256 {
    pub fn new() -> Self {
        let (log, exp) = Self::build_tables();
        Self { log, exp }
    }

    pub fn add(&self, a: u8, b: u8) -> u8 {
        a ^ b
    }

    pub fn mul(&self, a: u8, b: u8) -> u8 {
        if a == 0 || b == 0 {
            return 0;
        }
        self.exp[((self.log[a as usize] as u16 + self.log[b as usize] as u16) % 255) as usize]
    }

    pub fn div(&self, a: u8, b: u8) -> u8 {
        if b == 0 {
            panic!("Division by zero");
        }
        if a == 0 {
            return 0;
        }
        self.exp[((self.log[a as usize] as u16 + 255 - self.log[b as usize] as u16) % 255) as usize]
    }

    fn build_tables() -> ([u8; 256], [u8; 256]) {
        let mut exp = [0u8; 256];
        let mut log = [0u8; 256];
        let mut x = 1u16;
        for i in 0..255 {
            exp[i] = x as u8;
            log[x as usize] = i as u8;
            x <<= 1;
            if x & 0x100 != 0 {
                x ^= 0x11D; // x^8 + x^4 + x^3 + x^2 + 1
            }
        }
        (log, exp)
    }
}

impl Default for Gf256 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let gf = Gf256::new();
        assert_eq!(gf.add(0b0, 0b1), 0b1);
        assert_eq!(gf.add(0b1, 0b1), 0b0);
        assert_eq!(gf.add(0b1010, 0b0101), 0b1111);
        assert_eq!(gf.add(0b01010111, 0b10000011), 0b11010100);
    }

    #[test]
    fn test_mul() {
        let gf = Gf256::new();
        assert_eq!(gf.mul(0b1, 0b0), 0b0);
        assert_eq!(gf.mul(0b1, 0b1), 0b1);
        assert_eq!(gf.mul(0b10, 0b10), 0b100);
        assert_eq!(gf.mul(0b1010, 0b0101), 0b100010); // (x^3+x)(x^2+1) = ... = x^5 + x 
        assert_eq!(gf.mul(0x90, 0x04), 0x7A);
    }

    #[test]
    #[should_panic]
    fn test_div_by_zero_panics() {
        Gf256::new().div(1, 0);
    }

    #[test]
    fn test_div_is_inverse_of_mul() {
        let gf = Gf256::new();
        for a in 1u8..=255 {
            for b in 1u8..=255 {
                assert_eq!(gf.div(gf.mul(a, b), b), a);
            }
        }
    }
}
