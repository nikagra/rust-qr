use crate::gf256::Gf256;

/// Coefficients stored highest degree first
pub fn generator(gf: &Gf256, ec_codewords: usize) -> Vec<u8> {
    let mut poly = vec![0u8; ec_codewords + 1];
    poly[0] = 1;
    for i in 0..ec_codewords {
        let alpha_i = gf.pow(i);
        // Iterate high-to-low so poly[j-1] is still the previous iteration's value
        for j in (0..poly.len()).rev() {
            let d_prev = if j > 0 { poly[j - 1] } else { 0 };
            poly[j] = Gf256::add(d_prev, gf.mul(poly[j], alpha_i));
        }
    }
    poly.reverse();
    poly
}

pub fn encode(gf: &Gf256, data: &[u8], ec_codewords: usize) -> Vec<u8> {
    let g = generator(gf, ec_codewords);
    let mut buf = data.to_vec();
    buf.resize(data.len() + ec_codewords, 0);
    for i in 0..data.len() {
        let lead = buf[i];
        if lead == 0 {
            continue;
        }
        for (j, &coef) in g.iter().enumerate() {
            buf[i + j] ^= gf.mul(coef, lead);
        }
    }
    buf[data.len()..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_2_ec() {
        let gf = Gf256::new();
        let g = generator(&gf, 2);
        assert_eq!(g.len(), 3);
        assert_eq!(g, vec![1, 3, 2]);
    }

    #[test]
    fn test_generator_roots() {
        let gf = Gf256::new();
        let k = 10;
        let g = generator(&gf, k);
        for i in 0..k {
            // Evaluate G(α^i) using Horner's method
            let alpha_i = gf.pow(i);
            let result = g
                .iter()
                .fold(0u8, |acc, &coeff| Gf256::add(gf.mul(acc, alpha_i), coeff));
            assert_eq!(result, 0, "G(α^{i}) should be 0");
        }
    }

    #[test]
    fn test_encode_simple() {
        let gf = Gf256::new();
        let encoded = encode(&gf, &[5, 9], 2);
        assert_eq!(encoded, vec![0, 12]);
    }

    #[test]
    fn test_hello_world_1m() {
        /// "HELLO WORLD", version 1-M, alphanumeric mode (Thonky's tutorial).
        const HELLO_WORLD_1M_DATA: [u8; 16] = [
            32, 91, 11, 120, 209, 114, 220, 77, 67, 64, 236, 17, 236, 17, 236, 17,
        ];
        const HELLO_WORLD_1M_EC: [u8; 10] = [196, 35, 39, 119, 235, 215, 231, 226, 93, 23];
        let gf = Gf256::new();
        let encoded = encode(&gf, &HELLO_WORLD_1M_DATA, HELLO_WORLD_1M_EC.len());
        assert_eq!(encoded, HELLO_WORLD_1M_EC)
    }
}
