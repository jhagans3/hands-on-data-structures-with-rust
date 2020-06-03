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

// right O(n ln(n))
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    println!("Merge sort: {:?}", v);
    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

// Move the 1st element to the correct place
// Everything smaller should be before
// else should be after
// output is the pivot's location
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            // move our pivot forward 1, and put this element before it
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
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
    #[test]
    fn test_merge_sort() {
        // cargo test test_merge_sort -- --nocapture
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(v);

        let sorted_v = vec![1, 3, 4, 6, 8, 11, 13];
        assert_eq!(v, sorted_v);
    }
    #[test]
    fn test_pivot() {
        // cargo test test_pivot -- --nocapture
        let mut v = vec![4, 6, 1, 19, 8, 11, 13, 3];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p))
        }

        // let sorted_v = vec![1, 3, 4, 6, 19, 8, 11, 13];
        // assert_eq!(v, sorted_v);
    }
    #[test]
    fn test_quick_sort() {
        // cargo test test_quick_sort -- --nocapture
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);

        let sorted_v = vec![1, 3, 4, 6, 8, 11, 13];
        assert_eq!(v, sorted_v);
    }
    #[test]
    fn test_sorted_quick_sort() {
        // cargo test test_sorted_quick_sort -- --nocapture
        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        quick_sort(&mut v);

        let sorted_v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        assert_eq!(v, sorted_v);
    }
}
