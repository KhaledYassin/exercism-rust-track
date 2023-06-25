use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}

fn modular_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        0
    } else {
        let mut result = 1_u128;
        let mut b = base as u128;
        let mut e = exponent as u128;
        let m = modulus as u128;

        while e > 0 {
            if e % 2 == 1 {
                result = (result * b) % m;
            }

            b = (b * b) % m;
            e >>= 1;
        }

        result as u64
    }
}
