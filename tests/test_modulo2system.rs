use std::collections::HashSet;

use rand::Rng;
use sux::solvers::modulo2system::Modulo2Equation;
use sux::solvers::modulo2system::Modulo2System;

#[test]
fn test_random_system() {
    let mut rng = rand::thread_rng();
    let n_eqs = 5000;
    let n_vars_per_eq = 4;
    let n_vars = 3 * n_eqs / 2;

    let mut system = Modulo2System::new(n_vars);
    let mut var2_eq = vec![Vec::new(); n_vars];
    let mut c = vec![0; n_eqs];
    let mut edge = HashSet::<usize>::new();

    for i in 0..n_eqs {
        edge.clear();
        c[i] = rng.gen_range(0..100);
        let mut eq = Modulo2Equation::new(c[i], n_vars);
        for _ in 0..n_vars_per_eq {
            let mut x = rng.gen_range(0..n_vars);
            while edge.contains(&x) {
                x = rng.gen_range(0..n_vars);
            }
            edge.insert(x);
            eq.add(x);
            var2_eq[x].push(i);
        }
        system.add(eq);
    }

    let sol = Modulo2System::lazy_gaussian_elimination(
        Some(&mut system),
        &mut var2_eq,
        &c,
        &(0..n_vars).collect(),
    );
    assert!(sol.is_ok());
    assert!(system.check(&sol.unwrap()));
}
