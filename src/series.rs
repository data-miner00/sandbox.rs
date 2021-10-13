
pub fn generate_series(start: i32, for_len: i32) -> Vec<i32> {
    let mut results = Vec::new();
    for r in start..for_len + start {
        let res = match r % 2 {
            0 => (r+2),
            1 => -1 * (r+2),
            _ => 0
        };

        results.insert(0, res);
    }
    results.reverse();
    results
}

pub fn pretty_print(v: &Vec<i32>) -> String {
    let mut str: String = String::new();
    for i in 0..v.len() {
        str = format!("{} {} ", str, v.get(i).unwrap());
    }

    str
}