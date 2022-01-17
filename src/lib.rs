use wasm_bindgen::prelude::*;
use std::collections::HashSet;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate_numbers(n: usize, limit: usize, name: &str) -> String {
    let mut numbers = HashSet::new();
    let mut buf = [0u8; 1];
    while numbers.len() < n {
        getrandom::getrandom(&mut buf).unwrap();
        let secret_number = (buf[0] % limit as u8 + 1) as usize;
        numbers.insert(secret_number);
    }
    let mut ret = Vec::new();
    for n in numbers {
        ret.push(n);
    }
    ret.sort();
    let mut str = String::from(name.to_string() + "[");
    for (i, n) in ret.iter().enumerate() {
        str += &n.to_string();
        if i < ret.len() - 1 {
            str += ", ";
        }
    }
    str += "]";
    str
}