pub fn match_numeric(ch: char) -> Option<u64> {
    let num: u64 = match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => return None,
    };
    Some(num)
}

pub fn parse_number_from_str(target: &str) -> Result<u64, String> {
    let chars: Vec<char> = target.chars().collect();
    let chars_max_ind = chars.len() - 1;
    let nums = chars
        .into_iter()
        .enumerate()
        .map(|(ind, ch)| match match_numeric(ch) {
            Some(num) => Ok(num * 10u64.pow((chars_max_ind - ind).try_into().unwrap())),
            None => Err("Invalid numeric char in target".to_string()),
        })
        .collect::<Result<Vec<u64>, String>>()?;
    Ok(nums.into_iter().sum())
}
