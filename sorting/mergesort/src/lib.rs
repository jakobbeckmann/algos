//! Implementation of merge sort algorithm. Merge sort has O(n lg n) time complexity.

/// Sort a generic vector in place using mergesort.
///
/// ## Example
///
/// ```rust
/// use mergesort;
/// let mut vec = vec![1, 5, 4, 3, 2, 0, 1, -10];
/// mergesort::sort(&mut vec);
/// assert_eq!(vec, vec![-10, 0, 1, 1, 2, 3, 4, 5]);
/// ```
pub fn sort<T>(vec: &mut Vec<T>) where T: PartialOrd + Clone {
    let length = vec.len();
    let mut copy = vec.to_vec();
    split_merge_sort(&mut copy, vec, 0, length);
}

fn split_merge_sort<T>(a: &mut Vec<T>, b: &mut Vec<T>, begin: usize, end: usize)
        where T: PartialOrd + Clone {
    if end - begin < 2 {
        return;
    }

    let middle = (begin + end) / 2;

    split_merge_sort(b, a, begin, middle);
    split_merge_sort(b, a, middle, end);
    top_down_merge(a, b, begin, middle, end);
}

fn top_down_merge<T>(a: &mut Vec<T>, b: &mut Vec<T>, begin: usize, middle: usize, end: usize)
        where T: PartialOrd + Clone {
    let mut i = begin;
    let mut j = middle;

    // While there are elements in the left or right runs...
    for k in begin..end {
        // If left run head exists and is <= existing right run head.
        if i < middle && (j >= end || a[i] <= a[j]) {
            b[k] = a[i].clone();
            i += 1;
        } else {
            b[k] = a[j].clone();
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mergesort_ints() {
        let mut vec = vec![1, 5, 4, 3, 2, 0];
        sort(&mut vec);
        assert_eq!(vec, vec![0, 1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_mergesort_chars() {
        let mut vec = vec!['b', 'c', 'e', 'a', 'y'];
        sort(&mut vec);
        assert_eq!(vec, vec!['a', 'b', 'c', 'e', 'y']);
    }

    #[test]
    fn test_mergesort_duplicates() {
        let mut vec = vec![1, -1, 1, -1, 1, -1, 1, -1];
        sort(&mut vec);
        assert_eq!(vec, vec![-1, -1, -1, -1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_mergesort_singleton() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_mergesort_empty() {
        let mut vec: Vec<bool> = Vec::new();
        sort(&mut vec);
        assert_eq!(vec, Vec::<bool>::new());
    }
}
