use std::fmt::Debug;

mod b_rand;

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
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;

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

pub fn threaded_quick_sort<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    // explicit lifetime required in the type of `v`
    // lifetime `'static` requiredrustc(E0621)
    // lib.rs(106, 61): add explicit lifetime `'static` to the type of `v`

    struct RawSend<T>(*mut [T]); // one element tuple
    unsafe impl<T> Send for RawSend<T> {}

    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);

    // we call join in unsafe
    unsafe {
        let handle = std::thread::spawn(move || {
            threaded_quick_sort(&mut *raw_s.0);
        });

        threaded_quick_sort(&mut b[1..]);

        handle.join().ok();
    }
}

pub fn quick_sort_rayon<T: Send + PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    // puts the 2nd fn on a queue then start the 1st fn
    // if another thread is ready it will steal the 2nd fn
    // this works recursively down the stack
    rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
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

    #[test]
    fn test_threaded_quick_sort() {
        // cargo test test_threaded_quick_sort -- --nocapture
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        threaded_quick_sort(&mut v);

        let sorted_v = vec![1, 3, 4, 6, 8, 11, 13];
        assert_eq!(v, sorted_v);
    }
    #[test]
    fn test_threaded_sorted_quick_sort() {
        // cargo test test_threaded_sorted_quick_sort -- --nocapture
        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        threaded_quick_sort(&mut v);

        let sorted_v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        assert_eq!(v, sorted_v);
    }

    #[test]
    fn test_rayon_quick_sort() {
        // cargo test test_rayon_quick_sort -- --nocapture
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort_rayon(&mut v);

        let sorted_v = vec![1, 3, 4, 6, 8, 11, 13];
        assert_eq!(v, sorted_v);
    }
    #[test]
    fn test_rayon_sorted_quick_sort() {
        // cargo test test_rayon_sorted_quick_sort -- --nocapture
        let mut v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        quick_sort_rayon(&mut v);

        let sorted_v = vec![1, 2, 6, 7, 9, 12, 13, 14];
        assert_eq!(v, sorted_v);
    }
}
