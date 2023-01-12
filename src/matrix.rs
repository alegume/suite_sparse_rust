// #[derive(Debug, Clone)]
// pub enum MatrixType {
//     Symmetric,
//     // Asymmetric
// }

#[derive(Debug)]
pub struct Matrix {
    pub i_size: Option<u32>,
    pub j_size: Option<u32>,
    pub v_size: Option<u32>,
    //pub  m_type: MatrixType,
    pub i:Vec<u32>,
    pub j:Vec<u32>,
    pub v:Vec<f64>,
}

impl Matrix {
    /* MNL type of matrix oriented/ordered in regard of N () 
        Yale format ?? 
    */
    pub fn new() -> Self {
        Self {
            i_size: None,
            j_size: None,
            v_size: None,
            // m_type: m_type.clone(),
            i: Vec::new(),
            j: Vec::new(),
            v: Vec::new(),
        }
    }

    pub fn bandwidth(&self) -> u32 {
        let mut bandwidth:u32 = 0;
        for (i, j) in self.i.iter().zip(self.j.iter()) {
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
        let mut to_visit: VecDeque<u32> = VecDeque::from([self.j[0]]);
        loop {
            let v = to_visit.pop_front();
            match v {
                Some(v) => {
                    if !visiteds.contains(&v) { 
                        for j in &self.j { // TODO: optimize storing the index of Vec and beginig in it
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
    use std::fs;
    use std::io::{BufRead, BufReader};

    let filename = "instances/".to_owned() + filename;
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut matrix = Matrix::new();
    let mut header:bool = false;
    
    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("%") { continue; }
        // Header in format M, N, L
        let mut text = line.splitn(3, ' ');
        let i:&str = text.next().unwrap().trim();
        let j:&str = text.next().unwrap().trim();
        let i:u32 = i.parse().unwrap();
        let j:u32 = j.parse().unwrap();
        // println!("i:{}, j:{}", i, j);
        if let Some(v) = text.next() {
            if !header {
                let v:u32 = v.trim().parse().unwrap();
                matrix.i_size = Some(i);
                matrix.j_size = Some(j);
                matrix.v_size = Some(v);
                header = true;
                continue;
            }
            if let Ok(v) = v.trim().parse() {
                matrix.v.push(v);
                // println!("i:{}, j:{}, v:{}", i, j, v);
            } else { panic!("Can't catch v value ({v});"); }
        }
        // assert!(false);
        matrix.i.push(i);
        matrix.j.push(j);
    }
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