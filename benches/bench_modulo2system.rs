use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use std::collections::HashSet;
use sux::solvers::modulo2system::Modulo2System;
use sux::solvers::modulo2system::Modulo2Equation;

pub fn bench_lazy_gaussian_wbuild(cr: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(0);
    let size = 10000;
    let n_equations = 2*size/3;
    
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
            c[i] = rng.gen_range(0..100);
            break;
        }
    }

    let vars = (0..size).collect();

    cr.bench_function("test_lazy_gaussian_wbuild", |b| b.iter(|| {
        let _ = Modulo2System::lazy_gaussian_elimination(None, black_box(&mut var2_eq), black_box(&c), black_box(&vars));
    }));
}

pub fn bench_lazy_gaussian(cr: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(0);
    let size = 10000;
    let n_equations = 2*size/3;
    
    let mut system = Modulo2System::new(size);
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
    }

    let vars = (0..size).collect();

    cr.bench_function("test_lazy_gaussian_random", |b| b.iter_batched_ref(
        || system.clone(),
        |system| Modulo2System::lazy_gaussian_elimination(black_box(Some(system)), black_box(&mut var2_eq), black_box(&c), black_box(&vars)),
        criterion::BatchSize::SmallInput
    ));
}

pub fn bench_gaussian_random(cr: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(0);
    let size = 10000;
    let n_equations = 2*size/3;
    
    let mut system = Modulo2System::new(size);
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
            edge[i].insert(x);
            edge[i].insert(v);
            edge[i].insert(w);
            break;
        }
    }

    for e in edge.iter() {
        let mut eq = Modulo2Equation::new(rng.gen_range(0..100), size);
        e.iter().for_each(|&x| {eq.add(x);});
        system.add(eq.clone());
    }

    cr.bench_function("test_gaussian_random", |b| b.iter_batched_ref(
        || system.clone(),
        |system| system.gaussian_elimination(),
        criterion::BatchSize::SmallInput
    ));
}

criterion_group!(benches, bench_lazy_gaussian_wbuild, bench_lazy_gaussian, bench_gaussian_random);
criterion_main!(benches);