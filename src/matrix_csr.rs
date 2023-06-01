// use std::{collections::VecDeque, process::abort};
// use std::collections::HashMap;
use std::cmp::max;

use crate::read_files::{read_matrix_market_file_coordinates, Element};

#[derive(Debug, Clone)]
pub struct Matrix {
    /* ROW_INDEX[j] is the total number of nonzeros above row j.
    Each (row_index[n+1] - row_index[n]) represent a row
    */
    pub v: Vec<f64>,           // non zeros values
    pub col_index: Vec<usize>, // column indices of values in v
    pub row_index: Vec<usize>, // indices (in v and row_index) where the rows starts
    pub labels: Vec<usize>,
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
            bw: 0,
            max_degree: 0,
            min_bw: 0,
            m,
            n,
            nz_len,
        }
    }

    pub fn bandwidth(&mut self) -> usize {
        let mut bandwidth: usize = 0;
        let mut n_row: usize = 0;
        let mut diff: usize = 0;

        // Each entry on row_index represents a ROW!
        while n_row < self.row_index.len() - 1 {
            let row = self.get_columns_of_row(n_row);
            for j in row {
                // Columns in a row
                diff = self.labels[n_row].abs_diff(self.labels[*j]);
                if diff > bandwidth {
                    bandwidth = diff;
                }
            }
            n_row += 1;
        }
        self.bw = bandwidth;
        // TODO: remover return?
        bandwidth
    }

    pub fn get_columns_of_row(&self, n: usize) -> &[usize] {
        // if n < self.m {
        let start = self.row_index[n];
        // dbg!(self.row_index.len());
        let stop = self.row_index[n + 1];
        &self.col_index[start..stop]
        // } else { &[] }
    }

    pub fn get_values_of_row(&self, n: usize) -> &[f64] {
        if n < self.m {
            let start = self.row_index[n];
            let stop = self.row_index[n + 1];
            &self.v[start..stop]
        } else {
            &[]
        }
    }

    // Vec of degrees of each row
    pub fn degrees(&self) -> Vec<usize> {
        self.row_index
            .windows(2)
            .map(|i| i[1] - i[0])
            .collect::<Vec<usize>>()
    }

    // Degree of row i
    pub fn degree(&self, i: usize) -> usize {
        if i < self.row_index.len() - 1 {
            self.row_index[i + 1] - self.row_index[i]
        } else {
            0
        }
    }

    // Vertices in edges with bigest bandwidth
    pub fn criticals_neighbours(&mut self) -> Vec<usize> {
        let mut n_row: usize = 0;
        let mut criticals_neighbours: Vec<usize> = Vec::new();

        // TODO: remover
        self.bandwidth(); // Calculate self.bw
        while n_row < self.row_index.len() - 1 {
            let row = self.get_columns_of_row(n_row);
            for j in row {
                // Columns in a row
                if *j == n_row {
                    continue;
                }
                if n_row.abs_diff(*j) == self.bw {
                    criticals_neighbours.push(n_row);
                    // criticals_neighbours.push(*j);
                }
            }
            n_row += 1;
        }
        // TODO: optimize
        // Sort to remove duplications
        criticals_neighbours.sort_unstable();
        criticals_neighbours.dedup(); // remove duplications
        criticals_neighbours
    }

    // Calculate bw of vertex u
    // TODO: refac with labels
    fn bw_vertex(&self, u: usize) -> usize {
        let mut bw_v: usize = 0;

        let u_neighbour = self.get_columns_of_row(u);
        for v in u_neighbour {
            // Columns in a row
            let diff: usize = u.abs_diff(*v);
            if diff > bw_v {
                bw_v = diff;
            }
        }
        bw_v
    }

    pub fn print(&self) {
        let mut n_row: usize = 0;

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
            for j in row {
                // Columns in a row
                let j = j + 1;
                for _ in 1..j - count {
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

// Create matrix from HB file
pub fn mm_file_to_csr(file: &str) -> Matrix {
    let mut coordinates: Vec<Element>;
    let (n, m): (usize, usize);
    (coordinates, m, n) = read_matrix_market_file_coordinates(file);
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
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [5.0, 8.0, 3.0, 6.0]);
        assert_eq!(matrix.col_index, [0, 1, 2, 1]);
        assert_eq!(matrix.row_index, [0, 1, 2, 3, 4]);
        assert_eq!(matrix.labels, [0, 1, 2, 3]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 3);

        let file = "./input/tests/test2.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
        assert_eq!(matrix.col_index, [0, 1, 1, 3, 2, 3, 4, 5]);
        assert_eq!(matrix.row_index, [0, 2, 4, 7, 8]);
        assert_eq!(matrix.labels, [0, 1, 2, 3, 4, 5]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 6);

        let file = "./input/tests/test3.mtx";
        let matrix = mm_file_to_csr(file);
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
        /* Stress tests  */
        // let file = "./input/tests/apache2.mtx";
        // let mut matrix = mm_file_to_csr(file);
        // assert_eq!(matrix.bandwidth(), 65837);
        // let file = "./input/tests/pwtk.mtx";
        // let mut matrix = mm_file_to_csr(file);
        // assert_eq!(matrix.bandwidth(), 189331);

        let file = "./input/tests/test1.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 2);
        assert_eq!(matrix.degrees(), [1, 1, 1, 1]);
        matrix.cmr(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);
        // TODO: insert new matrix to assert

        let file = "./input/tests/test2.mtx";
        let mut matrix = mm_file_to_csr(file);
        println!("{:?}", matrix);
        assert_eq!(matrix.bandwidth(), 2);
        println!("{:?}", matrix);
        assert_eq!(matrix.degrees(), [2, 2, 3, 1]);
        matrix.cmr(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);

        let file = "./input/tests/test3.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 3);
        assert_eq!(matrix.degrees(), [3, 3, 2, 4]);
        matrix.cmr(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);

        let file = "./input/general/bcspwr01.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 38);
        matrix.cmr(matrix.col_index[0]);
        // assert_eq!(matrix.bandwidth(), 8);
        // CMr 8

        let file = "./input/general/lns__131.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 111);
        matrix.cmr(matrix.col_index[0]);
        // CMr 39

        let file = "./input/general/mcca.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 65);
        matrix.cmr(matrix.col_index[0]);
        // CMr 3

        let file = "./input/general/will199.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 169);
        matrix.cmr(matrix.col_index[0]);
        // CMr 115

        let file = "./input/general/662_bus.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 335);
        matrix.cmr(matrix.col_index[0]);
        // CMr 112

        let file = "./input/general/dwt__361.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 50);
        matrix.cmr(matrix.col_index[0]);
        // CMr 25

        let file = "./input/general/sherman4.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 368);
        matrix.cmr(matrix.col_index[0]);
        // CMr 0??
    }

    #[test]
    fn criticals_neighbours_test() {
        let file = "./input/tests/test1.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.criticals_neighbours(), vec![3]);

        let file = "./input/tests/test2.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.criticals_neighbours(), vec![1, 2, 3]);

        let file = "./input/tests/test3.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.criticals_neighbours(), vec![0, 3]);
    }
}
