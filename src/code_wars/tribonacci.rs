pub fn seq(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut vec = signature.iter().map(|x| *x).collect::<Vec<f64>>();
    let mut sig_sorted = [0f64; 3];
    sig_sorted.clone_from_slice(&signature[0..]);

    if n > 0 && *signature == sig_sorted {
        // These two are special cases
        if n == 1 {
            vec.pop();
            vec.pop();
        } else if n == 2 {
            vec.pop();
        } else {
            for i in 3..n {
                vec.push(vec[i - 3] + vec[i - 2] + vec[i - 1]);
            }
        }
    } else {
        vec.clear();
    }

    vec
}
