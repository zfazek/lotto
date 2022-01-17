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
    let mut numbers = BTreeSet::new();
    let mut buf = [0u8; 1];
    while numbers.len() < n {
        getrandom::getrandom(&mut buf).unwrap();
        let secret_number = (buf[0] % limit as u8 + 1) as usize;
        numbers.insert(secret_number);
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
            for idx in 0..v.len() - 1 {
                assert!(v[idx] < v[idx + 1]);
                assert!(v[idx] >= 1);
                assert!(v[idx] <= limit);
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
