use std::io;
use std::io::Write;

/*
    3. 임의의 숫자를 입력 받고(만약 문자열을 입력하면 에러메시지를 내고 다시 입력 받음) 그 숫자를 20 자리의 xxx,xxx,xxx 형태로 출력하세요.
    만약 출력 문자열의 자릿수가 20 자리가 안되면 앞에 '0' 을 붙여주세요.
    예를 들어 1234567 을 터미널에서 입력했으면 000000000001,234,567 와 같이 출력하세요.
**/
#[allow(dead_code)]
pub fn run()
{
    print!("? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input_num: i64 = match input.trim().parse()
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("Invalid input number.");
            return;
        }
    };
    println!("input = {}", input_num);

    let output = format!("{:0>20}", get_format_num(input_num));
    println!("output = {}", output);
}

fn get_format_num(number: i64) -> String
{
    let mut result = String::new();
    let s = number.to_string();
    let cs = s.chars().rev().enumerate();

    for (i, c) in cs
    {
        if i != 0 && i % 3 == 0
        {
            result.insert(0, ',');
        }
        result.insert(0, c);
    }

    result
}