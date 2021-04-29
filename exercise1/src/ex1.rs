/*
    1. 정수 1부터 100까지 더하여 화면에 출력하는 프로그램을 작성하세요.
**/
#[allow(dead_code)]
pub fn run()
{
    let mut sum:u32 = 0;

    for n in 1..101
    {
        sum += n;
    }

    println!("sum = {}", sum);
}