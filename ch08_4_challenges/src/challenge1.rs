use std::collections::HashMap;

/// Finds the median of a vector and returns an Option<f32>
/// if vector is empty, Option<None> is returned
pub fn median(v: &[i32]) -> Option<f32> {
    let length = v.len();
    if length == 0 {
        None
    } else {
        let mut sorted_v = v.to_vec();
        sorted_v.sort();

        let middle = length / 2;

        if length % 2 == 1 {
            Some(sorted_v[middle] as f32)
        } else {
            let left = sorted_v[middle - 1] as f32;
            let right = sorted_v[middle] as f32;
            Some(0.5 * (left + right))
        }
    }
}

pub fn mode(v: &[i32]) -> Option<i32> {
    if v.is_empty() {
        return None;
    }

    let mut map = HashMap::new();

    for &value in v {
        let count = map.entry(value).or_insert(0);
        *count += 1;
    }

    map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
}
