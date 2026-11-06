// This stub file contains items that aren't used yet; feel free to remove this module attribute
// 这个桩文件包含尚未使用的项目，可以随时删除此模块属性
// to enable stricter warnings.
// 以启用更严格的警告。
#![allow(unused)]

pub struct CsvRecordBuilder {
    content: String,
}

impl CsvRecordBuilder {
    // Create a new builder
    // 创建一个新的构建器
    pub fn new() -> Self {
        todo!("implement the `CsvRecordBuilder::new` method")
    }

    /// Adds an item to the list separated by a space and a comma.
    /// 添加一个项目，并用空格和逗号分隔列表。
    pub fn add(&mut self, val: &str) {
        todo!("implement the `CsvRecordBuilder::add` method, adding {val}")
    }

    /// Consumes the builder and returns the comma separated list
    /// 消耗构建器并返回逗号分隔的列表
    pub fn build(self) -> String {
        todo!("implement the `CsvRecordBuilder::build` method")
    }
}
