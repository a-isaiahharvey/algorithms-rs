fn comp_and_swap<T>(arr: &mut [T], left: usize, right: usize, dir: usize)
where
    T: PartialOrd,
{
    if left >= right {
        return;
    }
    if dir == 1 {
        if arr[left] > arr[right] {
            arr.swap(left, right);
        }
    } else if arr[left] < arr[right] {
        arr.swap(left, right);
    }
}

fn merge<T>(arr: &mut [T], low: usize, length: usize, dir: usize)
where
    T: PartialOrd,
{
    if length > 1 {
        let middle = length / 2;
        for i in low..low + middle {
            comp_and_swap(arr, i, i + middle, dir);
        }

        merge(arr, low, middle, dir);
        merge(arr, low + middle, middle, dir);
    }
}

/// Bitonic sort implementation.
///
/// Note that this program works only when size of input is a power of 2.
/// This function first produces a bitonic sequence by recursively
/// sorting its two halves in opposite sorting orders, and then calls
/// merge to make them in the same order.
///
/// # Arguments
///
/// * `arr` - The array to be sorted
/// * `low` - The starting index of the array
/// * `high` - The ending index of the array
/// * `dir` - The direction of sorting (1 for ascending, 0 for descending)
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Bitonic_sorter)
pub fn bitonic<T>(arr: &mut [T], low: usize, high: usize, dir: usize)
where
    T: PartialOrd,
{
    if high > 1 {
        let middle = high / 2;
        bitonic(arr, low, middle, 1);
        bitonic(arr, low + middle, middle, 0);
        merge(arr, low, high, dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitonic_sort() {
        let mut arr = [1, 3, 5, 7, 9, 11, 13, 15, 2, 4, 6, 8, 10, 12, 14, 16];
        bitonic(&mut arr, 0, 16, 1);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    }
}
