use std::collections::VecDeque;
use std::cmp::max;

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
    pub v:Vec<f64>, // non zeros values
    pub col_index:Vec<usize>, // column indices of values in v
    pub row_index:Vec<usize>, // indices (in v and row_index) where the rows starts
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
            m,
            n,
            nz_len
        }
    }

    pub fn bandwidth(&self) -> usize {
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

    fn degree(&self, i:usize) -> usize {
        if i < self.row_index.len() - 1 {
            self.row_index[i+1] - self.row_index[i]
        } else { 0 }
    }

    pub fn cmr(&mut self, start_v: usize) {
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

    fn reorder(&mut self, new_rows: &Vec<usize>) {
        let mut row_offset = Vec::with_capacity(self.m);
        let mut col_index = Vec::with_capacity(self.col_index.len());
        let mut v:Vec<f64> = Vec::with_capacity(self.v.len());
        let mut old_rows:Vec<usize> = vec![0; max(self.m, self.n)];

        for (i, val) in new_rows.iter().enumerate() {
            old_rows[*val] = i;
        }

        row_offset.push(0);
        for new in old_rows.iter() {
            // If n_columns > n_rows create empty row
            if new >= &self.m {
                row_offset.push(row_offset.last().copied().unwrap());
                continue;
            }
            // Change col_offsets 
            let start = col_index.len();
            let old_cols = self.get_columns_of_row(*new);
            for e in old_cols {
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
        } else {
            matrix.row_index.push(matrix.row_index.last().copied().unwrap());
        }
    }

    assert_eq!(matrix.row_index.len(), m + 1);
    assert_eq!(matrix.col_index.len(), matrix.nz_len);

    matrix
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mm_file_to_csr_test() {
        let file = "test1.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [5.0, 8.0, 3.0, 6.0]);
        assert_eq!(matrix.col_index, [0, 1, 2, 1]);
        assert_eq!(matrix.row_index, [0, 1, 2, 3, 4]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 3);

        let file = "test2.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
        assert_eq!(matrix.col_index, [0, 1, 1, 3, 2, 3, 4, 5]);
        assert_eq!(matrix.row_index, [0, 2, 4, 7, 8]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 6);

        let file = "test3.mtx";
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
        // let file = "apache2.mtx";
        // let mut matrix = mm_file_to_csr(file);
        // assert_eq!(matrix.bandwidth(), 65837);
        // let file = "pwtk.mtx";
        // let mut matrix = mm_file_to_csr(file);
        // assert_eq!(matrix.bandwidth(), 189331);

        let file = "test1.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 2);
        assert_eq!(matrix.degrees(), [1, 1, 1, 1]);
        matrix.cmr(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);
        // TODO: insert new matrix to assert
        
        let file = "test2.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 2);
        println!("{:?}", matrix);
        assert_eq!(matrix.degrees(), [2, 2, 3, 1]);
        matrix.cmr(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);

        let file = "test3.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 3);
        assert_eq!(matrix.degrees(), [3, 3, 2, 4]);
        matrix.cmr(matrix.col_index[0]);
        assert_eq!(matrix.bandwidth(), 2);

        let file = "bcspwr01.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 38);
        matrix.cmr(matrix.col_index[0]);
        // assert_eq!(matrix.bandwidth(), 8);
        // CMr 8

        let file = "lns__131.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 111);
        matrix.cmr(matrix.col_index[0]);
        // CMr 39

        let file = "mcca.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 65);
        matrix.cmr(matrix.col_index[0]);
        // CMr 3

        let file = "will199.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 169);
        matrix.cmr(matrix.col_index[0]);
        // CMr 115

        let file = "662_bus.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 335);
        matrix.cmr(matrix.col_index[0]);
        // CMr 112

        let file = "dwt__361.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 50);
        matrix.cmr(matrix.col_index[0]);
        // CMr 25

        let file = "sherman4.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 368);
        matrix.cmr(matrix.col_index[0]);
        // CMr 0??
    }

}