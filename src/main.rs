use getrandom::getrandom;

/// Get a random `usize` between `lower` (inclusive) and `upper` (exclusive).
///
/// Returns `None` if there was an error generating the random number.
/// Otherwise returns `Some(random_usize)` where `random_usize` is a random usize
/// and `lower <= random_usize < upper`.
fn rand(lower: usize, upper: usize) -> Option<usize> {
    let mut buf = [0u8; std::mem::size_of::<usize>()];
    getrandom(&mut buf).ok()?;
    Some(std::cmp::max(usize::from_ne_bytes(buf) % upper, lower))
}

/// Rearranges `a` so that all elements with index greater than or equal to `l`
/// and less than or equal to `r` less than `a[i]` come before `a[i]` and greater
/// than `a[i]` come after `a[i]`.
///
/// Returns the new index of `a[i]` *after* rearranging a. 
///
/// # Panics
/// Panics if any of *l*, *r*, or *i* are greater than or equal to `a.len()`.
fn partition(a: &mut [f32], l: usize, r: usize, i: usize) -> usize {
    // This code is shamefully adapated from Wikipedia pseudocode.
    let x = a[i];
    a.swap(i, r); // Move x (pivot) to end
    let mut j = l;
    for idx in l..r {
        if a[idx] < x {
            a.swap(j, idx);
            j += 1;
        }
    }
    a.swap(r, j); // Move x (pivot) to its final place
    j
}

/// Selects the *k*th largest element in `a`, or `None` if `a` is empty.
fn select(a: &mut [f32], k: usize) -> Option<f32> {
    // This code is also shamefully adapated from Wikipedia pseudocode.
    if a.is_empty() {
        return None;
    }

    let n = a.len();
    let (mut l, mut r) = (0, n - 1);
    loop {
        if l == r {
            return Some(a[l]);
        }
        let i = rand(l, r + 1)?;
        let i = partition(a, l, r, i);
        if k < i {
            r = i - 1;
        } else if k > i {
            l = i + 1;
        } else {
            return Some(a[k]);
        }
    }
}

/// Computes the median of `a`.
///
/// If `a` contains *n* numbers, then the median is the *n* / 2 largest
/// number in `a`.
///
/// Returns `None` if `a` is empty.
fn median(a: &mut Vec<f32>) -> Option<f32> {
    let n = a.len();
    if n % 2 == 0 {
        // Even length
        select(a, n / 2)
            .and_then(|fst| select(a, (n / 2) - 1).map(|snd| (fst + snd) / 2.0))
    } else {
        // Odd length
        select(a, n / 2)
    }
}

fn main() {
    let mut numbers = vec![7.0, 2.0, 3.0, 5.0, 10.0];
    let answer = median(&mut numbers);

    println!("median({:?}) = {:?}", numbers, answer);
}

#[test]
fn empty_list() {
    let mut input = &mut vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let mut input = &mut vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let mut input = &mut vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let mut input = &mut vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
