use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // // 我们试着用 `unwrap()` 把数字放出来。它会咬我们一口吗？
    // let first_number = first_number_str.parse::<i32>().unwrap();
    // let second_number = second_number_str.parse::<i32>().unwrap();
    // first_number * second_number

    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    first_number_str.parse::<i32>().and_then(|first_number|{ //里面要返回result，所以这里不能用map
        second_number_str.parse::<i32>().map(|second_number|{
            second_number* first_number
        })
    })
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_result() {
        let twenty = multiply("10", "2");
        println!("twenty is {:?}", twenty);
        let tt = multiply("t", "2");
        println!("double is {:?}", tt);

        let twenty = multiply2("10", "2");
        println!("twenty is {:?}", twenty);
        let tt = multiply2("t", "2");
        println!("double is {:?}", tt);
    }

    #[test]
    fn test_option() {
        let x: Option<&str> = None;
        assert_eq!(x.map_or(42, |v| v.len()), 42);
    }
}
