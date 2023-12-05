use std::collections::HashSet;

use sux::solvers::modulo2system::Modulo2System;
use sux::solvers::modulo2system::Modulo2Equation;
use rand::Rng;

#[test]
fn test_builder() {
    let mut eq = Modulo2Equation::new(2, 3);
    eq.add(2).add(0).add(1);
    assert_eq!(eq.variables().len(), 3);
    assert_eq!(eq.variables(), vec![0, 1, 2]);
}

#[test]
fn test_sub0() {
    let mut eq0 = Modulo2Equation::new(2, 11);
    eq0.add(1).add(4).add(9);
    let mut eq1 = Modulo2Equation::new(1, 11);
    eq1.add(1).add(4).add(10);
    eq0.add_equation(&eq1);
}

#[test]
fn test_one(){
    let mut system = Modulo2System::new(2);
    let mut eq = Modulo2Equation::new(2,2);
    eq.add(0);
    system.add(eq);
    let mut solution = vec![0; 2];
    let solvable = system.lazy_gaussian_elimination_constructor(&mut solution);
    assert!(solvable);
    assert!(system.check(&solution));
}

#[test]
fn test_impossible(){
    let mut system = Modulo2System::new(1);
    let mut eq = Modulo2Equation::new(2,1);
    eq.add(0);
    system.add(eq);
    eq = Modulo2Equation::new(1,1);
    eq.add(0);
    system.add(eq);
    let mut solution = vec![0];
    assert!(!system.lazy_gaussian_elimination_constructor(&mut solution));
    assert!(!system.check(&solution));
}

#[test]
fn test_redundant(){
    let mut system = Modulo2System::new(1);
    let mut eq = Modulo2Equation::new(2,1);
    eq.add(0);
    system.add(eq.clone());
    system.add(eq);
    let mut solution = vec![0];
    assert!(system.lazy_gaussian_elimination_constructor(&mut solution));
    assert!(system.check(&solution));
}

#[test]
fn test_small(){
    let mut system = Modulo2System::new(11);
    let mut eq = Modulo2Equation::new(0,11);
    eq.add(1).add(4).add(10);
    system.add(eq);
    eq = Modulo2Equation::new(2,11);
    eq.add(1).add(4).add(9);
    system.add(eq);
    eq = Modulo2Equation::new(0,11);
    eq.add(0).add(6).add(8);
    system.add(eq);
    eq = Modulo2Equation::new(1,11);
    eq.add(0).add(6).add(9);
    system.add(eq);
    eq = Modulo2Equation::new(2,11);
    eq.add(2).add(4).add(8);
    system.add(eq);
    eq = Modulo2Equation::new(0,11);
    eq.add(2).add(6).add(10);
    system.add(eq);

    let mut solution = vec![0;11];
    assert!(system.lazy_gaussian_elimination_constructor(&mut solution));
    assert!(system.check(&solution));
}

#[test]
fn test_random() {
    let mut rng = rand::thread_rng();
    let size = 1000;
    let mut system = Modulo2System::new(size);
    for _ in 0..2*size/3 {
        let mut eq = Modulo2Equation::new(rng.gen_range(0..100),size);
        eq.add(rng.gen_range(0..size/3)).add(size/3 + rng.gen_range(0..size/3)).add(2*size/3 + rng.gen_range(0..size/3));
        system.add(eq);
    }
    let mut solution = vec![0;size];
    assert!(system.lazy_gaussian_elimination_constructor(&mut solution));
    assert!(system.check(&solution));
}

#[test]
fn test_random_2() {
    let mut rng = rand::thread_rng();
    for size in vec![10, 100, 1000, 10000] {
        let mut system = Modulo2System::new(size);
        let mut edge = vec![HashSet::<usize>::new(); 2*size/3];
        let mut x;
        let mut v;
        let mut w;

        for i in 0..edge.len() {
            'gen_edge: loop {
                x = rng.gen_range(0..size);
                v = rng.gen_range(0..size); while v == x { v = rng.gen_range(0..size); }
                w = rng.gen_range(0..size); while w == x || w == v { w = rng.gen_range(0..size); }
                edge[i].insert(x);
                edge[i].insert(v);
                edge[i].insert(w);
                for j in 0..i {
                    if edge[j].contains(&x) && edge[j].contains(&v) && edge[j].contains(&w) {
                        continue 'gen_edge;
                    }
                }
                break;
            }
        }

        for e in edge.iter() {
            let mut eq = Modulo2Equation::new(rng.gen_range(0..100), size);
            e.iter().for_each(|&x| {eq.add(x);});
            system.add(eq);
        }

        let mut solution = vec![0;size];
        assert!(system.lazy_gaussian_elimination_constructor(&mut solution));
        assert!(system.check(&solution));
    }
}