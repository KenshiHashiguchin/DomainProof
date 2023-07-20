pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/poseidon.circom";

template Main() {
    component poseidon = Poseidon(2);
    signal input address;
    signal input secret;
    signal output digest;
    poseidon.inputs[0] <== address;
    poseidon.inputs[1] <== secret;
    digest <== poseidon.out;
}

component main {public [address]} = Main();