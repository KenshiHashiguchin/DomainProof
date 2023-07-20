module domain_proof::generate_verified_domain {
    use sui::tx_context::{Self, TxContext};
    use sui::object::{Self, UID};
    use sui::groth16;
    use sui::event;
    use sui::transfer;

    /// Owned object
    struct VerifiedDomain<phantom T> has key, store {
        id: UID,
        domain: vector<u8>, // example.com
        expire: u64 // unix timestamp
    }

    /// Event on whether the proof is verified
    struct VerifiedEvent has copy, drop {
        is_verified: bool,
    }

    public fun verify_proof<T: drop>(_w: T, vk: vector<u8>, public_inputs_bytes: vector<u8>, proof_points_bytes: vector<u8>, domain: vector<u8>, expire: u64, ctx: &mut TxContext) {
        // The first element of public_inputs_bytes is a hash TODO
        // The second element of public_inputs_bytes is an address TODO
        let pvk = groth16::prepare_verifying_key(&groth16::bn254(), &vk);
        let public_inputs = groth16::public_proof_inputs_from_bytes(public_inputs_bytes);
        let proof_points = groth16::proof_points_from_bytes(proof_points_bytes);
        let is_verified = groth16::verify_groth16_proof(&groth16::bn254(), &pvk, &public_inputs, &proof_points);
        event::emit(VerifiedEvent {is_verified});
        assert!(is_verified, 0);

        // create Certificate
        let v:VerifiedDomain<T> = VerifiedDomain {
            id: object::new(ctx),
            domain,
            expire
        };
        transfer::public_transfer(v, tx_context::sender(ctx));
    }

    public fun domain<T: drop>(self: &VerifiedDomain<T>): vector<u8> {
        self.domain
    }

    entry public fun destroy<T>(s: VerifiedDomain<T>) {
        let VerifiedDomain { id, domain: _, expire: _ } = s;
        object::delete(id);
    }
}




