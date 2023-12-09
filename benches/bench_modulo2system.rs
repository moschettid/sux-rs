use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sux::solvers::modulo2system::Modulo2System;
use sux::solvers::modulo2system::Modulo2Equation;
use std::collections::HashSet;
use rand::Rng;

pub fn criterion_benchmark(cr: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let size = 10000;
    let n_equations = 2*size/3;
    
    let mut system = Modulo2System::new(size);
    let mut system_clone = Modulo2System::new(size);
    let mut var2_eq = vec![Vec::new(); size];
    let mut c = vec![0; n_equations];
    let mut edge = vec![HashSet::<usize>::new(); n_equations];
    let mut x;
    let mut v;
    let mut w;

    for i in 0..edge.len() {
        'gen_edge: loop {
            x = rng.gen_range(0..size);
            v = rng.gen_range(0..size); while v == x { v = rng.gen_range(0..size); }
            w = rng.gen_range(0..size); while w == x || w == v { w = rng.gen_range(0..size); }
            for j in 0..i {
                if edge[j].contains(&x) && edge[j].contains(&v) && edge[j].contains(&w) {
                    continue 'gen_edge;
                }
            }
            var2_eq[x].push(i);
            var2_eq[v].push(i);
            var2_eq[w].push(i);
            edge[i].insert(x);
            edge[i].insert(v);
            edge[i].insert(w);
            break;
        }
    }

    for (i, e) in edge.iter().enumerate() {
        let c_val = rng.gen_range(0..100);
        c[i] = c_val;
        let mut eq = Modulo2Equation::new(c_val, size);
        e.iter().for_each(|&x| {eq.add(x);});
        system.add(eq.clone());
        system_clone.add(eq);
    }

    cr.bench_function("test_lazy_random", |b| b.iter(|| {
        let _ = Modulo2System::lazy_gaussian_elimination(None, black_box(&mut var2_eq), black_box(&c), black_box(&(0..size).collect()));
    }));

    cr.bench_function("test_random", |b| b.iter(|| {
        let _ = system_clone.gaussian_elimination();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);