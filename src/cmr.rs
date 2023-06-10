use std::cmp::max;
// use std::collections::HashMap;
use std::collections::VecDeque;

use crate::matrix_csr::Matrix;

impl Matrix {

    pub fn cmr_labels(&mut self, v: usize) {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.m];
        let mut n:usize = 0;
        let mut v = v;
        // let mut distances = vec![0; self.m];

        self.labels = vec![999;self.m];
        
        loop {
            queue.push_back(v);
            visited[v] = true;
            while let Some(v) = queue.pop_front() {
                // println!("{} => {:?}", v, self.get_columns_of_row(v));
                // let x = ;
                let mut neighbours =  self.get_columns_of_row(v).to_vec();
                for u in neighbours {
                    if !visited[u] {
                        queue.push_back(u);
                        visited[u] = true;
                    }
                }
                self.labels[v] = n;
                n += 1;
            }
            if let Some(u) = visited.iter().position(|&x| x == false) { 
                v = u;
            } else {
                break;
            }
        }
        self.labels.reverse();
        println!("{:?}", self.labels);
    }

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
                row2.sort_by(|a, b| self.degree(*a).cmp(&self.degree(*b)));
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
