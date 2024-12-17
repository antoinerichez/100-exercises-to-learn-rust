use std::ops::Add;
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let static_vec: &'static mut [i32] = v.leak();

    let half = static_vec.len() / 2;
    let (first, rest) = static_vec.split_at(half);

    let first_thread = thread::spawn(|| first.iter().sum::<i32>());
    let rest_thread = thread::spawn(|| rest.iter().sum::<i32>());

    first_thread.join().unwrap().add(rest_thread.join().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
