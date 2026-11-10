pub fn reverse(input: &str) -> String {
    // chars() 按 Unicode 标量值反转；组合字符（字素簇）不在本题范围内
    input.chars().rev().collect()
}
