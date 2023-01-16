#[derive(Debug)]
pub struct Matrix {
    /* ROW_INDEX[j] is the total number of nonzeros above row j.
    I *DON'T* use the last element in row_index as the total number of nonzeros in the matrix. Because I can use row_index.len()*/
    pub v:Vec<f64>, // non zeros values
    pub col_index:Vec<u32>, // column indices of values in v
    pub row_index:Vec<u32>, // indices (in v and row_index) where the rows starts
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            v: Vec::new(),
            col_index: Vec::new(),
            row_index: Vec::new(),
        }
    }

    pub fn bandwidth(&self) -> u32 {
        let mut bandwidth:u32 = 0;
        for (i, j) in self.col_index.iter().zip(self.row_index.iter()) {
            // println!("[{} - {}] = {}", *i, *j, i.abs_diff(*j));
            if i.abs_diff(*j) > bandwidth {
                bandwidth = i.abs_diff(*j);
            }
        }
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

pub fn read_matrix_market(filename: &str) -> Matrix {
    // Indices are 1-based, i.e. A(1,1) is the first element.
    use std::fs;
    use std::io::{BufRead, BufReader};

    let filename = "instances/".to_owned() + filename;
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut matrix = Matrix::new();
    let mut header:bool = false;
    let mut nz_len:usize = 0;
    
    for line in reader.lines() {
        // Format => I1  J1  M(I1, J1)
        let line = line.unwrap();
        if line.starts_with("%") { continue; }
        let mut text = line.splitn(3, ' ');
        let i:&str = text.next().unwrap().trim();
        let j:&str = text.next().unwrap().trim();
        let i:u32 = i.parse::<u32>().unwrap() - 1u32;
        let j:u32 = j.parse::<u32>().unwrap() - 1u32;
        println!("i:{}, j:{}", i, j);
        if let Some(v) = text.next() {
            if !header {
                // first line of file => (rows, columns, entries)
                nz_len = v.trim().parse().expect("Error reading first line of file.mtx");
                // matrix.i_size = Some(i.try_into().unwrap());
                // matrix.j_size = Some(j.try_into().unwrap());
                header = true;
                continue;
            }
            if let Ok(v) = v.trim().parse() {
                matrix.v.push(v);
                // println!("i:{}, j:{}, v:{}", i, j, v);
            } else { panic!("Can't catch v value ({v});"); }
        }
        matrix.row_index.push(i);
        matrix.col_index.push(j);
    }
    assert_eq!(nz_len, matrix.v.len());
    assert_eq!(nz_len, matrix.col_index.len());
    // assert_eq!(matrix.j_size.unwrap(), matrix.j.len());
    matrix
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn import_bw() {
        /* Stress tests  */
        // let file = "apache2.mtx";
        // let matrix = read_matrix_market(file);
        // assert_eq!(matrix.bandwidth(), 65837);
        // let file = "pwtk.mtx";
        // let matrix = read_matrix_market(file);
        // assert_eq!(matrix.bandwidth(), 189331);

        let file = "test1.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 1);

        
        let file = "bcspwr01.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 38);
        // CMr 8

        let file = "lns__131.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 111);
        // CMr 39

        let file = "mcca.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 65);
        // CMr 3

        let file = "will199.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 169);
        // CMr 115

        let file = "662_bus.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 335);
        // CMr 112

        let file = "dwt__361.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 50);
        // CMr 25

        let file = "sherman4.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 368);
        // CMr 0??
    }
}