/* TODO
Chiarire convenzione variabili del sistema: usize o u32?
Rendere idiomatico + uso di pattern (result ecc...)
Documentazione
Chiarire istruzione add */

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
    num_vars: u32, //per ora u32 per il limite di valore. Anche se solitamente per le dimensioni si usa usize
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

    /*TODO: gestire visibilità (inizialmete era protected, usata nel metodo copy).
    Va pensato il funzionamento e la necessità di scrivere qusto metodo assieme al ragionamento sulla necessità di clone.
    Il codice è in questo metodo e clone si limita a chiamarlo. Si potrebbe pensare di spostare il codice di questo metodo in clone. (addirittura eliminarlo se come allo stato attuale viene derivato).
    Questo è consentito solo se viene accettato il fatto che BitVec implementi clone.
    Inoltre si può pensare di rinominare a "from"
    */
    fn from_equation(equation: &Modulo2Equation) -> Self {
        let bv = equation.bit_vector.clone();
        Modulo2Equation {
            bit_vector: bv,
            c: equation.c,
            first_var: equation.first_var,
            is_empty: equation.is_empty,
        }
    }

    //TODO: In Java è strutturato così, cioé restituisce un'istanza (che
    //in realtà è la stessa). In questo caso cosa vogliamo fare? Lo mettiamo in
    //stile procedura o lo lasciamo come java (stato attuale)? 
    //C'è da dire che in Java funziona anche in stile procedura, perché sta
    //generando un ulteriore riferimento e se vuole lo può scartare. Nel nostro
    //caso non possiamo fare la stessa cosa, perchè non possiamo fare la giocata
    //del doppio riferimento (di fatto consumiamo il primo). Quale delle due
    //versioni è più adatta?
    /*pub fn add(mut self, variable: usize) -> Self {
        assert!(!self.bit_vector.get(variable));
        self.bit_vector.set(variable, true);
        self.is_empty = false;
        self
    }*/

    //Conserva la chain (ma staccata dalla definizione)
    pub fn add(&mut self, variable: usize) -> &mut Self{
        assert!(!self.bit_vector.get(variable));
        self.bit_vector.set(variable, true);
        self.is_empty = false;
        self
    }
    

    pub fn variables(&self) -> Vec<usize> {
        //TODO: può avere senso? O è meglio quello esplicito sotto? Domanda a
        //cui ci risponderà il benchmarking oppure posso pensare di iterare con
        //enumerate sul bit vector, ma non so se la classe BitVec lo metta a
        //disposizione (non penso) e in quel caso se vada bene che implementi
        (0..self.bit_vector.len()).filter(|&x| self.bit_vector.get(x)).collect::<Vec<_>>()
        /*let mut variables = Vec::new();
        for i in 0..self.bit_vector.len() {
            if self.bit_vector.get(i) {
                variables.push(i);
            }
        }
        variables*/

        //Seriamente controlliamo una ad una tutte le variabili? Non è meglio controllare sulle word del bit vector
        //e approfonidre la questione solo se troviamo una word con almeno un bit a 1?
    }

    pub fn add_equation(&mut self, equation: &Modulo2Equation) {
        self.c ^= equation.c;
        let x = self.bit_vector.mut_bits();
        let y = equation.bit_vector.bits();
        let mut is_not_empty: usize = 0;
        //TODO: non sicuro del fatto che sia idiomatico
        for i in 0..x.len(){
            x[i] ^= y[i];
            is_not_empty |= x[i];
        }
        self.is_empty = is_not_empty == 0;
    }

    fn update_first_var(&mut self) {
        if self.is_empty {self.first_var = usize::MAX;}
        else {
            //TODO non sicuro del fatto che sia idiomatico
            let mut i = 0;
            let bits = self.bit_vector.bits();
            while bits[i]==0 {i+=1};
            //Mi sembra possa essere scritto meglio
            self.first_var = i*usize::BITS as usize + bits[i].trailing_zeros() as usize;
        }
    }

    fn is_unsolvable(&self) -> bool {
        self.is_empty && self.c!=0
    }

    fn is_identity(&self) -> bool {
        self.is_empty && self.c==0
    }

    fn scalar_product(bits: &Vec<usize>, values: &Vec<u64>) -> u64 {
        let mut sum: u64 = 0;
        //TODO: non sicuro del fatto che sia idiomatico
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

/*
Necessario solo qualora si scelga di far si che BitVec non implementi Clone (anche solo derivando)
impl Clone for Modulo2Equation {
    fn clone(&self) -> Self {
        Modulo2Equation::from_equation(&self)
    }
}*/

impl Modulo2System {
    pub fn new (num_vars: u32) -> Self {
        Modulo2System {
            num_vars: num_vars,
            equations: Vec::new(),
        }
    }

    //Non è la classica firma di un from
    //Due versioni: una che consuma (più veloce) e una effettua deep copy delle equazioni
    //un po' diverso da sux4j, perchè sono obbligato a consumare (a meno di ulteriori incapsulamenti)
    //Questa versione ottima da usare per implementare il clone
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

    //TODO: mi sembra poco idiomatico. Restituire un Result mi pare più corretto
    //Allo stato attuale consuma l'equazione. Può essere un problema? (non credo)
    pub fn add(&mut self, equation: Modulo2Equation) {
        assert!(equation.bit_vector.len() == self.num_vars as usize); //Dovrebbe essere più informativo in caso si verifichi l'errore(?)
        self.equations.push(Rc::new(RefCell::new(equation)));
    }

    //TODO: mi sembra poco idiomatico. Restituire un Result mi pare più corretto
    pub fn check(&self, solution: &Vec<u64>) -> bool {
        assert!(solution.len() == self.num_vars as usize); //Dovrebbe essere più informativo in caso si verifichi l'errore(?)
        self.equations.iter().map(|eq| eq.borrow()).all(|eq|
            eq.c == Modulo2Equation::scalar_product(&eq.bit_vector.bits(), &solution)
        )
    }

    //TODO: mi sembra poco idiomatico. Restituire un Result mi pare più corretto
    //Gestione ripugnante. Salvate le variabili e creato il blocco perché il
    //borrow checker non in grado di accorgersi dell'uso corretto
    fn echelon_form(&mut self) -> bool {
        //TODO: if davvero brutto ma altrimenti salta alla sottrazione
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

    //TODO: mi sembra poco idiomatico. Restituire un Result mi pare più corretto.
    pub fn gaussian_elimination(&mut self, solution: &mut Vec<u64>) -> bool {
        assert!(solution.len() == self.num_vars as usize); //Dovrebbe essere più informativo in caso si verifichi l'errore(?
        self.equations.iter().for_each(|x| x.borrow_mut().update_first_var());

        if !self.echelon_form() {return false};

        //Versione fedele a sux4j
        /*for i in (0..self.equations.len()).rev() {
            let eq_i = self.equations[i].borrow();
            if eq_i.is_identity() {continue};
            assert!(solution[eq_i.first_var as usize] == 0);
            solution[eq_i.first_var as usize] = eq_i.c ^ Modulo2Equation::scalar_product(&eq_i.bit_vector.bits(), &solution);
        }*/

        //Versione che mi sembra più idiomatica
        self.equations.iter().rev().map(|eq| eq.borrow()).filter(|eq| !eq.is_identity()).for_each(|eq| {
            assert!(solution[eq.first_var as usize] == 0);
            solution[eq.first_var as usize] = eq.c ^ Modulo2Equation::scalar_product(&eq.bit_vector.bits(), &solution);
        });
        true
    }

    //Costruzione solo per unit test
    pub fn lazy_gaussian_elimination_constructor(&mut self, solution: &mut Vec<u64>) -> bool {
        //Creato per problemi di borrowing alla chiamata finale
        let num_vars = self.num_vars as usize;
        //Sto temporaneamente specificando u32, ma può darsi che dopo possa ometterlo
        let mut var2_eq = vec![Vec::<u32>::new(); num_vars];
        let mut d = vec![0; num_vars];
        self.equations.iter().map(|e| e.borrow()).for_each(|eq|
            (0..eq.bit_vector.len()).filter(|&x| eq.bit_vector.get(x)).for_each(|x| d[x] += 1)
        );
        //TODO In questo approccio sto già creando i vector vuoti, e qui li
        //ridimensiono. Potrei provare a creare var2_eq vuoto ma con capacità
        //giusta e poi pushare i vec creati con la capacità giusta. Su due piedi
        //mi sembra un approccio ancora più efficiente
        var2_eq.iter_mut().enumerate().for_each(|(i, v)| v.reserve_exact(d[i]));

        let mut c = vec![0; self.equations.len()];
        self.equations.iter().enumerate().for_each(|(i, e)| {
            //TODO: qua sto facendo borrowing esplicito anziché con map, perché
            //non so bene come gestire l'enumerate, potrebbe esserci margine di
            //miglioramento
            let eq = e.borrow();
            c[i] = eq.c;
            (0..eq.bit_vector.len()).filter(|&x| eq.bit_vector.get(x)).for_each(|x|
                //Qua sto sfruttando il push (la capacità è corretta quindi non
                //fa riallocazioni). Ci sarebbe da capire se inizializzando e
                //dopodiché sovrascrivendo (stile sux4j), sia meglio (ma dubito
                //fortemente).
                var2_eq[x].push(i as u32)
            );
        });
        Modulo2System::lazy_gaussian_elimination(Some(self), &mut var2_eq, &c, &(0..num_vars as u32).collect(), solution)
    }

    //VIGNA: ma restituire il vec di soluzione (in un result) anzichè il riferimento?
    fn lazy_gaussian_elimination(system_op: Option<&mut Modulo2System>, var2_eq: &mut Vec<Vec<u32>>, c: &Vec<u64>, variable: &Vec<u32>, solution: &mut Vec<u64>) -> bool {
        let num_equations = c.len();
        if num_equations == 0 {return true};

        let num_vars = var2_eq.len();
        assert!(solution.len() == num_vars); //Dovrebbe essere più informativo in caso si verifichi l'errore(?)

        //TODO, VIGNA: non riesco proprio a rendere opzionale la creazione del sistema sostituto
        let mut new_system = Modulo2System::new(num_vars as u32);
        let build_system = system_op.is_none();
        let system;
        if build_system {
            system = &mut new_system;
            c.iter().for_each(|&x| system.add(Modulo2Equation::new(x, num_vars as u32)));
        } else {system = system_op.unwrap()};
        
        let mut weight: Vec<u32> = vec![0; num_vars];
        let mut priority: Vec<u32> = vec![0; num_equations];

        //Guardiamo tutte le varibili
        for &v in variable.iter(){
            //Salviamo (il riferimento al)la lista delle equazioni che contengono la variabile
            let eq = &mut var2_eq[v as usize];
            if eq.len() == 0 {continue};

            let mut curr_eq = eq[0];
            let mut curr_coeff = true;
            let mut j = 0;

            //Guardiamo tutte le equazioni che contengono la variabile
            for i in 1..eq.len() {
                //Controlliamo se è nuova (ci potrebbero essere equazioni ripetute)
                if eq[i] != curr_eq {
                    assert!(eq[i] > curr_eq);
                    //Aggiungiamo l'equazione corrente al sistema se necessario
                    if curr_coeff {
                        if build_system { system.equations[curr_eq as usize].borrow_mut().add(v as usize); }
                        weight[v as usize] += 1;
                        priority[curr_eq as usize] += 1;
                        eq[j] = curr_eq;
                        j += 1;
                    }
                    //Aggiorniamo l'equazione corrente
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

        //VIGNA: nella versione originale viene usato un array identity (t[i] = i) 
        //di cui non capisco l'utilità
        let mut variables = vec![0; num_vars];
        {
            let mut count = vec![0; num_equations+1];

            //VIGNA: non so quanto spingere sulla versione funzionale con iteratori.
            //t.iter().for_each(|&x| count[weight[x] as usize] += 1);
            for x in 0..num_vars {count[weight[x] as usize] += 1};
            //(1..num_equations).for_each(|i| count[i] += count[i-1]);
            for i in 1..num_equations {count[i] += count[i-1]};
            for i in (0..num_vars).rev() {
                count[weight[i] as usize] -= 1;
                variables[count[weight[i] as usize]] = i as u32;
            }
        }

        //Lista equazioni non dense con priorità 0 o 1
        let mut equation_list: Vec<u32> = (0..priority.len() as u32)
        .filter(|&x| priority[x as usize] <= 1)
        .collect();

        //Equazioni dense che appartengono al sistema (fatte solo di variabili attive)
        let mut dense: Vec<Rc<RefCell<Modulo2Equation>>> = Vec::new();
        //Equazioni che definiscono una variabile risolta in termini di variabili attive
        let mut solved: Vec<Rc<RefCell<Modulo2Equation>>> = Vec::new();
        //Variabili risolte (parallelo a soved)
        let mut pivots: Vec<u32> = Vec::new();

        let equations = &system.equations;
        //Un bit vector contenente 1 in corrispondenza di ogni variabile idle
        let mut idle_normalized = vec![usize::MAX; equations[0].borrow().bit_vector.bits().len()];

        //A solo scopo di debug
        let mut num_active = 0;

        let mut remaining = equations.len();
        while remaining != 0 {
            if equation_list.is_empty() {
                //Rendo una nuova variabile attiva
                let mut var = variables.pop().unwrap();
                while weight[var as usize] == 0 {var = variables.pop().unwrap()};
                num_active += 1;
                idle_normalized[var as usize / usize::BITS as usize] ^= 1 << (var % usize::BITS);
                var2_eq[var as usize].iter().for_each(|&eq|{
                    priority[eq as usize] -= 1;
                    if priority[eq as usize] == 1 {equation_list.push(eq)}
                });
            }
            else {
                remaining -= 1;
                let first = equation_list.pop().unwrap(); //An equation of weight 0 or 1
                let ref_equation = &equations[first as usize];
                let equation = ref_equation.borrow();

                if priority[first as usize] == 0 {
                    if equation.is_unsolvable() {return false};
                    if equation.is_identity() {continue};
                    dense.push(Rc::clone(&ref_equation));
                } else if priority[first as usize] == 1 {
                    let mut word_index = 0;
                    while (equation.bit_vector.bits()[word_index] & idle_normalized[word_index]) == 0 {word_index += 1}
                    let pivot = word_index * usize::BITS as usize + (equation.bit_vector.bits()[word_index] & idle_normalized[word_index]).trailing_zeros() as usize;
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
            solution[pivot as usize] = eq.c ^ Modulo2Equation::scalar_product(&eq.bit_vector.bits(), &solution);
        }

        true
    }
}