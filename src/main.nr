use dep::std::ec::tecurve::affine::{Curve as AffineCurve, Point};
use dep::std::collections::vec::Vec;
use dep::std;

global bjj_a = 168700;
global bjj_d = 168696;
global bjj_generator_x = 995203441582195749578291179787384436505546430278305826713579947235728471134;
global bjj_generator_y = 5472060717959818805561601436314318772137091100104008585924551046643952123905;
global bjj_basept_x = 5299619240641551281634865583518297030282874472190772894086521144482721001553;
global bjj_basept_y = 16950150798460657717958625567821834550301663161624707787222815936182638968203;

struct Ciphertext {
    C1: Point,
    C2: Point,
}

fn main(private_key: Field, plaintext: u32, randomness: Field) {
    std::print("\n");
    std::print("Original plaintext message: ");
    std::print(plaintext);
    std::print("\n");
    let public_key: Point = bjj_priv_to_pub_key(private_key);

    let ciphertext: Ciphertext = bjj_exp_elgamal_encrypt(public_key, plaintext, randomness);

    let decryption = bjj_exp_elgamal_decrypt(private_key, ciphertext);
    
    // Print the x and y coordinates of the decrypted point directly
    std::print("decrypted_x:");
    std::print(decryption.x); // This will be parsed by Rust
    std::print("\n");
    std::print("decrypted_y:");
    std::print(decryption.y); // This will be parsed by Rust
    std::print("\n");
}

#[test]
fn test_main() {
    let message: u32 = 943594123;
    let private_key: Field = 2291123624948246627368989940774052753470489062495018070576418670157516550852;
    let randomness: Field = 168986485046885582825082387270879151100288537211746581237924789162159767775;

    main(private_key, message, randomness);
}

fn bjj_priv_to_pub_key(private_key: Field) -> Point {
    let bjj_affine: AffineCurve = AffineCurve::new(bjj_a, bjj_d, Point::new(bjj_generator_x, bjj_generator_y));
    let base_pt: Point = Point::new(bjj_basept_x, bjj_basept_y);
    bjj_affine.mul(private_key, base_pt)
}

fn bjj_exp_elgamal_encrypt(public_key: Point, plaintext: u32, randomness: Field) -> Ciphertext {
    let bjj_affine: AffineCurve = AffineCurve::new(bjj_a, bjj_d, Point::new(bjj_generator_x, bjj_generator_y));
    let base_pt: Point = Point::new(bjj_basept_x, bjj_basept_y);

    let C1: Point = bjj_affine.mul(randomness, base_pt);
    let plain_embedded: Point = bjj_affine.mul(plaintext as Field, base_pt);
    let shared_secret: Point = bjj_affine.mul(randomness, public_key);
    let C2: Point = bjj_affine.add(shared_secret, plain_embedded);

    Ciphertext { C1, C2 }
}

fn bjj_exp_elgamal_decrypt(private_key: Field, ciphertext: Ciphertext) -> Point {
    let bjj_affine: AffineCurve = AffineCurve::new(bjj_a, bjj_d, Point::new(bjj_generator_x, bjj_generator_y));
    let shared_secret = bjj_affine.mul(private_key, ciphertext.C1);
    let plain_embedded = bjj_affine.subtract(ciphertext.C2, shared_secret);
    plain_embedded
}
