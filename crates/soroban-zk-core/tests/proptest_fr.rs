use ethnum::u256;
use proptest::prelude::*;
use soroban_zk_core::Bn254;

fn fr_from_seed(seed: [u8; 32]) -> u256 {
    u256::from_be_bytes(seed) % Bn254::FR_MODULUS
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn additive_identity_holds(a_bytes in any::<[u8; 32]>()) {
        let a = fr_from_seed(a_bytes);
        prop_assert_eq!(Bn254::add(a, u256::from(0u8)), a);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn additive_inverse_holds(a_bytes in any::<[u8; 32]>()) {
        let a = fr_from_seed(a_bytes);
        let neg_a = Bn254::sub(u256::from(0u8), a);
        prop_assert_eq!(Bn254::add(a, neg_a), u256::from(0u8));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn addition_is_commutative(a_bytes in any::<[u8; 32]>(), b_bytes in any::<[u8; 32]>()) {
        let a = fr_from_seed(a_bytes);
        let b = fr_from_seed(b_bytes);
        prop_assert_eq!(Bn254::add(a, b), Bn254::add(b, a));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn addition_is_associative(
        a_bytes in any::<[u8; 32]>(),
        b_bytes in any::<[u8; 32]>(),
        c_bytes in any::<[u8; 32]>(),
    ) {
        let a = fr_from_seed(a_bytes);
        let b = fr_from_seed(b_bytes);
        let c = fr_from_seed(c_bytes);
        prop_assert_eq!(Bn254::add(Bn254::add(a, b), c), Bn254::add(a, Bn254::add(b, c)));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn multiplicative_identity_holds(a_bytes in any::<[u8; 32]>()) {
        let a = fr_from_seed(a_bytes);
        prop_assert_eq!(Bn254::mul(a, u256::from(1u8)), a);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn multiplicative_inverse_holds(a_bytes in any::<[u8; 32]>()) {
        let a = fr_from_seed(a_bytes);
        prop_assume!(a != u256::from(0u8));

        let inv_a = Bn254::invert(a);
        prop_assert_eq!(Bn254::mul(a, inv_a), u256::from(1u8));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn multiplication_is_commutative(a_bytes in any::<[u8; 32]>(), b_bytes in any::<[u8; 32]>()) {
        let a = fr_from_seed(a_bytes);
        let b = fr_from_seed(b_bytes);
        prop_assert_eq!(Bn254::mul(a, b), Bn254::mul(b, a));
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn multiplication_distributes_over_addition(
        a_bytes in any::<[u8; 32]>(),
        b_bytes in any::<[u8; 32]>(),
        c_bytes in any::<[u8; 32]>(),
    ) {
        let a = fr_from_seed(a_bytes);
        let b = fr_from_seed(b_bytes);
        let c = fr_from_seed(c_bytes);
        prop_assert_eq!(
            Bn254::mul(Bn254::add(a, b), c),
            Bn254::add(Bn254::mul(a, c), Bn254::mul(b, c))
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]
    #[test]
    fn fermats_little_theorem_holds(a_bytes in any::<[u8; 32]>()) {
        let a = fr_from_seed(a_bytes);
        prop_assume!(a != u256::from(0u8));

        let exponent = Bn254::FR_MODULUS - u256::from(1u8);
        prop_assert_eq!(Bn254::pow(a, exponent), u256::from(1u8));
    }
}
