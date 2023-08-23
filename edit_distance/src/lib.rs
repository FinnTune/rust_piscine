pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    // If one of the strings is empty, the answer is the length of the other string.
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    // Create a table to store results of subproblems.
    // dp[i][j] will be storing the edit distance between the first i characters of source and the first j characters of target.
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize the base cases.
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // Fill in the table using a bottom-up approach.
    for i in 1..=m {
        for j in 1..=n {
            // If the last characters of the two strings are the same, ignore them and get the count for the remaining characters.
            if &source[i - 1..i] == &target[j - 1..j] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                // If they are different, consider all possibilities (insert, delete, replace) and get the minimum.
                dp[i][j] = 1 + std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i][j - 1], dp[i - 1][j]));
            }
        }
    }

    // The value in the bottom-right corner of the table is the answer.
    dp[m][n]
}