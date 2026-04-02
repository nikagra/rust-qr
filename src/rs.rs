use crate::gf256::Gf256;

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
    poly
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_2_ec() {
        let gf = Gf256::new();
        let g = generator(&gf, 2);
        assert_eq!(g.len(), 3);
        assert_eq!(g, vec![2, 3, 1]);
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
                .rev()
                .fold(0u8, |acc, &coeff| Gf256::add(gf.mul(acc, alpha_i), coeff));
            assert_eq!(result, 0, "G(α^{i}) should be 0");
        }
    }
}
