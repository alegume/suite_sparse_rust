use rand::Rng;

// use std::collections::HashMap;
// use std::collections::VecDeque;

use crate::matrix_csr::Matrix;

impl Matrix {
    // Swap vertices if it's good and update bw
    pub fn vertex_to_swap_with(&mut self, u: &usize) -> Vec<usize> {
        // Test if swap is good
        let mut old_bw_u:usize = 0;
        let mut bw_u:usize = 0;
        let mut v_best:&usize = u;

        // bw of original vertex u
        let u_neighbour = self.get_columns_of_row(*u).clone();
        for v in u_neighbour { // Columns in a row
            // if u == v {continue;}
            if u.abs_diff(*v) > old_bw_u {
                old_bw_u = u.abs_diff(*v);
                v_best = v;
            }
        }
        // dbg!(u, old_bw_u, u_neighbour, v_best);
        u_neighbour.iter().filter(|x| x != &u).collect::<Vec<&usize>>();

        // // bw of vertex u if swap occurs
        // for v in u_neighbour { // Columns in a row
        //     if u.abs_diff(*v as usize) > bw_u {
        //         bw_u = u.abs_diff(*v as usize);
        //     }
        // }

        // dbg!(u, v, bw_u, bw_v);

        // TODO: rever

        // TODO: Só aceitar troca se melhora a solucao?

        // TODO: funcao Swap otimizada
        // if () and () {
        // }
        return Vec::new();
        // Update bw 
    }


    pub fn local_search(&mut self, solution: &mut Vec<usize>) -> Vec<usize>{
        // let mut criticals_neighbours:HashMap<usize, Vec<usize>> = HashMap::new();
        // self.print();
        let mut bw_0 = self.bandwidth();
        // dbg!(bw_0);
        // let mut solution_0 = solution.clone();
        let mut improved:bool = false;

        for (u) in self.criticals_neighbours() {
            let swaps = self.vertex_to_swap_with(&u);
            for v in swaps {
                println!("swap ({}, {}) s={:?}", u, v, solution);
                // TODO: Só reordena se melhora
                // let swap:bool = self.vertex_swap_bw_update(&u, &v);
                let mut u = solution.get(u).unwrap().clone();
                let mut v = solution.get(v).unwrap().clone();
                self.print();
                println!("\t ANTES; o={:?}",solution);
                solution.swap(u, v);
                self.reorder(&solution);
                self.bandwidth();
                println!("\t mudou ({}, {}); o={:?}",u,v,solution);
                if self.bw > bw_0 {
                    solution.swap(u, v);
                    self.reorder(&solution);
                    self.bandwidth();
                    println!("\tpiorou");
                    println!("\t ({:?}, {:?}); o={:?}",&u,&v, solution);
                    self.print();
                } else {
                    bw_0 = self.bw;
                    improved = true;
                }
                // println!("swap ({}, {}) s={:?}\n", u, v, solution);
                // println!("bf={}, bw_0={})", self.bw, bw_0);

            }
            // if !improved {
            //     self.reorder(&solution_0);
            // }
        }
        // Considera o melhor
        // self.reorder(&solution_0);
        // self.bandwidth();
        // println!("SELF.bw={}, bw_0={})", self.bw, bw_0);
        // solution_0
        vec![]
    }
    
    pub fn ils(&mut self) {
        // !!!!!!!!! Não pode começar pelos criticos??????
        // TODO:gerar vertice aleatoria para inicio
        let mut rng = rand::thread_rng();
        let v: usize = rng.gen_range(1..self.m);
        
        // println!("\tcriticals = {:?}", self.criticals);
        for _ in 0..1 {
            let mut new_rows = self.cmr(self.col_index[0]);
            // self.print();
            self.local_search(&mut new_rows);
        }
    }


    // Swap vertices if it's good and update bw
    pub fn vertices_swap_bw_update(&mut self, u: &usize, v: &usize) -> bool {
        // TODO: deal with it
        assert!(u < v);
        //// Swap the values in col_index
        for x in &mut self.col_index {
            if *x == *u {
                *x = *v
            } else if *x == *v {
                *x = *u
            }
        }
        //// Swap the order of values in col_index
        let start_col_u = self.row_index[*u];
        let stop_col_u = self.row_index[*u + 1];
        let start_col_v = self.row_index[*v];
        let stop_col_v = self.row_index[*v + 1];
        let mut first_part:Vec<usize> = self.col_index[..start_col_u].to_owned();
        let mut old_col_u:Vec<usize> = self.col_index[start_col_u..stop_col_u].to_owned();
        let mut middle:Vec<usize> = self.col_index[stop_col_u..start_col_v].to_owned();
        let mut old_col_v:Vec<usize> = self.col_index[start_col_v..stop_col_v].to_owned();
        let mut last:Vec<usize> = self.col_index[stop_col_v..].to_owned();
        // Sort columns to respect CSR definitions
        // This does not work because first, middle and last must be sorted line by line
        // old_col_u.sort_unstable();
        // old_col_v.sort_unstable();
        // first_part.sort_unstable();
        // middle.sort_unstable();
        // last.sort_unstable();
        // println!("f={:?}",&first_part);
        // println!("o_v={:?}",&old_col_v);
        // println!("middle={:?}",&middle);
        // println!("o_u={:?}", &old_col_u);
        // println!("l={:?}", &last);


        // Replace with correct columns
        self.col_index = [first_part, old_col_v, middle, old_col_u, last].concat();

        //// Fix row_index for u (first part is already ok)
        let diff_u = (stop_col_v - start_col_v);
        // dbg!(diff_u);
        self.row_index[u+1] = start_col_u + (diff_u);
        // update in between
        for i in u+2..v+1 {
            self.row_index[i] += diff_u;
        }
        // Fix for v
        let diff_v = (stop_col_u - start_col_u);
        self.row_index[v+1] = self.row_index[*v] + diff_v;
        // update lasts
        for i in v+2..self.m-1 {
            self.row_index[i] += diff_v;
        }
        println!("{:?}", self.row_index);
        // self.row_index[stop_col_u+1] = start_col_u;
        // let first_part:usize = ;
        // let old_col_u:usize = ;
        // let middle:usize = ;
        // let old_col_v:usize = ;
        // let last:usize = ;

        /// Fix row_index for u
        
        // TODO!! Update bw 
        self.bandwidth();
        true
    }


}