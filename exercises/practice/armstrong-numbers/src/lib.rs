pub fn is_armstrong_number(num: u32) -> bool {
    // 十进制字符串的长度就是位数，0 也是 1 位
    let s = format!("{num}");
    let l = s.len();
    s.chars()
        // 字符只可能是 0-9，to_digit 不会失败
        .map(|c| c.to_digit(10).unwrap().pow(l as u32))
        // 累加一旦溢出就得到 None，后面就不会等于 Some(num)
        .try_fold(0u32, u32::checked_add)
        == Some(num)
}
