use std::collections::HashSet;

pub fn count(m: i32, n: i32, traps: &[(i32, i32)]) -> i32 {
    if m < 1 || m < 1 {
        return 0;
    }
    let mut set = HashSet::<i32>::new();
    for &(row, col) in traps {
        if (row > m || col > n) || (1 == row && 1 == col) {
            return 0;
        }
        set.insert(row * (n + 1) + col);
    }
    let mut dp = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];
    //    println!("{:?}", set);
    dp[1][1] = 1;
    for i in 2..=n {
        let one = if set.contains(&(i + 1 + n)) { 0 } else { 1 };
        dp[1][i as usize] = one * dp[1][(i - 1) as usize];
    }
    for j in 2..=m {
        let one = if set.contains(&(j * (n + 1) + 1)) {
            0
        } else {
            1
        };
        dp[j as usize][1] = one * dp[(j - 1) as usize][1];
    }
    //    println!("{:?}", dp);
    for i in 2..=m {
        for j in 2..=n {
            if set.contains(&(i * (n + 1) + j)) {
                dp[i as usize][j as usize] = 0;
            } else {
                dp[i as usize][j as usize] =
                    dp[i as usize][(j - 1) as usize] + dp[(i - 1) as usize][j as usize];
            }
        }
    }
    //    println!("{:?}", dp);
    dp[m as usize][n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_test_01() {
        let traps = [(1, 2)];
        assert_eq!(count(2, 3, &traps), 1);
    }

    #[test]
    fn count_test_02() {
        let traps = [(2, 3)];
        assert_eq!(count(4, 4, &traps), 11);
    }

    #[test]
    fn count_test_03() {
        let traps = [(1, 3)];
        assert_eq!(count(4, 4, &traps), 16);
    }

    #[test]
    fn count_test_04() {
        let traps = [(1, 3), (3, 1)];
        assert_eq!(count(4, 4, &traps), 12);
    }

    #[test]
    fn count_test_05() {
        let traps = [(1, 3), (2, 2)];
        assert_eq!(count(3, 4, &traps), 1);
    }

    #[test]
    fn count_test_06() {
        let traps = [(1, 3), (3, 2)];
        assert_eq!(count(3, 4, &traps), 4);
    }

    #[test]
    fn count_test_07() {
        let traps = [(1, 3), (3, 4)];
        assert_eq!(count(3, 4, &traps), 0);
    }

    #[test]
    fn count_test_08() {
        let traps = [(1, 1)];
        assert_eq!(count(3, 4, &traps), 0);
    }

    #[test]
    fn count_test_09() {
        let traps = [];
        assert_eq!(count(9, 9, &traps), 12870);
    }
}
