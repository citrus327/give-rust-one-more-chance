mod my_trait;

use std::fmt::Display;

use my_trait::traits::Summary;

pub struct Article {
    pub title: String,
}

impl Summary for Article {
    fn summarize(&self) {
        println!("The method is overrided, The title is {}", &self.title)
    }
}

impl Summary for dyn Display {}

fn main() {
    let artitle = Article {
        title: String::from("C++ Prime"),
    };

    artitle.summarize();

    need_to_impl_summary(&artitle);

    need_to_impl_summary_simple(&artitle);
}

// 要求item对应的T类型必须实现Summary Trait
fn need_to_impl_summary<T: Summary>(item: &T) {
    item.summarize()
}

// 缩略版
fn need_to_impl_summary_simple(item: &impl Summary) {
    item.summarize()
}
