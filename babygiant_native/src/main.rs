use ark_ed_on_bn254::{EdwardsAffine as BabyJubJub, Fr, Fq, FqParameters, EdwardsParameters};
use ark_ff::{BigInteger256, field_new, PrimeField, SquareRootField, FpParameters, BigInteger};
use ark_ec::{AffineCurve, ProjectiveCurve};
use std::process::Command;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use num_cpus;
use ark_ec::twisted_edwards_extended::{GroupAffine, GroupProjective};
use ark_ec::group::Group;
use std::env;
use num_bigint::BigUint;
use num_traits::Num;
use std::time::Instant;

fn baby_giant(max_bitwidth: u64, a: &GroupAffine<EdwardsParameters>, b: &GroupProjective<EdwardsParameters>) -> Option<u64> {
    println!("🔍 Starting baby-giant step algorithm...");
    let start_time = Instant::now();

    let m = 1u64 << (max_bitwidth / 2);
    let threads = num_cpus::get() as u64;
    let chunk_size = m / threads;
    let (tx, rx) = mpsc::channel();

    for idx in 0..threads {
        let a = a.clone();
        let b = b.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            println!("⚙️ Starting thread {} for chunk of size {}", idx + 1, chunk_size);

            let start = idx * chunk_size;
            let end = if idx == threads - 1 { m } else { start + chunk_size };
            let mut table = HashMap::new();

            let mut v = Group::mul(&a, &Fr::from(start));
            let a1 = Group::mul(&a, &Fr::from(1u64));

            for j in start..end {
                table.insert(v, j);
                v = v + a1;
            }

            let am = Group::mul(&a, &Fr::from(m)).into_projective();
            let mut gamma = b.clone();

            for i in 0..m {
                if let Some(j) = table.get(&gamma.into_affine()) {
                    tx.send(Some(i * m + j)).unwrap();
                    return;
                }
                gamma = gamma - am;
            }
            tx.send(None).unwrap();
        });
    }

    let mut result = None;
    for _ in 0..threads {
        if let Some(res) = rx.recv().unwrap() {
            result = Some(res);
            break;
        }
    }
    
    if result.is_some() {
        println!("✅ Baby-giant step algorithm completed successfully.");
    } else {
        println!("❌ Baby-giant step algorithm failed to find a match.");
    }

    let duration = start_time.elapsed();
    println!("⏱️ Baby-giant step algorithm took: {:?}", duration);

    result
}

fn run_noir() -> std::process::Output {
    println!("🚀 Running Noir test...");
    let start_time = Instant::now();

    let output = Command::new("nargo")
        .arg("test")
        .arg("--show-output")
        .output()
        .expect("Failed to execute Noir test");

    let output_str = String::from_utf8_lossy(&output.stdout);
    if output_str.is_empty() {
        eprintln!("❌ Error: No output from Noir.");
    } else {
        println!("📄 Noir Output:\n{}", output_str);
    }

    let duration = start_time.elapsed();
    println!("⏱️ Noir encryption took: {:?}", duration);

    output
}

fn parse_noir_output(output: &str) -> (String, String) {
    println!("🔍 Parsing Noir output...");
    let start_time = Instant::now();

    let decrypted_x = extract_value(output, "decrypted_x:");
    let decrypted_y = extract_value(output, "decrypted_y:");

    let duration = start_time.elapsed();
    println!("⏱️ Parsing Noir output took: {:?}", duration);

    (decrypted_x, decrypted_y)
}

fn extract_value(output: &str, key: &str) -> String {
    if let Some(start) = output.find(key) {
        let start = start + key.len();
        let end = output[start..].find('\n').unwrap_or(output.len());
        let extracted = output[start..start + end].trim().to_string();
        
        if extracted.is_empty() {  
            eprintln!("❌ Error: Extracted value for key '{}' is empty.", key);
            eprintln!("Output: {}", output);
            std::process::exit(1);
        }

        return extracted;
    } else {
        eprintln!("❌ Error: Key '{}' not found in output.", key);
        eprintln!("Output:\n{}", output);
        std::process::exit(1);
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "0");
    println!("🚀 Starting the process...");
    let overall_start_time = Instant::now();

    let output = run_noir();
    let output_str = std::str::from_utf8(&output.stdout).expect("Failed to parse output");

    if output_str.is_empty() {
        eprintln!("❌ Error: The output from Noir is empty.");
        std::process::exit(1);
    }

    let (decrypted_x, decrypted_y) = parse_noir_output(output_str);

    println!("🔓 Extracted decrypted_x: {}", decrypted_x);
    println!("🔓 Extracted decrypted_y: {}", decrypted_y);

    let dlog_result = do_compute_dlog(&decrypted_x, &decrypted_y);
    println!("🔑 Discrete logarithm result (decrypted message): {}", dlog_result);

    let overall_duration = overall_start_time.elapsed();
    println!("⏱️ Overall process took: {:?}", overall_duration);
}

fn do_compute_dlog(decrypted_x: &str, decrypted_y: &str) -> u64 {
    println!("🧮 Starting discrete logarithm computation...");
    let start_time = Instant::now();

    let coeff_twisted = field_new!(Fq, "168700").sqrt().unwrap();
    let gx = field_new!(Fq, "5299619240641551281634865583518297030282874472190772894086521144482721001553") * coeff_twisted;
    let gy = field_new!(Fq, "16950150798460657717958625567821834550301663161624707787222815936182638968203");
    let a = BabyJubJub::new(gx, gy);

    let stripped_x = &decrypted_x[2..];
    let stripped_y = &decrypted_y[2..];

    println!("🔢 Stripped decrypted_x: {}", stripped_x);
    println!("🔢 Stripped decrypted_y: {}", stripped_y);

    let bx = match convert_to_fq(stripped_x) {
        Some(val) => val * coeff_twisted,
        None => {
            println!("❌ Failed to convert stripped decrypted_x to Fq. Input: {}", stripped_x);
            return 0;
        }
    };

    let by = match convert_to_fq(stripped_y) {
        Some(val) => val,
        None => {
            println!("❌ Failed to convert stripped decrypted_y to Fq. Input: {}", stripped_y);
            return 0;
        }
    };

    println!("🔄 Converted Fq values - bx: {:?}, by: {:?}", bx, by);

    let b = BabyJubJub::new(bx, by);

    let result = baby_giant(40, &a, &b.into_projective()).unwrap_or_else(|| {
        panic!("❌ Discrete log computation failed for input x: {}, y: {}", decrypted_x, decrypted_y);
    });

    println!("🔑 Decrypted message as integer: {}", result);

    let duration = start_time.elapsed();
    println!("⏱️ Discrete logarithm computation took: {:?}", duration);

    result
}

fn convert_to_fq(s: &str) -> Option<Fq> {
    println!("🔄 Converting string to Fq...");
    let start_time = Instant::now();

    let s = s.strip_prefix("0x").unwrap_or(s);

    let padded_s = format!("{:0>64}", s);

    let bigint = match BigUint::from_str_radix(&padded_s, 16) {
        Ok(val) => val,
        Err(e) => {
            println!("❌ Hex decoding error: {}", e);
            return None;
        }
    };

    let fq_modulus = BigUint::from_bytes_le(&FqParameters::MODULUS.to_bytes_le());

    let reduced_bigint = bigint % fq_modulus;

    let mut buf = [0u64; 4];
    for (i, chunk) in reduced_bigint.to_bytes_le().chunks(8).enumerate() {
        buf[i] = u64::from_le_bytes(chunk.try_into().expect("Failed to convert chunk"));
    }

    let fq_value = Fq::from_repr(BigInteger256::new(buf));

    if fq_value.is_none() {
        println!("❌ Conversion to Fq failed for input: {:?}", s);
    }

    let duration = start_time.elapsed();
    println!("⏱️ String to Fq conversion took: {:?}", duration);

    fq_value
}
