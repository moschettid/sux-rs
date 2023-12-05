use crate::bits::bit_vec::BitVec;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Modulo2Equation {
    bit_vector: BitVec,
    c: u64,
    first_var: usize,
    is_empty: bool,
}

#[derive(Debug)]
pub struct Modulo2System {
    num_vars: u32,
    equations: Vec<Rc<RefCell<Modulo2Equation>>>,
}

impl Modulo2Equation {
    pub fn new(c: u64, num_vars: u32) -> Self {
        Modulo2Equation {
            bit_vector: BitVec::new(num_vars as usize),
            c: c,
            first_var: usize::MAX,
            is_empty: true,
        }
    }

    fn from_equation(equation: &Modulo2Equation) -> Self {
        let bv = equation.bit_vector.clone();
        Modulo2Equation {
            bit_vector: bv,
            c: equation.c,
            first_var: equation.first_var,
            is_empty: equation.is_empty,
        }
    }

    pub fn add(&mut self, variable: usize) -> &mut Self{
        assert!(!self.bit_vector.get(variable));
        self.bit_vector.set(variable, true);
        self.is_empty = false;
        self
    }
    

    pub fn variables(&self) -> Vec<usize> {
        (0..self.bit_vector.len()).filter(|&x| self.bit_vector.get(x)).collect::<Vec<_>>()
    }

    pub fn add_equation(&mut self, equation: &Modulo2Equation) {
        self.c ^= equation.c;
        let x = self.bit_vector.as_mut();
        let y = equation.bit_vector.as_ref();
        let mut is_not_empty: usize = 0;
        for i in 0..x.len(){
            x[i] ^= y[i];
            is_not_empty |= x[i];
        }
        self.is_empty = is_not_empty == 0;
    }

    fn update_first_var(&mut self) {
        if self.is_empty {self.first_var = usize::MAX;}
        else {
            let mut i = 0;
            let bits = self.bit_vector.as_ref();
            while bits[i]==0 {i+=1};
            self.first_var = i*usize::BITS as usize + bits[i].trailing_zeros() as usize;
        }
    }

    fn is_unsolvable(&self) -> bool {
        self.is_empty && self.c!=0
    }

    fn is_identity(&self) -> bool {
        self.is_empty && self.c==0
    }

    fn scalar_product(bits: &[usize], values: &Vec<u64>) -> u64 {
        let mut sum: u64 = 0;
        for i in 0..bits.len() {
            let offset = i * usize::BITS as usize;
            let mut word = bits[i];
            while word != 0 {
                let lsb = word.trailing_zeros();
                sum ^= values[offset + lsb as usize];
                word &= word - 1;
            }
        }
        sum
    }
}

impl Modulo2System {
    pub fn new (num_vars: u32) -> Self {
        Modulo2System {
            num_vars: num_vars,
            equations: Vec::new(),
        }
    }

    fn from_copied (num_vars: u32, equations: &Vec<Rc<RefCell<Modulo2Equation>>>) -> Self {
        Modulo2System {
            num_vars: num_vars,
            equations: equations.iter().map(|eq| {Rc::new(RefCell::new(eq.borrow().clone()))}).collect(),
        }
    }

    fn from (num_vars: u32, equations: Vec<Rc<RefCell<Modulo2Equation>>>) -> Self {
        Modulo2System {
            num_vars: num_vars,
            equations: equations,
        }
    }

    pub fn add(&mut self, equation: Modulo2Equation) {
        assert!(equation.bit_vector.len() == self.num_vars as usize);
        self.equations.push(Rc::new(RefCell::new(equation)));
    }

    pub fn check(&self, solution: &Vec<u64>) -> bool {
        assert!(solution.len() == self.num_vars as usize);
        self.equations.iter().map(|eq| eq.borrow()).all(|eq|
            eq.c == Modulo2Equation::scalar_product(&eq.bit_vector.as_ref(), &solution)
        )
    }

    fn echelon_form(&mut self) -> bool {
        if self.equations.len() == 0 {return true};
        'main: for i in 0..self.equations.len()-1 {
            assert!(self.equations[i].borrow().first_var != usize::MAX);
            for j in i+1..self.equations.len() {
                let fvi: usize;
                let fvj: usize;

                {
                    let eq_j = self.equations[j].borrow();
                    let mut eq_i = self.equations[i].borrow_mut();
                    assert!(eq_i.first_var != usize::MAX);
                    assert!(eq_j.first_var != usize::MAX);

                    if eq_i.first_var == eq_j.first_var {
                        eq_i.add_equation(&eq_j);
                        if eq_i.is_unsolvable() {return false};
                        if eq_i.is_identity() {continue 'main};
                        eq_i.update_first_var();
                    }

                    fvi = eq_i.first_var;
                    fvj = eq_j.first_var;
                }
                

                if fvi > fvj {self.equations.swap(i, j)};
            }
        }
        true
    }

    pub fn gaussian_elimination(&mut self, solution: &mut Vec<u64>) -> bool {
        assert!(solution.len() == self.num_vars as usize);
        self.equations.iter().for_each(|x| x.borrow_mut().update_first_var());

        if !self.echelon_form() {return false};

        self.equations.iter().rev().map(|eq| eq.borrow()).filter(|eq| !eq.is_identity()).for_each(|eq| {
            assert!(solution[eq.first_var as usize] == 0);
            solution[eq.first_var as usize] = eq.c ^ Modulo2Equation::scalar_product(&eq.bit_vector.as_ref(), &solution);
        });
        true
    }

    //Only for testing purposes
    pub fn lazy_gaussian_elimination_constructor(&mut self, solution: &mut Vec<u64>) -> bool {
        let num_vars = self.num_vars as usize;
        let mut var2_eq = vec![Vec::new(); num_vars];
        let mut d = vec![0; num_vars];
        self.equations.iter().map(|e| e.borrow()).for_each(|eq|
            (0..eq.bit_vector.len()).filter(|&x| eq.bit_vector.get(x)).for_each(|x| d[x] += 1)
        );
        
        var2_eq.iter_mut().enumerate().for_each(|(i, v)| v.reserve_exact(d[i]));

        let mut c = vec![0; self.equations.len()];
        self.equations.iter().enumerate().for_each(|(i, e)| {
            let eq = e.borrow();
            c[i] = eq.c;
            (0..eq.bit_vector.len()).filter(|&x| eq.bit_vector.get(x)).for_each(|x|
                var2_eq[x].push(i as u32)
            );
        });
        Modulo2System::lazy_gaussian_elimination(Some(self), &mut var2_eq, &c, &(0..num_vars as u32).collect(), solution)
    }

    fn lazy_gaussian_elimination(system_op: Option<&mut Modulo2System>, var2_eq: &mut Vec<Vec<u32>>, c: &Vec<u64>, variable: &Vec<u32>, solution: &mut Vec<u64>) -> bool {
        let num_equations = c.len();
        if num_equations == 0 {return true};

        let num_vars = var2_eq.len();
        assert!(solution.len() == num_vars);

        let mut new_system = Modulo2System::new(num_vars as u32);
        let build_system = system_op.is_none();
        let system;
        if build_system {
            system = &mut new_system;
            c.iter().for_each(|&x| system.add(Modulo2Equation::new(x, num_vars as u32)));
        } else {system = system_op.unwrap()};
        
        let mut weight: Vec<u32> = vec![0; num_vars];
        let mut priority: Vec<u32> = vec![0; num_equations];

        for &v in variable.iter(){
            let eq = &mut var2_eq[v as usize];
            if eq.len() == 0 {continue};

            let mut curr_eq = eq[0];
            let mut curr_coeff = true;
            let mut j = 0;

            for i in 1..eq.len() {
                if eq[i] != curr_eq {
                    assert!(eq[i] > curr_eq);
                    if curr_coeff {
                        if build_system { system.equations[curr_eq as usize].borrow_mut().add(v as usize); }
                        weight[v as usize] += 1;
                        priority[curr_eq as usize] += 1;
                        eq[j] = curr_eq;
                        j += 1;
                    }
                    curr_eq = eq[i];
                    curr_coeff = true;
                } else {curr_coeff = !curr_coeff};
            }

            if curr_coeff {
                if build_system { system.equations[curr_eq as usize].borrow_mut().add(v as usize); }
                weight[v as usize] += 1;
                priority[curr_eq as usize] += 1;
                eq[j] = curr_eq;
                j+=1;
            }
            eq.truncate(j);
        }

        let mut variables = vec![0; num_vars];
        {
            let mut count = vec![0; num_equations+1];

            for x in 0..num_vars {count[weight[x] as usize] += 1};
            for i in 1..num_equations {count[i] += count[i-1]};
            for i in (0..num_vars).rev() {
                count[weight[i] as usize] -= 1;
                variables[count[weight[i] as usize]] = i as u32;
            }
        }

        let mut equation_list: Vec<u32> = (0..priority.len() as u32)
        .filter(|&x| priority[x as usize] <= 1)
        .collect();

        let mut dense: Vec<Rc<RefCell<Modulo2Equation>>> = Vec::new();
        let mut solved: Vec<Rc<RefCell<Modulo2Equation>>> = Vec::new();
        let mut pivots: Vec<u32> = Vec::new();

        let equations = &system.equations;
        let mut idle_normalized = vec![usize::MAX; equations[0].borrow().bit_vector.as_ref().len()];

        let mut remaining = equations.len();
        while remaining != 0 {
            if equation_list.is_empty() {
                let mut var = variables.pop().unwrap();
                while weight[var as usize] == 0 {var = variables.pop().unwrap()};
                idle_normalized[var as usize / usize::BITS as usize] ^= 1 << (var % usize::BITS);
                var2_eq[var as usize].iter().for_each(|&eq|{
                    priority[eq as usize] -= 1;
                    if priority[eq as usize] == 1 {equation_list.push(eq)}
                });
            }
            else {
                remaining -= 1;
                let first = equation_list.pop().unwrap();
                let ref_equation = &equations[first as usize];
                let equation = ref_equation.borrow();

                if priority[first as usize] == 0 {
                    if equation.is_unsolvable() {return false};
                    if equation.is_identity() {continue};
                    dense.push(Rc::clone(&ref_equation));
                } else if priority[first as usize] == 1 {
                    let mut word_index = 0;
                    while (equation.bit_vector.as_ref()[word_index] & idle_normalized[word_index]) == 0 {word_index += 1}
                    let pivot = word_index * usize::BITS as usize + (equation.bit_vector.as_ref()[word_index] & idle_normalized[word_index]).trailing_zeros() as usize;
                    pivots.push(pivot as u32);
                    solved.push(Rc::clone(&ref_equation));
                    weight[pivot] = 0;
                    var2_eq[pivot].iter()
                    .filter(|&&eq_idx| eq_idx != first)
                    .for_each(|&eq|{
                        priority[eq as usize] -= 1;
                        if priority[eq as usize] == 1 {equation_list.push(eq)}
                        equations[eq as usize].borrow_mut().add_equation(&equation);
                    });
                }
            }
        }

        let mut dense_system = Modulo2System::from(num_vars as u32, dense);
        if !dense_system.gaussian_elimination(solution) {return false};

        for i in 0..solved.len() {
            let eq = solved[i].borrow();
            let pivot = pivots[i];
            assert!(solution[pivot as usize] == 0);
            solution[pivot as usize] = eq.c ^ Modulo2Equation::scalar_product(&eq.bit_vector.as_ref(), &solution);
        }

        true
    }
}