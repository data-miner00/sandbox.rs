#[cfg(test)]


#[test]
#[should_panic]
fn test_basic() {
    

    assert!(1 == 1);
    
    panic!("Oh no");
}


#[test]
#[ignore]
fn test_equals() {
    let expected = 1;
    let result = super::get_two();

    assert_eq!(expected, result);
}

struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.height == self.width
    }
}

#[test]
#[should_panic]
fn test_square() {
    let r = Rectangle {
        width: 50,
        height: 25
    };

    assert!(r.is_square());
}