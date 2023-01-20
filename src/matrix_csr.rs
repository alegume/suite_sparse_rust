#[derive(Debug)]
pub struct Matrix {
    /* ROW_INDEX[j] is the total number of nonzeros above row j.
    Each (row_index[n+1] - row_index[n]) represent a row
    */
    pub v:Vec<f64>, // non zeros values
    pub col_index:Vec<usize>, // column indices of values in v
    pub row_index:Vec<usize>, // indices (in v and row_index) where the rows starts
}

#[derive(Debug, PartialEq)]
pub struct Element {
    v: Option<f64>,
    i: usize,
    j: usize,
}

impl Matrix {
    pub fn new(v_size:usize, row_size:usize, col_size:usize) -> Self {
        Self {
            v: Vec::with_capacity(v_size),
            row_index: Vec::with_capacity(row_size),
            col_index: Vec::with_capacity(col_size),
        }
    }

    pub fn bandwidth(&self) -> usize {
        let mut bandwidth:usize = 0;
        let mut n_row:usize = 0;

        // Each entry on row_index represents a ROW!
        while n_row < self.row_index.len() - 1 {
            let row = self.get_row(n_row);
            // print!("row{:?}   ", row);
            for j in row { // Columns in a row
                if n_row.abs_diff(*j as usize) > bandwidth {
                    bandwidth = n_row.abs_diff(*j as usize);
                }
            }
            n_row += 1;
        }
        bandwidth
    }

    // Get the nth row of matrix
    fn get_row(&self, n:usize) -> &[usize] {
        let start = self.row_index[n] as usize;
        let stop = self.row_index[n + 1] as usize;
        let row = &self.col_index[start..stop];
        row
    }

    pub fn cmr(&self) {
        // push_back to add to the queue, and pop_front to remove from the queue.
        use std::collections::VecDeque;
        let mut lines_visited: Vec<usize> = Vec::new();
        let mut to_visit: VecDeque<usize> = VecDeque::from([self.col_index[0]]);
        let last_row = self.row_index.len() - 1;
        // let n_row:usize = 0;

        'main_loop: loop {
            if let Some(i) = to_visit.pop_front() {
                if !lines_visited.contains(&i) { 
                    // If it's the last column PTR it's invalid
                    if i >= last_row { 
                        // println!("PARAR! {}", i);
                        continue; 
                    } else {
                        // println!("NAO parou {} -- {}", i, last_row);
                    }
                    // // println!("> {i}");
                    // get row of i (neighbours of i)
                    let row = self.get_row(i);
                    // TODO: sort by degree (number os elements in each row ov i in row)
                    // println!("ROW{:?}", row);
                    for j in row {
                        if !lines_visited.contains(&j) {
                            // println!("\t\tpushou {j} (last{last_row})");
                            to_visit.push_back(*j);
                        } else {
                            // println!("\t\t naopushou");
                        }
                    }
                    lines_visited.push(i);
                } else { 
                    // println!{"visited = {i}"}; 
                    continue;
                }
            } else {  // Empty queue
                // TODO: Otimizar
                // println!{"\tEnd of queue"};
                // Covers the case of disconected graphs
                for j in self.col_index.iter() {
                    if lines_visited.contains(&j) { 
                        // println!("ja contem: {j}");
                        continue; 
                    } else {
                        // println!("NAO  contem: {j}");
                        // Just add if it's not a square matrices (M>N)
                        // Because cols > n_rows it's not reachable anyway
                        if *j >= last_row {
                            lines_visited.push(*j);
                            // println!{"visitou inatingivel {j} - {last_row}"};
                            continue;
                        } else {
                            to_visit.push_back(*j);
                            // println!{"adicionou fila {j} -  - {last_row}"};
                            continue 'main_loop;
                        }
                    }
                }
                break;
            };
        }
        // println!("\torder: {:?} (n={})", lines_visited, lines_visited.len());
        println!("(n={})", lines_visited.len());
    }
}


pub fn mm_file_to_csr(file: &str) -> Matrix {
    let mut coordinates = read_matrix_market_file(file);
    let len_v: usize;
    if let Some(_) = coordinates[0].v {
        len_v = coordinates.len();
    } else { len_v = 0}
    let mut matrix = Matrix::new(len_v, coordinates.len(),coordinates.len());

    // Sort in regard of i and then j
    coordinates.sort_by_key(|e| (e.i, e.j) );

    // row_index always starts the first line
    matrix.row_index.push(coordinates[0].i);
    for el in &coordinates {
        if let Some(v) = el.v { matrix.v.push(v); }
        matrix.col_index.push(el.j);
        // println!("i:{:?}, j:{:?}, lr:{:?}, col.len{:?}, ", el.i, el.j, matrix.row_index.len(), matrix.col_index.len());
        // Each (row_index[n+1] - row_index[n]) represent a row
        if el.i > matrix.row_index.len() - 1 {
            matrix.row_index.push(matrix.col_index.len() - 1);
        }
    } 
    //he last element is NNZ , i.e., the fictitious index in V immediately after the last valid index NNZ - 1
    matrix.row_index.push(coordinates.len());

    matrix
}

pub fn read_matrix_market_file(filename: &str) -> Vec<Element> {
    // Indices are 1-based, i.e. A(1,1) is the first element.
    use std::fs;
    use std::io::{BufRead, BufReader};

    let filename = "instances/".to_owned() + filename;
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut header:bool = false;
    let mut nz_len:usize = 0;
    let mut coordinates = Vec::<Element>::new();
    
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
    coordinates
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

        let file = "test2.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.v, [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0]);
        assert_eq!(matrix.col_index, [0, 1, 1, 3, 2, 3, 4, 5]);
        assert_eq!(matrix.row_index, [0, 2, 4, 7, 8]);
    }

    #[test]
    fn read_matrix_market_file_test() {
        let file = "test1.mtx";
        let coordinates = read_matrix_market_file(file);
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

        let file = "test2.mtx";
        let coordinates = read_matrix_market_file(file);
        println!("coordinates:{:?}", coordinates);
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
    }

    #[test]
    fn bw_test() {
        /* Stress tests  */
        // let file = "apache2.mtx";
        // let matrix = mm_file_to_csr(file);
        // assert_eq!(matrix.bandwidth(), 65837);
        // let file = "pwtk.mtx";
        // let matrix = mm_file_to_csr(file);
        // assert_eq!(matrix.bandwidth(), 189331);

        let file = "test1.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 2);
        matrix.cmr();
        assert_eq!(matrix.bandwidth(), 2);

        let file = "test2.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 2);
        matrix.cmr();
        assert_eq!(matrix.bandwidth(), 2);

        let file = "bcspwr01.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 38);
        matrix.cmr();
        // assert_eq!(matrix.bandwidth(), 8);
        // CMr 8

        let file = "lns__131.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 111);
        // CMr 39

        let file = "mcca.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 65);
        // CMr 3

        let file = "will199.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 169);
        // CMr 115

        let file = "662_bus.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 335);
        // CMr 112

        let file = "dwt__361.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 50);
        // CMr 25

        let file = "sherman4.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 368);
        // CMr 0??
    }
}