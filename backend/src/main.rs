use regex::Regex;
use std::fs;

fn main() {
    let file_path = "./src/test.txt";
    let chars_to_trim: &[char] = &[' ', '\n'];

    let contents = fs::read_to_string(file_path)
        .expect("unable to read file");

    //*Convert contents into a boxed string to avoid corrupting data
    let boxed_contents: Box<str> = contents.into_boxed_str();

    let re = Regex::new(r"%([^%]+)%").unwrap();
    let mut vector_match: Vec<String> = Vec::new();

    for found in re.captures_iter(&boxed_contents) {
        let mut tmp_str= String::new();
        for i in 0..1 {
          tmp_str.push_str(&found[i]); 
        }
        vector_match.push(tmp_str);
    }

    for exercise in vector_match {
        let mut index = 0u32;
        let mut set_index = 0u32;
        let mut start_flg = 0u16;
        for line in exercise.lines() {
            if line == "%" {
                if start_flg == 0 {
                    println!("Start");
                    start_flg = 1;
                }
                else if start_flg == 1 {
                    println!("End");
                    start_flg = 0;
                }
            }
            else if index == 1 {
                let nameofexercise: Vec<&str> = line.split('-').collect();
                println!("For{}, you completed", nameofexercise[1].trim_matches(chars_to_trim));
            }
            else if index >= 2 {
                set_index += 1;
                let set_info: Vec<&str> = line.trim_matches(chars_to_trim).split('*').collect();
                println!("set {}: {} reps with {} kg", set_index, set_info[0],set_info[1]);
            }
            index += 1;
        }
    }

}

