pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut sorted_points = points.clone();
    sorted_points.sort_by(|a, b| {
        ((a[0] * a[0] + a[1] * a[1]) as f64)
            .sqrt()
            .partial_cmp(&((b[0] * b[0] + b[1] * b[1]) as f64).sqrt())
            .unwrap()
    });
    let mut k_closest = Vec::new();

    for i in 0..k {
        k_closest.push(sorted_points[i as usize].clone());
    }
    k_closest
}
