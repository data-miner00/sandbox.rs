//! # common
//!
//! `common` is a collection of utilities to make performing certain
//! calculations more convenient.

mod threading;

#[derive(Clone)]
pub struct Person {
    pub name: String,
    pub age: i32,
    pub country: String,
}

impl Person {
    pub fn new(&self, name: String, age: i32, country: String) -> Self {
        Self { name, age, country }
    }

    pub fn rename(self, name: String) -> Self {
        Self { name, ..self }
    }
}

/// Adds an unsigned integer to another.
/// 
/// # Examples
/// 
/// ```
/// let left: usize = 5;
/// let right: usize = 8;
/// 
/// let answer = common::add(left, right);
/// 
/// assert_eq!(13, answer);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
