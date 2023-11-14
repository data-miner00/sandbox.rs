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
