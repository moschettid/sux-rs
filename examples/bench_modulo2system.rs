use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use std::collections::HashSet;
use sux::solvers::modulo2system::Modulo2Equation;
use sux::solvers::modulo2system::Modulo2System;

static DELTAS: [f64; 5] = [0.0, 0.0, 0.0, 1.09, 1.025];

fn splitmix64_next(seed: &mut usize) -> usize {
    *seed = seed.wrapping_add(0x9e3779b97f4a7c15);

    let mut z = *seed;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn init_rng_from_u64(seed: usize) -> SmallRng {
    let mut k = seed;
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51afd7ed558ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ceb9fe1a85ec53);
    k ^= k >> 33;

    let s0 = splitmix64_next(&mut k);
    let s1 = splitmix64_next(&mut k);
    let s2 = splitmix64_next(&mut k);
    let s3 = splitmix64_next(&mut k);

    let mut state = [0u8; 32];
    state[0..8].copy_from_slice(&s0.to_le_bytes());
    state[8..16].copy_from_slice(&s1.to_le_bytes());
    state[16..24].copy_from_slice(&s2.to_le_bytes());
    state[24..32].copy_from_slice(&s3.to_le_bytes());

    SmallRng::from_seed(state)
}

fn gen_bounded(rng: &mut SmallRng, n: usize) -> usize {
    let n_u64 = n as u64;
    let mut t = rng.next_u64();
    let n_minus_1 = n_u64 - 1;

    let mut u = t >> 1;
    t = u % n_u64;
    while u + n_minus_1 < t {
        u = rng.next_u64() >> 1;
        t = u % n_u64;
    }

    t as usize
}

fn gen_system(
    rng: &mut SmallRng,
    n_eqs: usize,
    n_vars_per_eq: usize,
) -> (Modulo2System, Vec<Vec<usize>>, Vec<usize>) {
    let n_vars = (n_eqs as f64 * DELTAS[n_vars_per_eq]).ceil() as usize;
    let mut system = Modulo2System::new(n_vars);
    let mut var2_eq = vec![Vec::new(); n_vars];
    let mut c = vec![0; n_eqs];
    let mut edge = HashSet::<usize>::new();

    for i in 0..n_eqs {
        edge.clear();
        c[i] = gen_bounded(rng, 100);
        let mut eq = Modulo2Equation::new(c[i], n_vars);
        for _ in 0..n_vars_per_eq {
            let mut x = gen_bounded(rng, n_vars);
            while edge.contains(&x) {
                x = gen_bounded(rng, n_vars);
            }
            edge.insert(x);
            eq.add(x);
            var2_eq[x].push(i);
        }
        system.add(eq);
    }
    (system, var2_eq, c)
}

pub fn main() {
    let seed = std::env::args().nth(1).unwrap().parse::<usize>().unwrap();
    for n_eqs in [1000, 10000, 100000] {
        for n_vars_per_eq in [3, 4] {
            for _ in 0..10 {
                let mut rng = init_rng_from_u64(seed);
                loop {
                    let mut system;
                    let mut var2_eq;
                    let n_vars = (n_eqs as f64 * DELTAS[n_vars_per_eq]).ceil() as usize;
                    let c;
                    (system, var2_eq, c) = gen_system(&mut rng, n_eqs, n_vars_per_eq);

                    let vars = (0..n_vars).collect();

                    if let Ok(result) = Modulo2System::lazy_gaussian_elimination(
                        Some(&mut system),
                        &mut var2_eq,
                        &c,
                        &vars,
                    ) {
                        if !system.check(&result) {
                            println!("Error: solution is not valid");
                        }
                        break;
                    }
                }
            }
        }
        //println!();
    }
}
