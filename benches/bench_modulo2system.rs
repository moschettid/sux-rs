use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{BufReader, BufRead};
use sux::solvers::modulo2system::Modulo2System;
use sux::solvers::modulo2system::Modulo2Equation;

pub fn criterion_benchmark(cr: &mut Criterion) {
    let mut lines = BufReader::new(File::open("benches/data/input_system.txt").unwrap()).lines();
    let size = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let n_equations = 2*size/3;
    
    let mut system = Modulo2System::new(size);
    let mut var2_eq = vec![Vec::new(); size];
    let mut c = vec![0; n_equations];
    let vars = (0..size).collect();

    for (i, vals) in lines.enumerate().map(|(i, line)| (i, line.unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>())).take(n_equations) {
        let cv = vals[0];
        let x = vals[1];
        let v = vals[2];
        let w = vals[3];

        c[i] = cv;
        let mut eq = Modulo2Equation::new(cv, size);
        eq.add(x).add(v).add(w);
        system.add(eq);
        
        var2_eq[x].push(i);
        var2_eq[v].push(i);
        var2_eq[w].push(i);
    }

    cr.bench_function("test_lazy_gaussian_wbuild", |b| b.iter(|| {
        let _ = Modulo2System::lazy_gaussian_elimination(None, black_box(&mut var2_eq), black_box(&c), black_box(&vars));
    }));

    cr.bench_function("test_lazy_gaussian_random", |b| b.iter_batched_ref(
        || system.clone(),
        |system| Modulo2System::lazy_gaussian_elimination(black_box(Some(system)), black_box(&mut var2_eq), black_box(&c), black_box(&vars)),
        criterion::BatchSize::SmallInput
    ));

    cr.bench_function("test_gaussian_random", |b| b.iter_batched_ref(
        || system.clone(),
        |system| system.gaussian_elimination(),
        criterion::BatchSize::SmallInput
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);