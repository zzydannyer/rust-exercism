// This stub file contains items that aren't used yet; feel free to remove this module attribute
// 这个桩文件包含尚未使用的项目，可以随时删除此模块属性
// to enable stricter warnings.
// 以启用更严格的警告。
#![allow(unused)]

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        todo!()
    }

    pub fn name(&self) -> &str {
        todo!()
    }

    pub fn age(&self) -> u32 {
        todo!()
    }

    pub fn weight(&self) -> f32 {
        todo!()
    }

    pub fn set_age(&mut self, new_age: u32) {
        todo!()
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        todo!()
    }
}
