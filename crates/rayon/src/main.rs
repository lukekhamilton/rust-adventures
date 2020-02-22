// ref: https://www.youtube.com/watch?v=gof_OEv71Aw
use rayon::prelude::*;

fn main() {
    println!("Hello, world!");
}

fn sum_of_squares(input: &[i32]) -> i32 {
    input.iter().map(|&i| i * i).sum()
}

fn sum_of_squares_with_rayon(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum()
}

fn increment_all(counts: &mut [u32]) {
    for c in counts.iter_mut() {
        *c += 1;
    }
}

fn increment_all_with_rayon(counts: &mut [u32]) {
    counts.par_iter_mut().for_each(|c| *c += 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_all() {
        let mut numbers: [u32; 5] = [1, 2, 3, 4, 5];
        increment_all(&mut numbers);
        assert_eq!(numbers, [2, 3, 4, 5, 6])
    }

    #[test]
    fn test_increment_all_with_rayon() {
        let mut numbers: [u32; 5] = [1, 2, 3, 4, 5];
        increment_all_with_rayon(&mut numbers);
        assert_eq!(numbers, [2, 3, 4, 5, 6])
    }

    #[test]
    fn test_sum_of_squares_1() {
        let i = [1];
        let res = sum_of_squares(&i);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_sum_of_squares_2() {
        let i = [1, 2];
        let res = sum_of_squares(&i);
        assert_eq!(res, 5);
    }

    #[test]
    fn test_sum_of_squares_with_rayon_1() {
        let i = [1];
        let res = sum_of_squares_with_rayon(&i);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_sum_of_squares_with_rayon_2() {
        let i = [1, 2];
        let res = sum_of_squares_with_rayon(&i);
        assert_eq!(res, 5);
    }
}
