use ark_bn254::Bn254;
use ark_circom::CircomBuilder;
use ark_circom::CircomConfig;
use ark_groth16::Groth16;
use ark_snark::SNARK;
use rand::Rng;
use ark_serialize::CanonicalSerialize;
use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    // Load the WASM and R1CS for witness and proof generation
    let cfg = CircomConfig::<Bn254>::new("../main_js/main.wasm", "../main.r1cs").unwrap();

    // Insert our secret inputs as key value pairs. We insert two inputs, namely the input to the hash function.
    let mut builder = CircomBuilder::new(cfg);
    let address = "67af222e12100ed4d87d35197c2f61a7a4a31a4beb44e8ff0fd2d50829ed77d3";
    let address_decimal_string = BigInt::parse_bytes(address.as_bytes(), 16).unwrap().to_string();
    let address_parsed_number = BigInt::from_str(&address_decimal_string).expect("Error");
    println!("{}", address_decimal_string);

    // let secret = "secret";
    let secret = "2e12100ed4d87d35197c";
    let secret_decimal_string = BigInt::parse_bytes(secret.as_bytes(), 16).unwrap().to_string();
    let secret_parsed_number = BigInt::from_str(&secret_decimal_string).expect("Error");
    println!("{}", secret_decimal_string);

    builder.push_input("address", address_parsed_number);
    builder.push_input("secret", secret_parsed_number);

    // Create an empty instance for setting it up
    let circom = builder.setup();

    // WARNING: The code below is just for debugging, and should instead use a verification key generated from a trusted setup.
    // See for example https://docs.circom.io/getting-started/proving-circuits/#powers-of-tau.
    let mut rng = rand::thread_rng(); // TODO
    let params =
        Groth16::<Bn254>::generate_random_parameters_with_reduction(circom, &mut rng).unwrap();

    let circom = builder.build().unwrap();

    // There's only one public input, namely the hash digest.
    let inputs = circom.get_public_inputs().unwrap();

    // Generate the proof
    let proof = Groth16::<Bn254>::prove(&params, circom, &mut rng).unwrap();
    let mut vk_inputs_bytes = Vec::new();
    params.vk.serialize_compressed(&mut vk_inputs_bytes).unwrap();
    // params.vk.alpha_g1.serialize_compressed(&mut vk_inputs_bytes).unwrap();
    // params.vk.beta_g2.serialize_compressed(&mut vk_inputs_bytes).unwrap();
    // params.vk.gamma_g2.serialize_compressed(&mut vk_inputs_bytes).unwrap();
    // params.vk.delta_g2.serialize_compressed(&mut vk_inputs_bytes).unwrap();
    // params.vk.gamma_abc_g1.serialize_compressed(&mut vk_inputs_bytes).unwrap(); // ok
    println!("{}", "params.vk");
    // println!("{:?}", params.vk);
    println!("{:?}", vk_inputs_bytes);
    println!("{}", vk_inputs_bytes.iter().map(|n| format!("{:02X}", n)).collect::<String>());


    // Check that the proof is valid
    let pvk = Groth16::<Bn254>::process_vk(&params.vk).unwrap();

    println!("{}", "#############################");

    // println!("{}", "pvk");
    // let mut pvk_inputs_bytes = Vec::new();
    // pvk.serialize_compressed(&mut pvk_inputs_bytes).unwrap();
    // let mut pvk_inputs_bytes_1 = Vec::new();
    // let mut pvk_inputs_bytes_2 = Vec::new();
    // let mut pvk_inputs_bytes_3 = Vec::new();
    // let mut pvk_inputs_bytes_4 = Vec::new();
    // pvk.vk.gamma_abc_g1.serialize_compressed(&mut pvk_inputs_bytes_1).unwrap();
    // pvk.alpha_g1_beta_g2.serialize_compressed(&mut pvk_inputs_bytes_2).unwrap();
    // pvk.gamma_g2_neg_pc.serialize_compressed(&mut pvk_inputs_bytes_3).unwrap();
    // pvk.delta_g2_neg_pc.serialize_compressed(&mut pvk_inputs_bytes_4).unwrap();
    // println!("{:?}", pvk_inputs_bytes_1);
    // println!("{:?}", pvk_inputs_bytes_2);
    // println!("{:?}", pvk_inputs_bytes_3);
    // println!("{:?}", pvk_inputs_bytes_4);

    // println!("{:?}", pvk_inputs_bytes);

    let verified = Groth16::<Bn254>::verify_with_processed_vk(&pvk, &inputs, &proof).unwrap();
    assert!(verified);
    println!("{}", "///////////////////////////////");

    // let mut vk_split_inputs_bytes = Vec::new();
    // println!("{}", "vk_split_inputs_bytes");

    // params.vk.alpha_g1_beta_g2_bytes.serialize_compressed(&mut vk_inputs_bytes).unwrap();
    // params.vk.gamma_g2_neg_pc_bytes.serialize_compressed(&mut vk_inputs_bytes).unwrap();
    // params.vk.delta_g2_neg_pc_bytes.serialize_compressed(&mut vk_inputs_bytes).unwrap();


    let mut proof_inputs_bytes = Vec::new();
    inputs.serialize_compressed(&mut proof_inputs_bytes).unwrap();
    println!("inputs");
    println!("{:?}", inputs);
    println!("{:?}", proof_inputs_bytes);
    println!("{}", proof_inputs_bytes.iter().map(|n| format!("{:02X}", n)).collect::<String>());


    let mut proof_points_bytes = Vec::new();
    proof.a.serialize_compressed(&mut proof_points_bytes).unwrap();
    proof.b.serialize_compressed(&mut proof_points_bytes).unwrap();
    proof.c.serialize_compressed(&mut proof_points_bytes).unwrap();
    println!("proof_points_bytes");
    println!("{:?}", proof_points_bytes);
    println!("{}", proof_points_bytes.iter().map(|n| format!("{:02X}", n)).collect::<String>());

}