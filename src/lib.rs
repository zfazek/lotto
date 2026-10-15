use wasm_bindgen::prelude::*;
use std::collections::BTreeSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate_numbers(n: usize, limit: usize, name: &str) -> String {
    let numbers = generate_random_numbers(n, limit);
    format_set(&numbers, name)
}

fn generate_random_numbers(n: usize, limit: usize) -> BTreeSet<usize> {
    assert!(limit >= 1 && limit <= 256, "limit must be in 1..=256");
    assert!(n <= limit, "cannot draw more distinct numbers than the limit");
    let mut numbers = BTreeSet::new();
    let mut buf = [0u8; 1];
    // Largest multiple of `limit` that fits in a byte's 0..=255 range.
    // Reject bytes at or above it to avoid modulo bias and keep the
    // distribution uniform (important for a lottery simulation).
    let max_acceptable = (256 / limit) * limit;
    while numbers.len() < n {
        getrandom::getrandom(&mut buf).unwrap();
        let value = buf[0] as usize;
        if value >= max_acceptable {
            continue;
        }
        numbers.insert(value % limit + 1);
    }
    numbers
}

fn format_set(numbers: &BTreeSet<usize>, name: &str) -> String {
    let mut str = String::from(name.to_string() + "[");
    for (i, number) in numbers.iter().enumerate() {
        if number < &10 {
            str += " ";
        }
        str += &number.to_string();
        if i < numbers.len() - 1 {
            str += ", ";
        }
    }
    str += "]";
    str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_numbers() {
        for _ in 0..1000 {
            let n = 3;
            let limit = 10;
            let numbers = generate_random_numbers(n, limit);
            assert_eq!(numbers.len(), n);
            let v: Vec<usize> = numbers.into_iter().collect();
            // `windows(2)` yields nothing for len < 2, so no underflow risk.
            for pair in v.windows(2) {
                assert!(pair[0] < pair[1]);
            }
            for &x in &v {
                assert!(x >= 1);
                assert!(x <= limit);
            }
        }
    }

    #[test]
    fn test_format_set() {
        let mut numbers = BTreeSet::new();
        numbers.insert(10);
        numbers.insert(5);
        numbers.insert(8);
        let result = format_set(&numbers, "foo: ");
        assert_eq!("foo: [ 5,  8, 10]", result);
    }
}
