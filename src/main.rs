use regex::Regex;
use std::fs;

fn main() {
    // m1 ∈ m x n
    // m2 ∈ n x l
    let path_m1 = String::from(".output/matrix_mult_MinDim.CAT_1/test_case_0/m1_1_7.txt");
    let m1 = read_matrix_from_txt(path_m1).unwrap();
    let path_m2 = String::from(".output/matrix_mult_MinDim.CAT_1/test_case_0/m2_7_4.txt");
    let m2 = read_matrix_from_txt(path_m2).unwrap();

    let path_expected = String::from(".output/matrix_mult_MinDim.CAT_1/test_case_0/res_1_4.txt");
    let expected = read_matrix_from_txt(path_expected).unwrap();

    let result = matrix_loop_v1(&m1, &m2);
    assert_eq!(result, expected);
}

fn matrix_loop_v1(m1: &Matrix, m2: &Matrix) -> Matrix {
    let mut values: Vec<f32> = vec![0.0; m1.m * m2.n];
    for i in 0..m1.m {
        for j in 0..m2.n {
            for k in 0..m1.n {
                values[i * m2.n + j] += m1.values[i * m1.n + k] * m2.values[k * m2.n + j];
            }
        }
    }
    Matrix {
        m: m1.m,
        n: m2.n,
        values,
    }
}

#[derive(Debug)]
struct Matrix {
    m: usize,
    n: usize,
    values: Vec<f32>,
}

impl PartialEq for Matrix {
    // Comparing two matrices
    fn eq(&self, other: &Self) -> bool {
        const EPS: f32 = 0.0001;
        let cond1 = self.m == other.m && self.n == other.n;
        if !cond1 {
            return cond1;
        }

        let cond2 = self
            .values
            .iter()
            .zip(other.values.iter())
            .fold(true, |prev, (x, y)| prev && (x - y).abs() < EPS);

        cond1 && cond2
    }
}

fn read_matrix_from_txt(file_path: String) -> Option<Matrix> {
    // First get matrix dimensions
    let re = Regex::new(r"_(?<m>[0-9]+)_(?<n>[0-9]+).txt").unwrap();
    let Some(caps) = re.captures(&file_path) else {
        println!("Could not read file!");
        return None;
    };

    let m: usize = caps["m"].parse().unwrap();
    let n: usize = caps["n"].parse().unwrap();

    // Now get actual matrix entries
    let contents =
        fs::read_to_string(&file_path).expect(&format!("Could not read file: {}", file_path));

    let mut values = Vec::with_capacity(m * n);
    for str_number in contents.split_whitespace() {
        let number: f32 = str_number.trim().parse().unwrap();
        values.push(number);
    }

    Some(Matrix { m, n, values })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_on_my_machine() {
        let m1 = Matrix {
            m: 2,
            n: 2,
            values: vec![1.0, 0.0, 1.0, 0.0],
        };
        let m2 = Matrix {
            m: 2,
            n: 2,
            values: vec![2.0, 2.0, 2.0, 2.0],
        };

        let result = matrix_loop_v1(&m1, &m2);
        assert_eq!(result, m2);
    }

    #[test]
    fn yolo() {
        let m1 = Matrix {
            m: 1,
            n: 4,
            values: vec![3.0, 0.0, 1.0, 0.0],
        };
        let m2 = Matrix {
            m: 4,
            n: 1,
            values: vec![2.0, 2.0, 2.0, 2.0],
        };
        let expected = Matrix {
            m: 1,
            n: 1,
            values: vec![8.0],
        };

        let result = matrix_loop_v1(&m1, &m2);
        assert_eq!(result, expected);
    }

    #[test]
    fn ciao() {
        let m1 = Matrix {
            m: 4,
            n: 1,
            values: vec![3.0, 0.0, 1.0, 0.0],
        };
        let m2 = Matrix {
            m: 1,
            n: 4,
            values: vec![2.0, 2.0, 2.0, 2.0],
        };
        let expected = Matrix {
            m: 4,
            n: 4,
            values: vec![
                6.0, 6.0, 6.0, 6.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 0.0, 0.0, 0.0, 0.0,
            ],
        };

        let result = matrix_loop_v1(&m1, &m2);
        assert_eq!(result, expected);
    }
}
