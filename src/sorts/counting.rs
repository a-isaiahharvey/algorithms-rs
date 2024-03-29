use alloc::vec;
use alloc::vec::Vec;

use core::ops::AddAssign;

/// A counting sort implementation for all unsigned types
#[allow(clippy::pedantic)]
pub fn counting<T>(arr: &mut [T])
where
    T: Ord + Into<f64> + From<u8> + AddAssign + Copy,
{
    let max = arr.len();

    let mut occurrences: Vec<usize> = vec![0; max + 1];

    for &data in arr.iter() {
        occurrences[data.into() as usize] += 1;
    }

    // Current index in output array
    let mut i = 0;

    // current data point, necessary to be type-safe
    let mut data = T::from(0);

    // This will iterate from 0 to the largest data point in `arr`
    // `number` contains the occurrences of the data point `data`
    for &number in &occurrences {
        for _ in 0..number {
            arr[i] = data;
            i += 1;
        }

        data += T::from(1);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr = [1, 5, 2, 3, 4, 6, 7, 8, 9, 10];
        counting(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
