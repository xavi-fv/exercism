fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut b: u128 = base as u128;
    let mut e: u128 = exponent as u128;
    let m: u128 = modulus as u128;

    let mut result: u128 = 1;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % m;
        }
        e = e >> 1;
        b = (b * b) % m;
    }
    result as u64
}

pub fn private_key(p: u64) -> u64 {
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}
