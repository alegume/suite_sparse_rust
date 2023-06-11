use std::{collections::VecDeque, io::repeat};
// use std::{collections::VecDeque, process::abort};
// use std::collections::HashMap;
use std::cmp::max;

use crate::read_files::{read_matrix_market_file_coordinates, Element, read_matrix_market_file_coordinates_no_values};

#[derive(Debug, Clone)]
pub struct Matrix {
    /* ROW_INDEX[j] is the total number of nonzeros above row j.
    Each (row_index[n+1] - row_index[n]) represent a row
    */
    pub v: Vec<f64>,           // non zeros values
    pub col_index: Vec<usize>, // column indices of values in v
    pub row_index: Vec<usize>, // indices (in v and row_index) where the rows starts
    pub labels: Vec<usize>,
    pub old_labels: Vec<usize>,
    pub bw: usize, // Current bandwidth
    pub max_degree: usize,
    pub min_bw: usize,
    pub m: usize,
    pub n: usize,
    pub nz_len: usize,
}

impl Matrix {
    pub fn new(v_size: usize, m: usize, n: usize, nz_len: usize) -> Self {
        Self {
            v: Vec::with_capacity(v_size),
            row_index: Vec::with_capacity(m + 1),
            col_index: Vec::with_capacity(nz_len),
            labels: Vec::with_capacity(max(m, n)),
            old_labels: Vec::new(),
            bw: 0,
            max_degree: 0,
            min_bw: 0,
            m,
            n,
            nz_len,
        }
    }

    /// Perform breadth-first search (BFS)
    /// Return leave nodes and maximun eccentricity
    fn bfs(&self, v: usize) -> (Vec<usize>, usize) {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.m];
        let mut distances = vec![0; self.m];
        let mut eccentricity = 0;

        queue.push_back(v);
        visited[v] = true;
        // Find if any vertex are left unvisited (e.g. diconected graph)
        for i in 0..self.m {
            if visited[i] {
                continue;
            }
            while let Some(v) = queue.pop_front() {
                // println!("{} => {:?}", v, self.get_columns_of_row(v));
                for u in self.get_columns_of_row(v) {
                    if !visited[*u] {
                        queue.push_back(*u);
                        visited[*u] = true;
                        distances[*u] = distances[v] + 1;
                        eccentricity = distances[*u];
                    }
                }
            }
        }
        // dbg!(&distances);
        let mut leaves: Vec<usize> = Vec::new();
        if eccentricity > 0{
            // Find vertices in the last level of the RLS
            leaves =  distances
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x == eccentricity)
                .map(|(i, _)| i)
                .collect();
        }

        (leaves, eccentricity)
    }

    // George-Liu for for finding pseudo-peripheral vertex
    pub fn pseudo_george_liu(&mut self, v: usize) -> usize {
        let mut v = v;
        let mut leaves_v: Vec<usize>;
        let mut leaves_u: Vec<usize>;
        let mut eccentricity_u = 0;
        let mut eccentricity_v = 0;

        // BFS 
        (leaves_v, eccentricity_v) = self.bfs(v);
        loop {
            // Find the leave of minimum degree
            let mut u:usize = leaves_v.pop().unwrap_or(v);
            if u == v { // There are no leaves (directed/disconected graph)
                break;
            }
            // Find minimum degree vertex
            let mut u_degree = self.degree(u);
            for l in &leaves_v {
                let l_degree = self.degree(*l);
                if l_degree < u_degree {
                    u = *l;
                    u_degree = l_degree;
                }
            }
            // BFS in u
            (leaves_u, eccentricity_u) = self.bfs(u);
            let troca: bool = (eccentricity_u > eccentricity_v);
            // println!("{}|{:?}|{}|{}|{:?}|{}|{:?}", v, leaves_v, eccentricity_v, u, leaves_u, eccentricity_u, troca);
            if eccentricity_u > eccentricity_v {
                v = u;
                eccentricity_v = eccentricity_u;
                leaves_v = leaves_u;
            } else {
                break;
            }
        }
        // self.print();
        // dbg!(&distances, &leaves, &u, &u_degree);
        // dbg!(diameter);
        // assert_eq!(eccentricity, *distances.iter().max().unwrap());

        v
        // self.labels[]
    }

    // Reorder matrix based on a new labeling
    pub fn reorder(&mut self, new_rows: &[usize]) {
        let mut row_offset = Vec::with_capacity(self.m);
        let mut col_index = Vec::with_capacity(self.col_index.len());
        let mut v: Vec<f64> = Vec::with_capacity(self.v.len());
        let mut old_rows: Vec<usize> = vec![0; max(self.m, self.n)];

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
            for e in old_cols {
                // New columns
                col_index.push(new_rows[*e]); // TODO: Verify optimization
            }
            col_index[start..].sort(); // Sort last part by columns
                                       //  Change V's if its the case
            if !self.v.is_empty() {
                let values = self.get_values_of_row(*new);
                let mut v_slc: Vec<(&usize, &f64)> =
                    col_index[start..].iter().zip(values.iter()).collect();
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
        self.v = v;
        self.col_index = col_index;
        self.row_index = row_offset;
    }

    // Calculate bandwidth of matrix and return it
    pub fn bandwidth(&mut self) -> usize {
        let mut bandwidth: usize = 0;
        let mut n_row: usize = 0;
        let mut diff: usize = 0;

        // Each entry on row_index represents a ROW!
        while n_row < self.row_index.len() - 1 {
            let row = self.get_columns_of_row(n_row);
            for j in row {
                if n_row == *j {
                    continue;
                }
                let j = self.labels[*j];
                let i = self.labels[n_row];
                // Columns in a row
                diff = i.abs_diff(j);
                if diff > bandwidth {
                    bandwidth = diff;
                }
            }
            n_row += 1;
        }
        self.bw = bandwidth;
        bandwidth
    }

    // Calculate bw of vertex u
    fn bw_vertex(&self, v: usize) -> usize {
        let mut bw_v: usize = 0;

        let v_neighbour = self.get_columns_of_row(v);
        for u in v_neighbour {
            if v == *u {
                continue;
            }
            let v = self.labels[v];
            let u = self.labels[*u];
            let diff: usize = v.abs_diff(u);
            if diff > bw_v {
                bw_v = diff;
            }
        }
        bw_v
    }

    // Get neighbours of vertex n
    pub fn get_columns_of_row(&self, n: usize) -> &[usize] {
        let start = self.row_index[n];
        let stop = self.row_index[n + 1];
        &self.col_index[start..stop]
    }

    // Get values os row n (neighbours of vertex n)
    pub fn get_values_of_row(&self, n: usize) -> &[f64] {
        if n < self.m {
            let start = self.row_index[n];
            let stop = self.row_index[n + 1];
            &self.v[start..stop]
        } else {
            &[]
        }
    }

    // Degree of row i
    pub fn degree(&self, i: usize) -> usize {
        if i < self.row_index.len() - 1 {
            let i = self.labels[i]; // Consider the label of i
            self.row_index[i + 1] - self.row_index[i]
        } else {
            0
        }
    }

    // Vertices in edges with bigest bandwidth
    pub fn criticals(&mut self) -> Vec<usize> {
        let mut n_row: usize = 0;
        let mut criticals_neighbours: Vec<usize> = Vec::new();

        self.bandwidth();
        while n_row < self.row_index.len() - 1 {
            if self.bw_vertex(n_row) == self.bw {
                let i = self.labels[n_row];
                criticals_neighbours.push(i);
            }
            n_row += 1;
        }
        // TODO: optimize
        // Sort to remove duplications
        criticals_neighbours.sort_unstable();
        criticals_neighbours.dedup(); // remove duplications
        criticals_neighbours
    }

    pub fn old_label(&self, v: usize) -> usize {
        self.labels.iter().position(|x| x == &v).unwrap()
    }

    // Print considering label
    pub fn print(&self) {
        let mut n_row: usize = 0;

        print!("\n    ");
        for n in 0..self.n {
            print!("{} | ", n);
        }
        println!();
        while n_row < self.row_index.len() - 1 {
            let i = self.old_label(n_row);
            let row = self.get_columns_of_row(i);
            print!("{} |", n_row);
            let mut count: usize = 0;
            // Order by labels
            let mut rows: Vec<usize> = Vec::with_capacity(row.len());
            for j in row {
                rows.push(self.labels[*j]);
            }
            rows.sort_unstable();
            for j in rows {
                // Columns in a row
                let j = j + 1;
                for _ in 1..j - count {
                    print!(" 0 |");
                }
                count = j;
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

// Create matrix from HB file
pub fn mm_file_to_csr(file: &str, values:bool) -> Matrix {
    let mut coordinates: Vec<Element>;
    let (n, m): (usize, usize);
    if values {
        (coordinates, m, n) = read_matrix_market_file_coordinates(file);
    } else {
        (coordinates, m, n) = read_matrix_market_file_coordinates_no_values(file);
    }
    let len_v: usize = if coordinates[0].v.is_some() {
        coordinates.len()
    } else {
        0
    };
    let mut matrix = Matrix::new(len_v, m, n, coordinates.len());
    // Sort in regard of i and then j
    coordinates.sort_unstable_by_key(|e| (e.i, e.j));

    // row_index always starts whit 0 (first line)
    matrix.row_index.push(0);

    for i in 0..m {
        let row: Vec<&Element> = coordinates.iter().filter(|e| e.i == i).collect();

        for el in row.iter() {
            if let Some(v) = el.v {
                matrix.v.push(v);
            }
            matrix.col_index.push(el.j);
        }

        if !row.is_empty() {
            matrix.row_index.push(matrix.col_index.len());
            // Find max_degree
            if row.len() > matrix.max_degree {
                matrix.max_degree = row.len();
            }
        } else {
            matrix
                .row_index
                .push(matrix.row_index.last().copied().unwrap());
        }
        matrix.labels.push(i); // Original labels
    }
    if n > m {
        // In case it have more columns than rows
        for i in 0..n - m {
            matrix.labels.push(m + i);
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
        let file = "./input/tests/test1.mtx";
        let matrix = mm_file_to_csr(file, true);
        assert_eq!(matrix.v, [5.0, 8.0, 3.0, 6.0]);
        assert_eq!(matrix.col_index, [0, 1, 2, 1]);
        assert_eq!(matrix.row_index, [0, 1, 2, 3, 4]);
        assert_eq!(matrix.labels, [0, 1, 2, 3]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 4);

        let file = "./input/tests/test2.mtx";
        let matrix = mm_file_to_csr(file, true);
        assert_eq!(matrix.v, [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
        assert_eq!(matrix.col_index, [0, 1, 1, 3, 2, 3, 4, 5]);
        assert_eq!(matrix.row_index, [0, 2, 4, 7, 8, 8, 8]);
        assert_eq!(matrix.labels, [0, 1, 2, 3, 4, 5]);
        assert_eq!(matrix.m, 6);
        assert_eq!(matrix.n, 6);

        let file = "./input/tests/test3.mtx";
        let matrix = mm_file_to_csr(file, true);
        assert_eq!(
            matrix.v,
            [2.0, 3.0, 1.0, 3.0, 2.0, 5.0, 2.0, 4.0, 1.0, 5.0, 4.0, 2.0]
        );
        assert_eq!(matrix.col_index, [0, 1, 3, 0, 1, 3, 2, 3, 0, 1, 2, 3]);
        assert_eq!(matrix.row_index, [0, 3, 6, 8, 12]);
        assert_eq!(matrix.labels, [0, 1, 2, 3]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 4);
    }

    #[test]
    fn bw_test() {
        /* Stress tests  
        let file = "./input/tests/apache2.mtx";
        let mut matrix = mm_file_to_csr(file, true);
        assert_eq!(matrix.bandwidth(), 65837);
        let file = "./input/tests/pwtk.mtx";
        let mut matrix = mm_file_to_csr(file, true);
        assert_eq!(matrix.bandwidth(), 189331);*/

        let file = "./input/tests/test1.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        assert_eq!(matrix.bandwidth(), 2);
        matrix.labels = matrix.cmr_reorder(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);
        // TODO: insert new matrix to assert

        let file = "./input/tests/test2.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        assert_eq!(matrix.bandwidth(), 2);
        // assert_eq!(matrix.degrees(), [2, 2, 3, 1]);
        matrix.labels = matrix.cmr_reorder(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 5);

        let file = "./input/tests/test3.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        assert_eq!(matrix.bandwidth(), 3);
        // assert_eq!(matrix.degrees(), [3, 3, 2, 4]);
        matrix.cmr_reorder(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);

        let file = "./input/general/bcspwr01.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let mut matrix2 = matrix.clone();
        assert_eq!(matrix.bandwidth(), 38);
        matrix.cmr_reorder(matrix.col_index[0]);
        let p = matrix2.pseudo_george_liu(0);
        matrix2.cmr_reorder(p);
        // assert_eq!(matrix2.bandwidth(), 8);
        // cmr_reorder 8

        let file = "./input/general/lns__131.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        assert_eq!(matrix.bandwidth(), 111);
        matrix.cmr_reorder(matrix.col_index[0]);
        // cmr_reorder 39

        let file = "./input/general/mcca.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        assert_eq!(matrix.bandwidth(), 65);
        matrix.cmr_reorder(matrix.col_index[0]);
        // cmr_reorder 3

        let file = "./input/general/will199.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        assert_eq!(matrix.bandwidth(), 169);
        matrix.cmr_reorder(matrix.col_index[0]);
        // cmr_reorder 115

        let file = "./input/general/662_bus.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        assert_eq!(matrix.bandwidth(), 335);
        matrix.cmr_reorder(matrix.col_index[0]);
        // cmr_reorder 112

        let file = "./input/general/dwt__361.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        assert_eq!(matrix.bandwidth(), 50);
        matrix.cmr_reorder(matrix.col_index[0]);
        // cmr_reorder 25

        let file = "./input/general/sherman4.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        assert_eq!(matrix.bandwidth(), 368);
        matrix.cmr_reorder(matrix.col_index[0]);
        // cmr_reorder 0??
    }

    #[test]
    fn criticals_neighbours_test() {
        let file = "./input/tests/test1.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let mut matrix2 = matrix.clone();
        assert_eq!(matrix.criticals(), vec![3]);
        let order = matrix.cmr_reorder(0);
        matrix2.labels = order;
        assert_eq!(matrix.bandwidth(), 2);
        assert_eq!(matrix.criticals(), vec![0]);
        assert_eq!(matrix.criticals(), matrix2.criticals());

        let file = "./input/tests/test2.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let mut matrix2 = matrix.clone();
        assert_eq!(matrix.criticals(), vec![1, 2, 3]);
        let order = matrix.cmr_reorder(0);
        matrix2.labels = order.clone();
        assert_eq!(matrix.bandwidth(), 2);
        assert_eq!(matrix2.bandwidth(), 2);
        assert_eq!(matrix.criticals(), matrix2.criticals());

        let file = "./input/tests/test3.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let mut matrix2 = matrix.clone();
        assert_eq!(matrix.bandwidth(), 3);
        assert_eq!(matrix.criticals(), vec![0, 3]);
        let order = matrix.cmr_reorder(0);
        matrix2.labels = order;
        assert_eq!(matrix.criticals(), vec![1, 3]);
        assert_eq!(matrix.criticals(), matrix2.criticals());

        let file = "./input/tests/test4-ipo.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let mut matrix2 = matrix.clone();
        assert_eq!(matrix.bandwidth(), 5);
        assert_eq!(matrix.criticals(), vec![0, 5]);
        matrix2.labels = vec![2, 5, 1, 0, 3, 4];
        assert_eq!(matrix2.bandwidth(), 2);
        assert_eq!(matrix2.criticals(), vec![1, 2, 3, 4, 5]);
        let order = matrix.cmr_reorder(0);
        matrix2.labels = order;
        assert_eq!(matrix.criticals(), matrix2.criticals());
    }

    #[test]
    fn labels_degree_test() {
        let file = "./input/tests/test3.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let mut matrix2 = matrix.clone();
        assert_eq!(matrix.bandwidth(), 3);
        assert_eq!(matrix.degree(0), 2);
        assert_eq!(matrix.degree(1), 2);
        assert_eq!(matrix.degree(2), 1);
        assert_eq!(matrix.degree(3), 3);
        let order = matrix.cmr_reorder(matrix.col_index[0]);
        matrix.labels = order.clone();
        assert_eq!(matrix.bandwidth(), 3);
        assert_eq!(matrix.degree(0), 2);
        assert_eq!(matrix.degree(1), 2);
        assert_eq!(matrix.degree(2), 1);
        assert_eq!(matrix.degree(3), 3);
        // Assert that matrix2 == matrix
        assert_eq!(matrix2.bandwidth(), 3);
        assert_eq!(matrix2.degree(0), 2);
        assert_eq!(matrix2.degree(1), 2);
        assert_eq!(matrix2.degree(2), 1);
        assert_eq!(matrix2.degree(3), 3);
        matrix2.labels = order; // Change label according to cmr_reorder
        assert_eq!(matrix2.bandwidth(), 2);
        assert_eq!(matrix2.degree(0), 1);
        assert_eq!(matrix2.degree(1), 3);
        assert_eq!(matrix2.degree(2), 2);
        assert_eq!(matrix2.degree(3), 2);

        let file = "./input/general/lns__131.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let mut matrix2 = matrix.clone();
        assert_eq!(matrix.bandwidth(), 111);
        let order = matrix.cmr_reorder(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 90);
        assert_eq!(matrix2.bandwidth(), 111);
        matrix2.labels = order;
        assert_eq!(matrix2.bandwidth(), 90);

        let file = "./input/general/mcca.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let mut matrix2 = matrix.clone();
        assert_eq!(matrix.bandwidth(), 65);
        let order = matrix.cmr_reorder(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 59);
        assert_eq!(matrix2.bandwidth(), 65);
        matrix2.labels = order;
        assert_eq!(matrix2.bandwidth(), 59);
    }

    #[test]
    fn pseudo_george_liu_test() {
        let file = "./input/tests/test4-ipo.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let p = matrix.pseudo_george_liu(0);
        assert_eq!(p, 3);
        let p = matrix.pseudo_george_liu(1);
        assert_eq!(p, 1);
        let p = matrix.pseudo_george_liu(3);
        assert_eq!(p, 3);
        let p = matrix.pseudo_george_liu(4);
        assert_eq!(p, 3);

        let file = "./input/tests/test3.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let p = matrix.pseudo_george_liu(0);
        assert_eq!(p, 0);
        let p = matrix.pseudo_george_liu(1);
        assert_eq!(p, 1);
        let p = matrix.pseudo_george_liu(3);
        assert_eq!(p, 2);

        let file = "./input/tests/test2.mtx";
        let mut matrix = mm_file_to_csr(file, false);
        let p = matrix.pseudo_george_liu(0);
        assert_eq!(p, 0);
        let p = matrix.pseudo_george_liu(2);
        assert_eq!(p, 2);
        let p = matrix.pseudo_george_liu(3);
        assert_eq!(p, 3);
        
    }
}
