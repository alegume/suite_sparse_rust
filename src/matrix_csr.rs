use std::{collections::VecDeque, process::abort};
use std::cmp::max;
use std::collections::HashMap;
use rand::Rng;


use crate::read_files::{Element, read_matrix_market_file_coordinates};

#[derive(Debug, Clone)]
pub struct Matrix {
    /* ROW_INDEX[j] is the total number of nonzeros above row j.
    Each (row_index[n+1] - row_index[n]) represent a row
    */
    // TODO: use Option here
    // pub v:Option<Vec<f64>>,
    // Or leave that way and use into_iter() to convert
    // Vec<Option<f64>> to Option<Vec<f64>>
    pub v: Vec<f64>, // non zeros values
    pub col_index: Vec<usize>, // column indices of values in v
    pub row_index: Vec<usize>, // indices (in v and row_index) where the rows starts
    // pub criticals_neighbours: HashMap<usize, Vec::<usize>>, // Critical vertex (max bw)
    pub bw: usize, // Current bandwidth
    pub max_degree: usize,
    pub min_bw: usize,
    pub m: usize,
    pub n: usize,
    pub nz_len: usize,
}


impl Matrix {
    pub fn new(v_size:usize, m:usize, n:usize, nz_len:usize) -> Self {
        Self {
            v: Vec::with_capacity(v_size),
            row_index: Vec::with_capacity(m+1),
            col_index: Vec::with_capacity(nz_len),
            //  TODO: tune this capacity or remove
            // criticals_neighbours: HashMap::new(),
            bw: 0,
            max_degree: 0,
            min_bw: 0,
            m,
            n,
            nz_len
        }
    }

    pub fn bandwidth(&mut self) -> usize {
        let mut bandwidth:usize = 0;
        let mut n_row:usize = 0;

        // Each entry on row_index represents a ROW!
        while n_row < self.row_index.len() - 1 {
            let row = self.get_columns_of_row(n_row);
            for j in row { // Columns in a row
                if n_row.abs_diff(*j as usize) > bandwidth {
                    bandwidth = n_row.abs_diff(*j as usize);
                }
            }
            n_row += 1;
        }
        self.bw = bandwidth;
        // TODO: remover return?
        bandwidth
    }

    fn get_columns_of_row(&self, n:usize) -> &[usize] {
        // if n < self.m {
            let start = self.row_index[n];
            // dbg!(self.row_index.len());
            let stop = self.row_index[n + 1];
            &self.col_index[start..stop]
        // } else { &[] }
    }

    fn get_values_of_row(&self, n:usize) -> &[f64] {
        if n < self.m {
            let start = self.row_index[n] as usize;
            let stop = self.row_index[n + 1] as usize;
            &self.v[start..stop]
        } else { &[] }
    }

    // Vec of degrees of each row
    fn degrees(&self) -> Vec<usize> {
        self.row_index.
            windows(2).
            map(|i| i[1] - i[0]).
            collect::<Vec<usize>>()
    }

    // Degree of row i
    fn degree(&self, i:usize) -> usize {
        if i < self.row_index.len() - 1 {
            self.row_index[i+1] - self.row_index[i]
        } else { 0 }
    }

    pub fn cmr(&mut self, start_v: usize) -> Vec<usize>{
        let mut lines_visited:Vec<usize> = vec![std::usize::MAX; max(self.m, self.n)];
        // push_back to add to the queue and pop_front to remove from the queue.
        let mut to_visit: VecDeque<usize> = VecDeque::from([start_v]);
        let mut n:usize = std::cmp::max(self.m, self.n); 

        // Proceeds whit CMr based on vertex start_v
        self.cycle_through_queue_bfs(&mut to_visit, &mut lines_visited, &mut n);

        // Find if any vertex are left unvisited (e.g. diconected graph)
        for i in 0..self.m {
            if lines_visited[i] == std::usize::MAX {
                to_visit.push_back(i);
                self.cycle_through_queue_bfs(&mut to_visit, &mut lines_visited, &mut n);
            }
        }
        // dbg!(&lines_visited);
        self.reorder(&lines_visited);
        lines_visited
    }

    // Cycle through queue in breadth-first search and reverse labeling
    fn cycle_through_queue_bfs(&self, to_visit:&mut VecDeque<usize>, lines_visited:&mut Vec<usize>, n: &mut usize) {
        while let Some(i) = to_visit.pop_front() {
            if lines_visited[i] == std::usize::MAX { 
                let row = self.get_columns_of_row(i); // Get row of i (neighbours of i)
                let mut row2 = row.to_vec(); // Make a copy
                // Sort by degree
                row2.sort_by(|a, b| {
                    self.degree(*a).cmp(&self.degree(*b))
                });
                for j in row2 {
                    if j < self.m && lines_visited[j] == std::usize::MAX {
                        to_visit.push_back(j);
                    } else if lines_visited[j] == std::usize::MAX {
                        *n -= 1;
                        lines_visited[j] = *n;
                    }
                }
                *n -= 1;
                lines_visited[i] = *n;
            }
        }
    }

    // Reorder vertex for cmr
    pub fn reorder(&mut self, new_rows: &Vec<usize>) {
        let mut row_offset = Vec::with_capacity(self.m);
        let mut col_index = Vec::with_capacity(self.col_index.len());
        let mut v:Vec<f64> = Vec::with_capacity(self.v.len());
        let mut old_rows:Vec<usize> = vec![0; max(self.m, self.n)];

        for (i, val) in new_rows.iter().enumerate() {
            old_rows[*val] = i;
        }
        // Starts with 0
        row_offset.push(0);
        for new in old_rows.iter() {
            // If n_columns > n_rows create empty row
            if new >= &self.m {
                row_offset.push(row_offset.last().copied().unwrap());
                continue;
            }
            // Change col_offsets 
            let start = col_index.len(); // Where new colum starts
            let old_cols = self.get_columns_of_row(*new);
            for e in old_cols { // New columns
                col_index.push(new_rows[*e]); // TODO: Verify optimization
            }
            col_index[start..].sort(); // Sort last part by columns
            //  Change V's if its the case
            if self.v.len() > 0 {
                let values = self.get_values_of_row(*new);
                let mut v_slc:Vec<(&usize,&f64)> = col_index[start..].iter().zip(values.iter()).collect();
                v_slc.sort_by_key(|e| e.0);
                for (_, value) in v_slc {
                    v.push(*value);
                }
            }

            // Calculate row offset (size of old row)
            row_offset.push(col_index.len());
        }
        // Change matrix
        self.m = max(self.m, self.n);
        self.n = self.n;
        self.v = v;
        self.col_index = col_index;
        self.row_index = row_offset;
    }


    /*pub fn criticals_neighbours_old(&mut self) -> HashMap<usize, Vec<usize>>{
        let mut n_row:usize = 0;
        let mut criticals_neighbours:HashMap<usize, Vec<usize>> = HashMap::new();

        self.bandwidth(); // Calculate self.bw
        // Each entry on row_index represents a ROW!
        while n_row < self.row_index.len() - 1 {
            let row = self.get_columns_of_row(n_row);
            for j in row { // Columns in a row
                if *j == n_row {continue;}
                if n_row.abs_diff(*j) == self.bw {
                    let row: Vec<usize> = row.to_vec();
                    let row = row.clone()
                        .into_iter()
                        .filter(|value| *value != n_row)
                        .collect();
                    let row_j: Vec<usize> = self.get_columns_of_row(*j).to_vec();
                    let row_j = row_j.clone()
                        .into_iter()
                        .filter(|value| *value != *j)
                        .collect();
                    criticals_neighbours.insert(n_row, row);
                    criticals_neighbours.insert( *j, row_j);
                }
            }
            n_row += 1;
        }
        criticals_neighbours
    }*/

    // Vertices in edges with bigest bandwidth
    pub fn criticals_neighbours(&mut self) -> Vec<usize>{
        let mut n_row:usize = 0;
        let mut criticals_neighbours:Vec<usize> = Vec::new();

        // TODO: remover
        self.bandwidth(); // Calculate self.bw
        while n_row < self.row_index.len() - 1 {
            let row = self.get_columns_of_row(n_row);
            for j in row { // Columns in a row
                if *j == n_row {continue;}
                if n_row.abs_diff(*j) == self.bw {
                    criticals_neighbours.push(n_row);
                    // criticals_neighbours.push(*j);
                }
            }
            n_row += 1;
        }
        // TODO: optimize
        criticals_neighbours.sort_unstable();
        criticals_neighbours.dedup();
        criticals_neighbours
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
        dbg!(diff_u);
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

    // Calculate bw of vertex u
    fn bw_vertex(&self, u:usize) -> usize {
        let mut bw_v:usize = 0;

        let u_neighbour = self.get_columns_of_row(u);
        for v in u_neighbour { // Columns in a row
            let diff: usize = u.abs_diff(*v as usize);
            if diff > bw_v {
                bw_v = diff;
            }
        }
        bw_v
    }

    // Swap vertices if it's good and update bw
    pub fn vertex_to_swap_with(&mut self, u: &usize) -> Vec<usize> {
        // Test if swap is good
        let mut old_bw_u:usize = 0;
        let mut bw_u:usize = 0;
        let mut v_best:&usize = u;

        // bw of original vertex u
        let u_neighbour = self.get_columns_of_row(*u);
        for v in u_neighbour { // Columns in a row
            // if u == v {continue;}
            if u.abs_diff(*v as usize) > old_bw_u {
                old_bw_u = u.abs_diff(*v as usize);
                v_best = v;
            }
        }
        // dbg!(u, old_bw_u, u_neighbour, v_best);
        u_neighbour.to_vec().into_iter().filter(|x| x != u).collect()

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
                let u = solution.get(u).unwrap().clone();
                let v = solution.get(v).unwrap().clone();
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

    pub fn print(&self) {
        let mut n_row:usize = 0;

        print!("\n    ");
        for n in 0..self.n {
            print!("{} | ", n);
        }
        println!();
        // Each entry on row_index represents a ROW!
        while n_row < self.row_index.len() - 1 {
            let row = self.get_columns_of_row(n_row);
            print!("{} |", n_row);
            let mut count: usize = 0;
            for j in row { // Columns in a row
                let j = j + 1;
                for _ in 1..j-count {
                    print!(" 0 |");
                }
                count = j;
                // println!("\tj={} count={}", j, count);
                print!(" x |");
            }
            if count < self.n {
                print!(" 0 |");
            }
            n_row += 1;
            println!();
        }
        println!();
    }
}


pub fn mm_file_to_csr(file: &str) -> Matrix {
    let mut coordinates: Vec<Element>;
    let (n, m): (usize, usize); 
    (coordinates, m, n) = read_matrix_market_file_coordinates(file);
    let len_v: usize;
    if let Some(_) = coordinates[0].v {
        len_v = coordinates.len();
    } else { len_v = 0 }
    let mut matrix = Matrix::new(len_v, m, n, coordinates.len());
    // Sort in regard of i and then j
    coordinates.sort_unstable_by_key(|e| (e.i, e.j) );

    // row_index always starts whit 0 (first line)
    matrix.row_index.push(0);

    for i in 0..m {
        let row: Vec<&Element> = coordinates.iter()
            .filter(|e| e.i == i)
            .collect();

        for el in row.iter() {
            if let Some(v) = el.v { 
                matrix.v.push(v);
            }
            matrix.col_index.push(el.j);
        }

        if row.len() > 0 {
            matrix.row_index.push(matrix.col_index.len());
            // Find max_degree
            if row.len() > matrix.max_degree {matrix.max_degree = row.len();}
        } else {
            matrix.row_index.push(matrix.row_index.last().copied().unwrap());
        }
    }
    matrix.min_bw = matrix.max_degree / 2;
    assert_eq!(matrix.row_index.len(), m + 1);
    assert_eq!(matrix.col_index.len(), matrix.nz_len);

    matrix
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mm_file_to_csr_test() {
        let file = "./instances/tests/test1.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [5.0, 8.0, 3.0, 6.0]);
        assert_eq!(matrix.col_index, [0, 1, 2, 1]);
        assert_eq!(matrix.row_index, [0, 1, 2, 3, 4]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 3);

        let file = "./instances/tests/test2.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
        assert_eq!(matrix.col_index, [0, 1, 1, 3, 2, 3, 4, 5]);
        assert_eq!(matrix.row_index, [0, 2, 4, 7, 8]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 6);

        let file = "./instances/tests/test3.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [2.0, 3.0, 1.0, 3.0, 2.0, 5.0, 2.0, 4.0, 1.0, 5.0, 4.0, 2.0]);
        assert_eq!(matrix.col_index, [0, 1, 3, 0, 1, 3, 2, 3, 0, 1, 2, 3]);
        assert_eq!(matrix.row_index, [0, 3, 6, 8, 12]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 4);
    }

    #[test]
    fn bw_test() {
        /* Stress tests  */
        // let file = "./instances/tests/apache2.mtx";
        // let mut matrix = mm_file_to_csr(file);
        // assert_eq!(matrix.bandwidth(), 65837);
        // let file = "./instances/tests/pwtk.mtx";
        // let mut matrix = mm_file_to_csr(file);
        // assert_eq!(matrix.bandwidth(), 189331);

        let file = "./instances/tests/test1.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 2);
        assert_eq!(matrix.degrees(), [1, 1, 1, 1]);
        matrix.cmr(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);
        // TODO: insert new matrix to assert
        
        let file = "./instances/tests/test2.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 2);
        println!("{:?}", matrix);
        assert_eq!(matrix.degrees(), [2, 2, 3, 1]);
        matrix.cmr(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);

        let file = "./instances/tests/test3.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 3);
        assert_eq!(matrix.degrees(), [3, 3, 2, 4]);
        matrix.cmr(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);

        let file = "./instances/bcspwr01.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 38);
        matrix.cmr(matrix.col_index[0]);
        // assert_eq!(matrix.bandwidth(), 8);
        // CMr 8

        let file = "./instances/lns__131.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 111);
        matrix.cmr(matrix.col_index[0]);
        // CMr 39

        let file = "./instances/mcca.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 65);
        matrix.cmr(matrix.col_index[0]);
        // CMr 3

        let file = "./instances/will199.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 169);
        matrix.cmr(matrix.col_index[0]);
        // CMr 115

        let file = "./instances/662_bus.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 335);
        matrix.cmr(matrix.col_index[0]);
        // CMr 112

        let file = "./instances/dwt__361.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 50);
        matrix.cmr(matrix.col_index[0]);
        // CMr 25

        let file = "./instances/sherman4.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 368);
        matrix.cmr(matrix.col_index[0]);
        // CMr 0??
    }

}