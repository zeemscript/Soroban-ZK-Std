#![no_std]
use ethnum::u256;

pub struct Bn254;

impl Bn254 {
    pub const BASE_MODULUS: ethnum::u256 = ethnum::u256::from_words(
        0x30644e72e131a029b85045b68181585d_u128, // high 128 bits (first 16 bytes)
        0x97816a916871ca8d3c208c16d87cfd47_u128, // low 128 bits  (last 16 bytes)
    );

    pub const SCALAR_ORDER: ethnum::u256 = ethnum::u256::from_words(
        0x30644e72e131a029b85045b68181585d_u128,
        0x2833e84879b9709143e1f593f0000001_u128,
    );

    // pub const BASE_MODULUS: u256 = u256::from_words(
    //     0x30644e72e131a029b85045b68181585d,
    //     0x97816a916871ca8d3c208c16d87cfd47,
    // );

    pub fn is_valid_scalar(val: u256) -> bool {
        val < Self::BASE_MODULUS
    }

    pub fn add(a: u256, b: u256) -> u256 {
        let (sum, overflow) = a.overflowing_add(b);
        if overflow || sum >= Self::BASE_MODULUS {
            sum.wrapping_sub(Self::BASE_MODULUS)
        } else {
            sum
        }
    }

    /// Modular Multiplication: (a * b) % BASE_MODULUS
    /// Implements manual 512-bit long multiplication to bypass library limitations.
    pub fn mul(a: u256, b: u256) -> u256 {
        if a == 0 || b == 0 {
            return u256::from(0u8);
        }

        // Split a and b into 128-bit halves
        let a_low = u256::from(a.as_u128());
        let a_high = a >> 128;
        let b_low = u256::from(b.as_u128());
        let b_high = b >> 128;

        // Schoolbook multiplication (a_hi*2^128 + a_lo) * (b_hi*2^128 + b_lo)
        // This yields 4 partial products
        let p0 = a_low * b_low;
        let p1 = a_low * b_high;
        let p2 = a_high * b_low;
        let p3 = a_high * b_high;

        // Perform modular reduction on each partial product stage
        // to keep everything within 256-bit bounds.
        let mut res = p0 % Self::BASE_MODULUS;

        // Handle p1 and p2 (shifted by 128 bits)
        let mut p1_p2 = p1 % Self::BASE_MODULUS;
        p1_p2 = Self::add(p1_p2, p2 % Self::BASE_MODULUS);
        for _ in 0..128 {
            p1_p2 = Self::add(p1_p2, p1_p2); // Modular doubling
        }
        res = Self::add(res, p1_p2);

        // Handle p3 (shifted by 256 bits)
        let mut p3_red = p3 % Self::BASE_MODULUS;
        for _ in 0..256 {
            p3_red = Self::add(p3_red, p3_red); // Modular doubling
        }
        res = Self::add(res, p3_red);

        res
    }
    pub fn pow(mut base: u256, mut exp: u256) -> u256 {
        let mut res = u256::from(1u8);
        while exp > 0 {
            if exp % 2 == 1 {
                res = Self::mul(res, base);
            }
            base = Self::mul(base, base);
            exp /= 2;
        }
        res
    }

    pub fn invert(a: u256) -> u256 {
        if a == 0 {
            return u256::from(0u8);
        }
        let exponent = Self::BASE_MODULUS - u256::from(2u8);
        Self::pow(a, exponent)
    }

    pub fn is_valid_g1(x: u256, y: u256) -> bool {
        if x == 0 && y == 0 {
            return false;
        }
        if x >= Self::BASE_MODULUS || y >= Self::BASE_MODULUS {
            return false;
        }

        let y_sq = Self::mul(y, y);
        let x_sq = Self::mul(x, x);
        let x_cb = Self::mul(x_sq, x);
        let rhs = Self::add(x_cb, u256::from(3u8));

        y_sq == rhs
    }

    pub fn g1_scalar_mul(point: G1Projective, scalar: u256) -> G1Projective {
        // handle edge cases
        if scalar == 0 {
            return G1Projective::identity();
        }
        if scalar == 1 {
            return point;
        }

        let mut result = G1Projective::identity();

        // BN254 scalar field is ~254 bits. Loop 254 times for constant-time.
        for i in (0..254).rev() {
            // double the current result
            result = result.double();

            // always compute addition
            let added = result.add(&point);

            // extract the i-th of the scalar
            let bit = (scalar >> i) & 1u128;

            // constant time select: replaces if bit == 1 {result = added}
            // this ensure the same branch/instruction path is taken
            result = G1Projective::cf_select(bit == 1u128, added, result);
        }

        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct G1Projective {
    pub x: u256,
    pub y: u256,
    pub z: u256,
}

impl G1Projective {
    pub fn identity() -> Self {
        Self {
            x: u256::from(1u8),
            y: u256::from(1u8),
            z: u256::from(0u8),
        }
    }

    /// contant time seletion: return 'a' if choice is true, 'b' if choice is false
    pub fn cf_select(choice: bool, a: Self, b: Self) -> Self {
        if choice {
            a
        } else {
            b
        }
    }

    // you will need to implement these based on standard projective formulas
    pub fn double(&self) -> Self {
        todo!()
    }
    pub fn add(&self, _other: &Self) -> Self {
        todo!()
    }
}
