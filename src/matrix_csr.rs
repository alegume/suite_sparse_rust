#[derive(Debug)]
pub struct Matrix {
    /* ROW_INDEX[j] is the total number of nonzeros above row j.
    I *DON'T* use the last element in row_index as the total number of nonzeros in the matrix. Because I can use row_index.len()*/
    pub v:Vec<f64>, // non zeros values
    pub col_index:Vec<u32>, // column indices of values in v
    pub row_index:Vec<u32>, // indices (in v and row_index) where the rows starts
}

#[derive(Debug, PartialEq)]
pub struct Element {
    v: Option<f64>,
    i: u32,
    j: u32,
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
        let mut row: usize = 0;

        while row < self.row_index.len() - 1 {
            let start = self.row_index[row] as usize;
            let stop = self.row_index[row + 1] as usize;
            let col_slc = &self.col_index[start..stop];
            println!("{:?}", col_slc);
            println!("slc[{start}..{stop}]");
            for j in col_slc {
                println!("{row} -> {j}");
                if start.abs_diff(*j as usize) > bandwidth {
                    bandwidth = row.abs_diff(*j as usize);
                    println!("banda:{bandwidth}");
                }
            }
            row += 1;
        }
        // for (i, j) in self.col_index.iter().zip(self.row_index.iter()) {
        //     // println!("[{} - {}] = {}", *i, *j, i.abs_diff(*j));
        //     if i.abs_diff(*j) > bandwidth {
        //         bandwidth = i.abs_diff(*j);
        //     }
        // }
        bandwidth
    }

    pub fn cmr(&self) {
        // push_back to add to the queue, and pop_front to remove from the queue.
        use std::collections::VecDeque;
        let mut visiteds: Vec<u32> = Vec::new();
        let mut to_visit: VecDeque<u32> = VecDeque::from([self.row_index[0]]);
        loop {
            let v = to_visit.pop_front();
            match v {
                Some(v) => {
                    if !visiteds.contains(&v) { 
                        for j in &self.row_index { // TODO: optimize storing the index of Vec and beginig in it
                            // Search for elements that are neighbour and have not been visited yet in order of degree
                            println!("{j}");
                            visiteds.push(v);
                        }
                    } else { println!{"visited = {v}"}; continue; }
                },
                None => { println!{"\tEnd of queue"}; break },
            }
        }
    }
}


pub fn mm_file_to_csr(file: &str) -> Matrix {
    let mut coordinates = read_matrix_market_file(file);
    let len_v: usize;
    if let Some(_) = coordinates[0].v {
        len_v = coordinates.len();
    } else { len_v = 0}
    let mut matrix = Matrix::new(len_v, coordinates.len(),coordinates.len());
    let mut last_row: u32 = 0;

    // Sort in regard of i and then j
    // println!("antes {:?}", coordinates);
    coordinates.sort_by_key(|e| (e.i, e.j) );
    // println!("depois {:?}", coordinates);

    // row_index always starts the first line
    // TODO: REvisar
    matrix.row_index.push(coordinates[0].i);
    for el in &coordinates {
        if let Some(v) = el.v { matrix.v.push(v); }
        matrix.col_index.push(el.j);
        if el.i > last_row {
            // println!("i:{:?}, j:{:?}, lr:{:?}, row.len{:?}, ", el.i, el.j, last_row, matrix.col_index.len());
            last_row = el.j;
            matrix.row_index.push(matrix.col_index.len() as u32 - 1u32);
        }
    } 
    //he last element is NNZ , i.e., the fictitious index in V immediately after the last valid index NNZ - 1
    matrix.row_index.push(coordinates.len() as u32);

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
                    i: i.parse::<u32>().unwrap() - 1u32,
                    j: j.parse::<u32>().unwrap() - 1u32,
                    v: Some(v),
                };
                coordinates.push(el);
            } else { panic!("Can't catch v value ({v});"); }
        } else { // Coordinate matrix only (don't have V's)
            let el = Element{
                i: i.parse::<u32>().unwrap() - 1u32,
                j: j.parse::<u32>().unwrap() - 1u32,
                v: None,
            };
            coordinates.push(el);
        }
    }
    assert_eq!(coordinates.len(), nz_len);
    coordinates
        /*
        matrix.col_index.push(j);
        // TODO: Fix row_index 
        // Only insert if new row starts 
        if let Some(&last_row) = matrix.row_index.last() {
            if j > last_row {
                matrix.row_index.push(matrix.row_index.len() as u32);
            } else if j < last_row {
                panic!("I need to ensure entries are provided in a pre-defined (column-oriented) order.")
            } else {
                println!("Jah inseriu. j={j}");
            }
        } else { // Nothing added yet
            println!("primeira inseriu. j={}", matrix.row_index.len());
            matrix.row_index.push(matrix.row_index.len() as u32);
        }
    }
    assert_eq!(nz_len, matrix.v.len());
    assert_eq!(nz_len, matrix.col_index.len());
    // assert_eq!(matrix.j_size.unwrap(), matrix.j.len());
    matrix
    */
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

        let file = "bcspwr01.mtx";
        let matrix = mm_file_to_csr(file);
        assert_eq!(matrix.bandwidth(), 38);
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