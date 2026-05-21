fn main() {
    // m1 ∈ m x n
    // m2 ∈ n x l
    println!("Hello, world!");
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
