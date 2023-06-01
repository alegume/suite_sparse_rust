use std::cmp::max;
// use std::collections::HashMap;
use std::collections::VecDeque;

use crate::matrix_csr::Matrix;

impl Matrix {
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
    fn cycle_through_queue_bfs(&self, to_visit:&mut VecDeque<usize>, lines_visited:&mut [usize], n: &mut usize) {
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
    pub fn reorder(&mut self, new_rows: &[usize]) {
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
            if !self.v.is_empty() {
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
        self.v = v;
        self.col_index = col_index;
        self.row_index = row_offset;
    }
}