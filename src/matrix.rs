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
        if line.starts_with("%") {
            // dbg!("Executing query: {}", line);
            // reader.lines().next();
            continue;
        }
        // Header in format M, N, L
        let mut text = line.splitn(3, ' ');
        let i:&str = text.next().unwrap();
        let j:&str = text.next().unwrap();
        let i:u32 = i.parse().unwrap();
        let j:u32 = j.parse().unwrap();
        let v:f64;
        if let Some(v2) = text.next() {
            v = v2.parse().unwrap();
            // println!("i:{}, j:{}, v:{}", i, j, v);
            if !header {
                matrix.i_size = Some(i);
                matrix.j_size = Some(j);
                matrix.v_size = Some(v as u32);
                header = true;
                continue;
            }
            matrix.v.push(v);
        }
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
        let file = "test1.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 1);

        let file = "cage3.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 4);

        let file = "c-26.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 4204);

        let file = "lp_nug05.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 205);

        let file = "divorce.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 49);

        let file = "mycielskian4.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 8);

        let file = "ch3-3-b1.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 17);

        let file = "mycielskian3.mtx";
        let matrix = read_matrix_market(file);
        assert_eq!(matrix.bandwidth(), 3);
    }
}