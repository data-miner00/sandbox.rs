use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub fn median(vec: &Vec<i32>) -> i32 {
    let mid: usize = vec.len() / 2;

    *vec.get(mid).unwrap()
}

pub fn mean(vec: &Vec<i32>) -> i32 {
    let len = vec.len() as i32;
    let mut accumulator: i32 = 0;

    for i in vec {
        accumulator += i;
    }

    accumulator / len
}

pub fn mode(vec: &Vec<i32>) -> i32 {
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

pub fn pig_latinify(line: &str) -> String {
    let mut letters = line.chars();
    let first_letter = letters.next().unwrap();
    let first_letter_lower = first_letter.to_lowercase().next().unwrap();
    match first_letter_lower {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", letters.as_str()),
        _ => format!("{}-{}ay", letters.as_str(), first_letter_lower)
    }
}

pub fn print_greetings() {
    let greetings = ["Hello", "こんいちは", "Hej"];

    for (index, greeting) in greetings.iter().enumerate() {
        print!("{} : ", greeting);
        match index {
            0 => println!("This code is editable and runnable!"),
            1 => println!("このコードは編集可能と実行可能です"),
            2 => println!("Den har koden kan redigeras och koras!"),
            _ => println!("What do you mean")
        }
    }
}

pub fn read_file() {
    let mut file = File::open(".gitignore").expect("Cant open file");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Cannot read file");

    println!("{}", contents);
}

pub fn write_file() {
    let mut file = File::create(".sampah").expect("Cant be created");

    file.write_all(b"Some sampah text").expect("Cant be written");
}

fn h(i: i32) -> Result<i32, String> {
    match i {
        i if i >= 0 => Ok(i + 10),
        _ => Err(format!("Input to h less than 0, found {}", i))
    }
}

fn use() {
    let input: i32 = 5;

    match h(input) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e)
    };

    if let Ok(i) = h(4) {
        // do smth with i
    }

    let i = match h() {
        Ok(i) => i,
        err => return err,
    };

    let i = h()?;
}

fn use_hash_map() {
    let marks = HashMap::new();

    // Insert entry
    marks.insert("Rust Programming", 96);

    // Get length
    println!("Subjects taken: {}", marks.len());

    // Match result
    match marks.get("Rust Programming") {
        Some(mark) => println!("Rust Programming: {}", mark),
        None => println!("You not enrolled")
    }

    // Remove entry
    marks.remove("Rust Programming");

    // Looping thru hm
    for (subject, mark) in &marks {
        println!("For {} you got {}", subject, mark);
    }

    // Check for a key
    println!("Did you study C++? {}", marks.contains_key("C++ Programming"));
}

fn string_methods() {
    {
        let my_string = String::from("Rust is fantastic");

        println!("After replace: {}", my_string.replace("fantastic", "great"));
    }


    {
        let my_string = String::from("The weather is\nnice\noutside");

        for line in my_string.lines() {
            println!("{}", line);
        }
    }

    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("At index 2: {}", tokens[2]);
    }

    {
        let my_string = String::from("        My name is   ");

        println!("After trim: {}", my_string.trim());
    }

    {
        let my_string = String::from("ghfhdfhsf");

        match my_string.chars().nth(4) {
            Some(c) => c,
            None => '.',
        }
    }
}

fn use_regex() {
    let re = Regex::new(r"\w{5}").unwrap();
    let text = "cod";

    println!("Found matfh? {}", re.is_match(text));

    match re.captures(text) {
        Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_string()), // &caps[0]
        None => println!("Fail")
    }


}

fn use_option() {
    let name = String::from("Domenic");

    println!("Character at index 8: {}", match name.chars().nth(8) {
        Some(c) => c.to_string(),
        None => "No fking way".to_string()
    });

    fn get_occupation(name: &str) -> Option<&str> {
        match name {
            "Domenic" => Some("Software Developer"),
            _ => None,
        }
    }
}

fn use_http() {
    match reqwest::get("http://youtube.local/hello") {
        Ok(mut response) => {
            if response.status() == reqwest::StatusCode::Ok {
                match response.text() {
                    Ok(text) => println!("Response text: {}", text),
                    Err(_) => println!("Could not read response text")
                }
            } else {
                println!("Not OK!");
            }
        },
        Err(_) => println!("Could not make the request")
    }
}

fn use_http2() {
    let response_text = reqwest::get("http://youtube.local/hello")
        .expect("Couldnt make request")
        .text()
        .expect("Could not read the response text");
}

use std::process::Command;

fn call_cmd() {

    let mut cmd = Command::new("python");
    cmd.arg("dcode.py");

    match cmd.output() {
        Ok(o) => {
            // unsafe
            unsafe {
                println!("Output: {}", String::from_utf8_unchecked(o.stdout));
            }
            println!("Output:{} ", String::from_utf8_lossy(&o.stdout));
        },
        Err(e) => {
            println!("There was an error: {}", e);
        }
    }
}

use serde_json::Value as JsonValue;

extern crate serde;

[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool
}

fn use_serde() {
    let json_str = r#"
        {
            "name": "Domenic",
            "age": 65,
            "is_male": true
        }
    "#;

    let res = serde_json::from_str(json_str);

    // if res.is_ok() {
    //     let p: JsonValue = res.unwrap();
    //     println!("The name is {}", p["name"]);
    // }
    if res.is_ok() {
        let p: Person = res.unwrap();
        println!("The name is {}", p.name);
    }
}

fn add_four(x: i32) -> i32 {
    x + 4
}

fn maybe_add_four(y: Option<i32>) -> Option<i32> {
    y.map(add_four)
}

// closure form
fn maybe_add_four_closure(y: Option<i32>) -> Option<i32> {
    y.map(|x| x + 4)
}

fn foo(input: Option<i32>) -> Option<i32> {
    match input {
        Some(i) if i >= 0 => Some(i),
        _ => None
    }
}

fn foo(input: Option<i32>) -> Option<i32> {
    input.and_then(|i| {
        if i < 0 {
            None
        } else {
            Some(i)
        }
    })
}

fn foo(input: Option<i32>) -> Option<i32> {
    input.filter(|i| i >= 0)
}

fn bar(input: Option<i32>) -> Result<i32, ErrNegative> {
    foo(input).ok_or(ErrNegative)
}

fn ping_all(foos: &[Foo]) {
    for f in foos {
        f.ping();
    }
}

