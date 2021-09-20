use std::collections::HashMap;

fn main() {

    let ints: Vec<i32> = vec![34, 53, 543, 23, 34];

    println!("mean: {} \nmedian: {}\nmode: {}",
             mean(&ints),
             median(&ints),
             mode(&ints));

    println!("{}", pig_latinify("whatthehell"));
    println!("{}", pig_latinify("apple"));
}

fn median(vec: &Vec<i32>) -> i32 {
    let mid: usize = vec.len() / 2;

    *vec.get(mid).unwrap()
}

fn mean(vec: &Vec<i32>) -> i32 {
    let len = vec.len() as i32;
    let mut accumulator: i32 = 0;

    for i in vec {
        accumulator += i;
    }

    accumulator / len
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut mode: i32 = 0;
    let mut result: i32 = 0;

    for i in vec {
        let result = map.entry(i).or_insert(0);
        *result += 1;
    }

    for (k, v) in map.iter() {
        if *v > mode {
            mode = *v;
            result = **k;
        }
    }

    result
}

fn pig_latinify(line: &str) -> String {
    let mut letters = line.chars();
    let first_letter = letters.next().unwrap();
    let first_letter_lower = first_letter.to_lowercase().next().unwrap();
    match first_letter_lower {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", letters.as_str()),
        _ => format!("{}-{}ay", letters.as_str(), first_letter_lower)
    }
}