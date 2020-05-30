// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge/538/week-5-may-29th-may-31st/3345/
pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut sorted_points = points.clone();
    sorted_points.sort_by(|a, b| {
        ((a[0] * a[0] + a[1] * a[1]) as f64)
            .sqrt()
            .partial_cmp(&((b[0] * b[0] + b[1] * b[1]) as f64).sqrt())
            .unwrap()
    });

    sorted_points[0..k as usize].to_vec()
}
