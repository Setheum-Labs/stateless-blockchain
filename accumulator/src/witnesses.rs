/// Membership Witness Management

use primitive_types::U256;
use crate::subroutines;
use crate::proofs;
use rstd::prelude::Vec;

/// Verifies that a membership witness + proof are valid. Acts as a wrapper for the proof of
/// exponentiation verifier.
pub fn verify_mem_wit(state: U256, elem: U256, witness: U256, proof: U256) -> bool {
    return proofs::verify_poe(witness, elem, state, proof);
}

/// Updates a membership witness based on untracked additions and deletions. Algorithm is based on
/// section 3.2 of the paper titled "Dynamic Accumulators and Applications to Efficient Revocation of
/// Anonymous Credentials". Note that "additions" represent the product of the added elements
/// and "deletions" represents the product of the deleted elements.
/// NOTE: Does not do any error checking on unwrap.
pub fn update_mem_wit(elem: U256, mut witness: U256, mut new_state: U256, additions: U256,
                      deletions: U256) -> U256 {
    // Handle added elems
    witness = subroutines::mod_exp(witness, additions, U256::from(super::MODULUS));

    // Handle deleted elems
    witness = subroutines::shamir_trick(witness, new_state, elem, deletions).unwrap();
    return witness;
}

/// Takes two elements + membership witnesses and returns the aggregated witness and aggregated proof.
/// NOTE: Does very little error checking (Ex: Does not do any error checking on unwrap).
pub fn agg_mem_wit(state: U256, witness_x: U256, witness_y: U256, x: U256, y: U256) -> (U256, U256) {
    let aggregated = subroutines::shamir_trick(witness_x, witness_y, x, y).unwrap();
    let proof = proofs::poe(aggregated, subroutines::mul_mod(x, y, U256::from(super::MODULUS)), state);
    return (aggregated, proof);
}

/// Creates individual membership witnesses. Acts as a wrapper for the RootFactor subroutine.
/// NOTE: "old_state" represents the state *before* the elements are added.
pub fn create_all_mem_wit(old_state: U256, new_elems: &[U256]) -> Vec<U256> {
    return subroutines::root_factor(old_state, new_elems);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MODULUS;

    #[test]
    fn test_verify_mem_wit() {
        let proof = proofs::poe(U256::from(2), U256::from(12123), U256::from(8));
        assert_eq!(verify_mem_wit(U256::from(8), U256::from(12123), U256::from(2), proof), true);
        assert_eq!(verify_mem_wit(U256::from(7), U256::from(12123), U256::from(2), proof), false);
    }

    #[test]
    fn test_agg_mem_wit() {
        let (aggregate, proof) = agg_mem_wit(U256::from(8), U256::from(6), U256::from(8),U256::from(3), U256::from(5));
        assert_eq!(aggregate, U256::from(2));
        assert_eq!(verify_mem_wit(U256::from(8), U256::from(15), aggregate, proof), true);
    }

    #[test]
    fn test_update_mem_wit() {
        let deletions = U256::from(15);
        let additions = U256::from(77);

        let elem = U256::from(12131);
        let mut witness = U256::from(8);
        let new_state = U256::from(11);

        assert_eq!(update_mem_wit(elem, witness, new_state, additions, deletions), U256::from(6));
    }

    #[test]
    fn test_create_all_mem_wit() {
        assert_eq!(create_all_mem_wit(U256::from(2), &vec![U256::from(3), U256::from(5), U256::from(7), U256::from(11)]),
                   vec![U256::from(2), U256::from(8), U256::from(5), U256::from(5)]);
    }


}