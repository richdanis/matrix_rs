use regex::Regex;
use std::fs;

fn main() {
    // m1 ∈ m x n
    // m2 ∈ n x l
    println!("Hello, world!");

    let file_path = String::from(".output/matrix_mult_MinDim.CAT_1/test_case_0/m1_1_7.txt");
    let m1 = read_matrix_from_txt(file_path).unwrap();
    println!("{:?}", m1.m);
    println!("{:?}", m1.n);
    println!("{:?}", m1.values);
}

fn matrix_loop_v1(m1: &[f32], m2: &[f32], m: usize, n: usize, l: usize) -> Vec<f32> {
    let mut res: Vec<f32> = vec![0.0; m * l];
    for i in 0..m {
        for j in 0..l {
            for k in 0..n {
                res[i * l + j] += m1[i * n + k] * m2[k * l + j];
            }
        }
    }
    res
}

struct Matrix {
    m: usize,
    n: usize,
    values: Vec<f32>,
}

fn read_matrix_from_txt(file_path: String) -> Option<Matrix> {
    let re = Regex::new(r"_(?<m>[0-9]+)_(?<l>[0-9]+).txt").unwrap();
    let Some(caps) = re.captures(&file_path) else {
        println!("Could not read file!");
        return None;
    };
    let m: usize = caps["m"].parse().unwrap();
    let n: usize = caps["m"].parse().unwrap();
    println!("{}", &caps["m"]);
    // let values = Vec::new();
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
        let m1 = vec![1.0, 0.0, 1.0, 0.0];
        let m2 = vec![2.0, 2.0, 2.0, 2.0];

        let result = matrix_loop_v1(&m1, &m2, 2, 2, 2);
        assert_eq!(result, m2);
    }

    #[test]
    fn yolo() {
        let m1 = vec![3.0, 0.0, 1.0, 0.0];
        let m2 = vec![2.0, 2.0, 2.0, 2.0];

        let result = matrix_loop_v1(&m1, &m2, 1, 4, 1);
        assert_eq!(result, vec![8.0]);
    }

    #[test]
    fn ciao() {
        let m1 = vec![3.0, 0.0, 1.0, 0.0];
        let m2 = vec![2.0, 2.0, 2.0, 2.0];

        let result = matrix_loop_v1(&m1, &m2, 4, 1, 4);
        assert_eq!(
            result,
            vec![
                6.0, 6.0, 6.0, 6.0, 0.0, 0.0, 0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 0.0, 0.0, 0.0, 0.0
            ]
        );
    }
}
