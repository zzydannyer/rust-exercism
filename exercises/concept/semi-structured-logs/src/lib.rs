// This stub file contains items that aren't used yet; feel free to remove this module attribute
// 这个桩文件包含尚未使用的项目，可以随时删除此模块属性
// to enable stricter warnings.
// 以启用更严格的警告。
#![allow(unused)]

/// various log levels
/// 各种日志级别
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
/// 输出日志的主要函数
pub fn log(level: LogLevel, message: &str) -> String {
    todo!("return a message for the given log level")
}
pub fn info(message: &str) -> String {
    todo!("return a message for info log level")
}
pub fn warn(message: &str) -> String {
    todo!("return a message for warn log level")
}
pub fn error(message: &str) -> String {
    todo!("return a message for error log level")
}
