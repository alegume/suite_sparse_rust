use std::cmp::max;
// use std::collections::HashMap;
use std::collections::VecDeque;

use crate::matrix_csr::Matrix;

impl Matrix {
    // CMr that do not change the graph, only labels
    // TODO: DO NOT CONSIDER PREVIOUS LABELS
    pub fn cmr_labels(&mut self, v: usize) {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.m];
        let mut order = vec![0; self.m];
        let mut n: usize = 0;
        let mut v = v;

        loop {
            queue.push_back(v);
            visited[v] = true;
            while let Some(v) = queue.pop_front() {
                let mut neighbours = self.get_columns_of_row(v).to_vec();
                // Sort by degree
                neighbours.sort_by_key(|&x| self.degree(x));
                for u in neighbours {
                    if !visited[u] {
                        queue.push_back(u);
                        visited[u] = true;
                    }
                }
                order[n] = v;
                n += 1;
            }
            // Find disconected vertices
            if let Some(u) = visited.iter().position(|&x| !x) {
                v = u;
            } else {
                break;
            }
        }
        // Labeling in reverse order
        let mut label = self.m;
        for i in order {
            label -= 1;
            self.labels[i] = label;
        }
    }

    /// LEGACY CODE FROM HERE!!!

    // CMr by reordering and changing the graph
    pub fn cmr_reorder(&mut self, start_v: usize) -> Vec<usize> {
        let mut lines_visited: Vec<usize> = vec![std::usize::MAX; max(self.m, self.n)];
        // push_back to add to the queue and pop_front to remove from the queue.
        let mut to_visit: VecDeque<usize> = VecDeque::from([start_v]);
        let mut n: usize = std::cmp::max(self.m, self.n);

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
    fn cycle_through_queue_bfs(
        &self,
        to_visit: &mut VecDeque<usize>,
        lines_visited: &mut [usize],
        n: &mut usize,
    ) {
        while let Some(i) = to_visit.pop_front() {
            if lines_visited[i] == std::usize::MAX {
                let row = self.get_columns_of_row(i); // Get row of i (neighbours of i)
                let mut row2 = row.to_vec(); // Make a copy
                                             // Sort by degree
                row2.sort_by_key(|&x| self.degree(x));
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix_csr::mm_file_to_csr;
    use crate::read_files::{
        read_matrix_market_file_coordinates, read_matrix_market_file_coordinates_no_values, Element,
    };

    #[test]
    fn cmr_labels_test() {
        let file = "./input/tests/test4-ipo.mtx";
        let mut matrix = mm_file_to_csr(file, true);
        let mut matrix2 = matrix.clone();
        matrix.cmr_labels(0);
        assert_eq!(matrix.bandwidth(), 3);
        assert_eq!(matrix.labels, vec![5, 0, 4, 2, 1, 3]);
        matrix2.cmr_labels(3);
        assert_eq!(matrix2.bandwidth(), 2);
        assert_eq!(matrix2.labels, vec![3, 0, 4, 5, 2, 1]);

        let file = "./input/tests/test3.mtx";
        let mut matrix = mm_file_to_csr(file, true);
        let mut matrix2 = matrix.clone();
        let mut matrix3 = matrix.clone();
        matrix.cmr_labels(0);
        assert_eq!(matrix.bandwidth(), 2);
        assert_eq!(matrix.labels, vec![3, 2, 0, 1]);
        matrix2.cmr_labels(2);
        assert_eq!(matrix2.bandwidth(), 2);
        assert_eq!(matrix2.labels, vec![1, 0, 3, 2]);
        matrix3.cmr_labels(3);
        assert_eq!(matrix3.bandwidth(), 3);
        assert_eq!(matrix3.labels, vec![1, 0, 2, 3]);
    }
}
