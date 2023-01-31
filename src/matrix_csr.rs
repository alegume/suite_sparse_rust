use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Matrix {
    /* ROW_INDEX[j] is the total number of nonzeros above row j.
    Each (row_index[n+1] - row_index[n]) represent a row
    */
    pub v:Vec<f64>, // non zeros values
    pub col_index:Vec<usize>, // column indices of values in v
    pub row_index:Vec<usize>, // indices (in v and row_index) where the rows starts
    m: usize,
    n: usize,
}

#[derive(Debug, PartialEq)]
pub struct Element {
    v: Option<f64>,
    i: usize,
    j: usize,
}

impl Matrix {
    pub fn new(v_size:usize, row_size:usize, col_size:usize, n:usize, m:usize) -> Self {
        Self {
            v: Vec::with_capacity(v_size),
            row_index: Vec::with_capacity(row_size),
            col_index: Vec::with_capacity(col_size),
            n,
            m
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
        let start = self.row_index[n] as usize;
        let stop = self.row_index[n + 1] as usize;
        let row = &self.col_index[start..stop];
        row
    }

    fn get_values_of_row(&self, n:usize) -> &[f64] {
        let start = self.row_index[n] as usize;
        let stop = self.row_index[n + 1] as usize;
        let row = &self.v[start..stop];
        row
    }

    fn degrees(&self) -> Vec<usize> {
        let mut degrees: Vec<usize> = vec![0; std::cmp::max(self.m, self.n)];
        for i in 1..self.row_index.len() - 1 {
            degrees[i] = self.row_index[i] - self.row_index[i-1];
        }
        degrees
    }

    fn degree(&self, i:usize) -> usize {
        if i < self.row_index.len() - 1 {
            self.row_index[i+1] - self.row_index[i]
        } else { 0 }
    }

    // TODO: find pseudoperipheral vertex with GL algo
    pub fn cmr(&mut self) {
        // lines_visited = old_col (old columns to new ones)
        let mut lines_visited:Vec<usize> = vec![std::usize::MAX; self.m];
        // push_back to add to the queue and pop_front to remove from the queue.
        let mut to_visit: VecDeque<usize> = VecDeque::from([self.col_index[0]]);
        let last_row = self.row_index.len() - 1; // TODO:: usar M
        let mut n:usize = std::cmp::max(self.m, self.n); 

        for i in 0..last_row {
            if lines_visited[i] == std::usize::MAX {
                to_visit.push_back(i);
                self.cycle_throw_queue(&mut to_visit, &mut lines_visited, last_row, &mut n);
            }
        }
        // dbg!(&lines_visited);

        self.reorder(&lines_visited);
    }

    fn cycle_throw_queue(&self, to_visit:&mut VecDeque<usize>, lines_visited:&mut Vec<usize>, last_row:usize, n: &mut usize) {
        while let Some(i) = to_visit.pop_front() {
            if lines_visited[i] == std::usize::MAX { 
                let row = self.get_columns_of_row(i); // get row of i (neighbours of i)
                let mut row2 = row.to_vec(); // Make a copy
                // Sort by degree
                row2.sort_by(|a, b| {
                    self.degree(*a).cmp(&self.degree(*b))
                });
                for j in row2 {
                    // If it's the last column ptr it's invalid
                     if j >= last_row {
                        if lines_visited[j] == std::usize::MAX {
                            *n -= 1;
                            lines_visited[j] = *n;
                        }
                    } else if lines_visited[j] == std::usize::MAX {
                        to_visit.push_back(j);
                    }
                }
                *n -= 1;
                lines_visited[i] = *n;
            }
        }
    }


    fn reorder(&mut self, new_rows: &Vec<usize>) {
        // let mut v = vec![0f64; self.v.len()];
        let n = std::cmp::max(self.m, self.n); // TODO: revisar
        let mut row_offset = Vec::with_capacity(n);
        let mut col_index = Vec::with_capacity(n);
        let mut v:Vec<f64> = Vec::with_capacity(self.v.len());

        let mut old_rows:Vec<usize> = vec![0; self.m];
        for (i, x) in new_rows.iter().enumerate() {
            old_rows[*x] = i;
        }

        row_offset.push(0);
        for new in old_rows.iter() {
            // Change col_offsets 
            let start = col_index.len();
            let old_cols = self.get_columns_of_row(*new);
            for e in old_cols {
                col_index.push(new_rows[*e]); // TODO: Verify optimization
            }
            //  Change V's if its the case
            if self.v.len() > 0 {
                let values = self.get_values_of_row(*new);
                // dbg!(values);
                let mut v_slc:Vec<(&usize,&f64)> = col_index[start..].iter().zip(values.iter()).collect();
                // println!("{:?}", v_slc);
                v_slc.sort_by_key(|e| e.0);
                // println!("{:?}\n", v_slc);
                for (_, value) in v_slc {
                    v.push(*value);
                }
            } else {
                col_index[start..].sort(); // Sort last part by columns
            }

            // Calculate row offset (size of old row)
            row_offset.push(col_index.len());
        }
        // Change matrix
        self.v = v;
        self.col_index = col_index;
        self.row_index = row_offset;
    }

}


pub fn mm_file_to_csr(file: &str) -> Matrix {
    let mut coordinates: Vec<Element>;
    let (n, m): (usize, usize); 
    (coordinates, n, m) = read_matrix_market_file(file);
    let len_v: usize;
    if let Some(_) = coordinates[0].v {
        len_v = coordinates.len();
    } else { len_v = 0 }
    let mut matrix = Matrix::new(len_v, coordinates.len(),coordinates.len(), n, m);

    // Sort in regard of i and then j
    coordinates.sort_by_key(|e| (e.i, e.j) );

    // row_index always starts the first line
    matrix.row_index.push(coordinates[0].i);
    for el in &coordinates {
        if let Some(v) = el.v { matrix.v.push(v); }
        matrix.col_index.push(el.j);
        // // println!("i:{:?}, j:{:?}, lr:{:?}, col.len{:?}, ", el.i, el.j, matrix.row_index.len(), matrix.col_index.len());
        // Each (row_index[n+1] - row_index[n]) represent a row
        if el.i > matrix.row_index.len() - 1 {
            matrix.row_index.push(matrix.col_index.len() - 1);
        }
    } 
    //he last element is NNZ , i.e., the fictitious index in V immediately after the last valid index NNZ - 1
    matrix.row_index.push(coordinates.len());

    matrix
}

pub fn read_matrix_market_file(filename: &str) -> (Vec<Element>, usize, usize) {
    // Indices are 1-based, i.e. A(1,1) is the first element.
    use std::fs;
    use std::io::{BufRead, BufReader};

    let filename = "instances/".to_owned() + filename;
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut header:bool = false;
    let mut nz_len:usize = 0;
    let mut coordinates = Vec::<Element>::new();
    let (mut n, mut m): (usize, usize)= (0, 0); 
    
    for line in reader.lines() {
        // Format => I1  J1  M(I1, J1)
        let line = line.unwrap();
        if line.starts_with("%") { continue; }
        let mut text = line.splitn(3, ' ');
    
        let i:&str = text.next().unwrap().trim();
        let j:&str = text.next().unwrap().trim();
        // Reading V
        if let Some(v) = text.next() {
            if !header { // first line of file => (rows, columns, entries)
                nz_len = v.trim().parse().expect("Error reading first line of file.mtx");
                header = true;
                m = i.parse::<usize>().unwrap();
                n = j.parse::<usize>().unwrap();
                // assert_eq!(i, j);
                continue;
            }
            if let Ok(v) = v.trim().parse() {
                // 1-based indices (-1)
                let el = Element{
                    i: i.parse::<usize>().unwrap() - 1,
                    j: j.parse::<usize>().unwrap() - 1,
                    v: Some(v),
                };
                coordinates.push(el);
            } else { panic!("Can't catch v value ({v});"); }
        } else { // Coordinate matrix only (don't have V's)
            let el = Element{
                i: i.parse::<usize>().unwrap() - 1,
                j: j.parse::<usize>().unwrap() - 1,
                v: None,
            };
            coordinates.push(el);
        }
    }
    assert_eq!(coordinates.len(), nz_len);
    (coordinates, n, m)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mm_file_to_csr_test() {
        let file = "test1.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [5.0, 8.0, 3.0, 6.0]);
        assert_eq!(matrix.col_index, [0, 1, 2, 1]);
        assert_eq!(matrix.row_index, [0, 1, 2, 3, 4]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 3);

        let file = "test2.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
        assert_eq!(matrix.col_index, [0, 1, 1, 3, 2, 3, 4, 5]);
        assert_eq!(matrix.row_index, [0, 2, 4, 7, 8]);
        assert_eq!(matrix.m, 4);
        assert_eq!(matrix.n, 6);
    }

    #[test]
    fn read_matrix_market_file_test() {
        let file = "test1.mtx";
        let (coordinates, n, m) = read_matrix_market_file(file);
        let coo = vec![
            Element{
                v: Some(5.0),
                i: 0,
                j: 0,
            },
            Element{
                v: Some(8.0),
                i: 1,
                j: 1,
            },
            Element{
                v: Some(3.0),
                i: 2,
                j: 2,
            },
            Element{
                v: Some(6.0),
                i: 3,
                j: 1,
            },
        ];
        let mut it = coordinates.iter();
        for el in coo.iter() {
            assert_eq!(Some(el), it.next());
        }
        assert_eq!(coordinates.len(), coo.len());
        assert_eq!(m, 4);
        assert_eq!(n, 3);

        let file = "test2.mtx";
        let (coordinates, n, m) = read_matrix_market_file(file);
        // println!("coordinates:{:?}", coordinates);
        let coo = vec![
            Element{
                v: Some(10.0),
                i: 0,
                j: 0,
            },
            Element{
                v: Some(20.0),
                i: 0,
                j: 1,
            },
            Element{
                v: Some(30.0),
                i: 1,
                j: 1,
            },
            Element{
                v: Some(40.0),
                i: 1,
                j: 3,
            },
            Element{
                v: Some(60.0),
                i: 2,
                j: 3,
            },
            Element{
                v: Some(80.0),
                i: 3,
                j: 5,
            },
            Element{
                v: Some(70.0),
                i: 2,
                j: 4,
            },
            Element{
                v: Some(50.0),
                i: 2,
                j: 2,
            },
        ];
        let mut it = coordinates.iter();
        for el in coo.iter() {
            assert_eq!(Some(el), it.next());
        }
        assert_eq!(coordinates.len(), coo.len());
        assert_eq!(m, 4);
        assert_eq!(n, 6);
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
        matrix.cmr();
        assert_eq!(matrix.bandwidth(), 2);
        assert_eq!(matrix.degrees(), [1, 1, 1, 1]);

        let file = "test2.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 2);
        matrix.cmr();
        assert_eq!(matrix.bandwidth(), 2);
        assert_eq!(matrix.degrees(), [2, 2, 3, 1]);

        let file = "bcspwr01.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 38);
        matrix.cmr();
        // assert_eq!(matrix.bandwidth(), 8);
        // CMr 8

        let file = "lns__131.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 111);
        // CMr 39

        let file = "mcca.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 65);
        // CMr 3

        let file = "will199.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 169);
        // CMr 115

        let file = "662_bus.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 335);
        // CMr 112

        let file = "dwt__361.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 50);
        // CMr 25

        let file = "sherman4.mtx";
        let mut matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 368);
        // CMr 0??
    }
}