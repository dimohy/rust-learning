use std::io;
use std::io::Write;

/*
    2. 터미널에서 문자열을 입력 받아서 그 문자열을 역순으로 출력하세요.
    예를 들어 터미널에서 "abbd" 를 입력 받았으면 "dbba"를 출력하세요.
**/
#[allow(dead_code)]
pub fn run()
{
    print!("? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    input = input.trim().to_string();
    println!("input = {}", input);

    let output: String = input.chars().rev().collect();
    println!("output = {}", output);
}