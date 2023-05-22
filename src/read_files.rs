// use crate::matrix_csr::Element;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Element {
    pub v: Option<f64>,
    pub i: usize,
    pub j: usize,
}

pub fn read_matrix_market_file_coordinates(filename: &str) -> (Vec<Element>, usize, usize) {
    // Indices are 1-based, i.e. A(1,1) is the first element.
    use std::fs;
    use std::io::{BufRead, BufReader};

    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut header:bool = false;
    let mut nz_len:usize = 0;
    let mut coordinates = Vec::<Element>::new();
    let (mut m, mut n): (usize, usize)= (0, 0); 
    
    for line in reader.lines() {
        // Format => I1  J1  M(I1, J1)
        let line = line.unwrap();
        if line.starts_with("%") { continue; }
        let mut text = line.splitn(3, ' ');
    
        let i:&str = text.next().unwrap().trim();
        let j:&str = text.next().unwrap().trim();
        // Reading V
        if let Some(v) = text.next() {
            if !header { // first line of file => (rows:m, columns:n, entries)
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
    (coordinates, m, n)
}


mod tests {
    // use super::*;

    #[test]
    fn read_matrix_market_file_test() {
        let file = "./instances/test1.mtx";
        let (coordinates, m, n) = read_matrix_market_file_coordinates(file);
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

        let file = "./instances/test2.mtx";
        let (coordinates, m, n) = read_matrix_market_file_coordinates(file);
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

        let file = "./instances/test3.mtx";
        let (coordinates, m, n) = read_matrix_market_file_coordinates(file);
        let coo = vec![
            Element{
                v: Some(2.0),
                i: 0,
                j: 0,
            },
            Element{
                v: Some(3.0),
                i: 0,
                j: 1,
            },
            Element{
                v: Some(1.0),
                i: 0,
                j: 3,
            },
            Element{
                v: Some(3.0),
                i: 1,
                j: 0,
            },
            Element{
                v: Some(2.0),
                i: 1,
                j: 1,
            },
            Element{
                v: Some(5.0),
                i: 1,
                j: 3,
            },
            Element{
                v: Some(2.0),
                i: 2,
                j: 2,
            },
            Element{
                v: Some(4.0),
                i: 2,
                j: 3,
            },
            Element{
                v: Some(1.0),
                i: 3,
                j: 0,
            },
            Element{
                v: Some(5.0),
                i: 3,
                j: 1,
            },
            Element{
                v: Some(4.0),
                i: 3,
                j: 2,
            },
            Element{
                v: Some(2.0),
                i: 3,
                j: 3,
            },
        ];
        let mut it = coordinates.iter();
        for el in coo.iter() {
            assert_eq!(Some(el), it.next());
        }
        assert_eq!(coordinates.len(), coo.len());
        assert_eq!(m, 4);
        assert_eq!(n, 4);
    }
}