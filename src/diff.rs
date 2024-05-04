pub fn longest_common_subsequence(prev: String, cur: String) -> String {
    let m: usize = prev.len();
    let n: usize = cur.len();

    let prev_chars: Vec<u8> = prev.as_bytes().to_vec();
    let cur_chars: Vec<u8> = cur.as_bytes().to_vec();

    let mut lcs_table = vec![vec![0 ; n + 1]; m + 1];
    for i in 1..(m + 1) {
        for j in 1..(n + 1) {
            if prev_chars[i - 1] == cur_chars[j - 1] {
                lcs_table[i][j] = lcs_table[i - 1][j - 1] + 1;
                continue;
            }
            if lcs_table[i - 1][j] >= lcs_table[i][j - 1] {
                lcs_table[i][j] = lcs_table[i - 1][j];
                continue;
            }
            lcs_table[i][j] = lcs_table[i][j - 1];
        }
    }

    let mut index: usize = lcs_table[m][n];
    let mut longest: Vec<u8> = vec![0; index];

    let mut i = m;
    let mut j = n;
    while i > 0 && j > 0 {
        if prev_chars[i - 1] != cur_chars[j - 1] {
            match lcs_table[i - 1][j] > lcs_table[i][j - 1] {
                true => i -= 1,
                false => j -= 1
            }
            continue;
        }
        longest[index - 1] = prev_chars[i - 1];
        index -= 1;
        i -= 1; j -= 1;
    }
    
    return String::from_utf8(longest.clone()).unwrap();
}