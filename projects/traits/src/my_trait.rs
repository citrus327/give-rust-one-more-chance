pub mod traits {
    pub trait Summary {
        fn summarize(&self) {
            println!("Read more!")
        }
    }
}
