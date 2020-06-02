use std::fmt::Debug;

// Big O n^2
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        // optimization check for a sorted list
        let mut sorted = true;

        // optimization after first pass
        // the biggest value bubbles to the end
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
            println!("pass:{},{} {:?}", p, i, v);
        }

        // optimization check for a sorted list
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        // cargo test test_bubble_sort -- --nocapture
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);

        let sorted_v = vec![1, 3, 4, 6, 8, 11, 13];
        assert_eq!(v, sorted_v);
    }
}
