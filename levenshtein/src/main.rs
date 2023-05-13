fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let mut matrix = vec![vec![0; b_chars.len() + 1]; a_chars.len() + 1];

    for i in 0..a_chars.len() + 1 {
        matrix[i][0] = i;
    }

    for j in 0..b_chars.len() + 1 {
        matrix[0][j] = j;
    }

    for i in 1..a_chars.len() + 1 {
        for j in 1..b_chars.len() + 1 {
            let indicator = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[i][j] = *[
                matrix[i - 1][j] + 1,
                matrix[i][j - 1] + 1,
                matrix[i - 1][j - 1] + indicator,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    // for i in 0..a_chars.len() + 1 {
    //     for j in 0..b_chars.len() + 1 {
    //         print!("{:?}", matrix[i][j]);
    //     }
    //     println!();
    // }

    matrix[a_chars.len()][b_chars.len()]
}

fn main() {
    let a = "Valentine";
    let b = "Galetin";

    println!(
        "Levenshtein distance between {} and {} is {}",
        a,
        b,
        levenshtein_distance(a, b)
    );
}
