use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mut first: i32 = 0;
    let mut rest: i32 = 0;

    thread::scope(|scope| {
        scope.spawn(|| first = v[..v.len() / 2].iter().sum::<i32>());
        scope.spawn(|| rest = v[v.len() / 2..].iter().sum::<i32>());
    });

    first + rest
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
