// &'static 表示这个字符串字面量在整个程序运行期间都有效
pub fn hello() -> &'static str {
    "Hello, World!"
}
