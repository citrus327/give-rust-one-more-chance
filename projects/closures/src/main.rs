use std::any;

mod inventory;

struct Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher { query, value: None }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn e1() {
    inventory::run()
}

fn e2() {
    let a_typed_closure = |x: i32, y: i32| -> i32 { x + y };
    println!("a_typed_closure(2, 3): {}", a_typed_closure(2, 3));
}

fn e3() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // ERROR
}

fn auto_borrowing() {
    let x = 42;

    // 通过引用捕获的闭包（自动借用）
    let closure_ref = || println!("x is {}", x);
    closure_ref();

    // 通过值捕获的闭包（自动借用）
    let closure_val = || {
        let y = x + 1;
        println!("y is {}", y);
    };
    closure_val();
}

#[test]
fn fnonce() {
    fn fn_once<F>(func: F)
    where
        F: Fn(usize) -> bool,
    {
        println!("{}", func(3));
    }

    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());
    fn_once(|z| z == x.len());
}

#[test]
fn after_move() {
    let closure = |mut v: Vec<i32>| {
        println!("vec length, {}", v.len());
        v.push(2);
        println!("vec length, {}", v.len());
    };

    let vec = vec![1, 2, 3];
}

fn main() {}
