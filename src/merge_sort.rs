use core::fmt::Debug;

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    } else {
        // split the thing into a pair
        let left = v.split_off(v.len() / 2);
        return merge(merge_sort(left), merge_sort(v));
    }

    fn merge<T: PartialOrd + Debug>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> {
        let mut result: Vec<T> = Vec::with_capacity(left.len() + right.len());

        while left.len() > 0 && right.len() > 0 {
            if left[0] < right[0] {
                result.push(left.remove(0))
            } else {
                result.push(right.remove(0))
            }
        }

        result.append(&mut left);
        result.append(&mut right);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_sort() {
        let v = vec![9, 6, 3, 16, 25, 2, 1, 6, 1239, 3, 1969, 20, 2];
        let result = merge_sort(v.clone());
        assert_eq!(result, vec![1, 2, 2, 3, 3, 6, 6, 9, 16, 20, 25, 1239, 1969])
    }
}
