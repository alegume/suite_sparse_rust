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
    pub fn new() -> Self {
        Self {
            i_size: None,
            j_size: None,
            v_size: None,
            // m_type: m_type.clone(),
            i: Vec::with_capacity(218_000),
            j: Vec::with_capacity(218_000),
            v: Vec::with_capacity(5_999_999),
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