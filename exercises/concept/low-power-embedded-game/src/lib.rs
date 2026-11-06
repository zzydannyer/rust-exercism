// This stub file contains items that aren't used yet; feel free to remove this module attribute
// 这个桩文件包含尚未使用的项目，可以随时删除此模块属性
// to enable stricter warnings.
// 以启用更严格的警告。
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    todo!("implement `fn divmod`");
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    todo!("implement `fn evens`");
    // TODO: remove this; it's only necessary to allow this function to compile
    // TODO：删除此内容；它只用于让该函数在
    // before the student has done any work.
    // 学生完成任何实现前能够编译。
    std::iter::empty()
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        todo!("implement `fn manhattan`")
    }
}
