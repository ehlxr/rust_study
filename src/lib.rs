use core::fmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, Read};
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Point1<T, U> {
    pub x: T,
    pub y: U,
}

impl<D, Z> Point1<D, Z> {
    pub fn x(&self) -> &D {
        &self.x
    }
}

impl Point1<i32, i32> {
    pub fn y(&self) -> i32 {
        self.y
    }
}

impl<T, U> Point1<T, U> {
    pub fn mixup<V, W>(self, p: Point1<V, W>) -> Point1<T, W> {
        Point1 { x: self.x, y: p.y }
    }
}

pub fn test_tuple(t: (i32, &str)) {
    println!("{} {}", t.0, t.1);
}

pub static mut COUNTER: u32 = 0;

pub fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[derive(Debug)]
pub struct Bt {
    pub height: u32,
    pub label: Option<String>,
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("{:?}", self);
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 实际绘制按钮的代码
        println!("{:?}", self);
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn hello_str(name: &str) {
    println!("Hello, {}!", name);
}

#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        use crate::List::*;
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//enum IpAddrKind {
//    V4,
//    V6,
//}

#[derive(Debug)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub enum Color {
    _Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

pub enum Msg {
    _Quit,
    ChangeColor(Color),
}

// Quit 没有关联任何数据。
// Move 包含一个匿名结构体。
// Write 包含单独一个 String。
// ChangeColor 包含三个 i32。
#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    pub fn call(&self) {
        println!("this message call.");
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("this clarinet");
        }
    }
}

pub mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

///  传播错误
pub fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    //    1.
    // let f = File::open(path);
    //
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };
    //
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    //    2.
    let mut f = File::open(path)?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    //    3.
    // let mut s = String::new();
    // File::open(path)?.read_to_string(&mut s)?;
    // Ok(s)

    //    4.
    // fs::read_to_string(path)
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// impl ToString for Tweet {
//     fn to_string(&self) -> String {
//         format!("{:?}", self)
//     }
// }

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tweet username: {}, content: {}",
            self.username, self.content
        )
    }
}

//fn notify<T: Summary + ToString, U: Summary>(t: &T, u: &U) -> String {
//    format!("{}, {}", u.summarize(), t.to_string())
//}

//fn notify(t: &(impl Summary + ToString), u: &(impl Summary)) -> String {
//    format!("{}, {}", u.summarize(), t.to_string())
//}

pub fn notify<T, U>(t: &T, u: &U) -> String
where
    T: Summary + ToString,
    U: Summary,
{
    format!("{}, {}", u.summarize(), t.to_string())
}

// 返回实现了指定 trait 的方法
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    //fn largest(list: &[impl PartialOrd + Copy]) -> impl PartialOrd + Copy {
    //fn largest<T>(list: &[T]) -> T
    //    where T: PartialOrd + Copy {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    // for item in list.iter() {
    //     if *item > largest {
    //         largest = *item;
    //     }
    // }

    return largest;
}

pub fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

#[derive(Debug)]
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只有那些为 T 类型实现了 PartialOrd trait（来允许比较）和 Display trait（来启用打印）的 Pair<T> 才会实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     calculation: T,
//     value: Option<u32>,
// }
//
// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Self {
//             calculation,
//             value: None,
//         }
//     }
//
//     fn value(&mut self, args: u32) -> u32 {
//         match self.value {
//             Some(&v) => v,
//             None => {
//                 let v = (self.calculation)(args);
//                 v
//             }
//         }
//     }
// }

pub struct Cacher<T, K, V>
where
    T: Fn(&K) -> V,
{
    pub calculation: T,
    pub value: HashMap<K, V>,
}
use core::hash::Hash;
impl<T, K, V> Cacher<T, K, V>
where
    V: Copy,
    K: Hash + Eq,
    T: Fn(&K) -> V,
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Self {
            calculation,
            value: HashMap::<K, V>::new(),
        }
    }

    pub fn value(&mut self, args: K) -> V {
        let value = &mut self.value;

        match value.get(&args) {
            Some(&v) => v,
            None => {
                let x = (self.calculation)(&args);
                let x1 = x.clone();
                value.insert(args, x);
                x1
            }
        }

        // match self.value.get(&args) {
        //     Some(&v) => v,
        //     None => {
        //         let x = (self.calculation)(&args);
        //         let x1 = x.clone();
        //         self.value.insert(args, x);
        //         x1
        //     }
        // }
    }
}

#[derive(Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        // .iter() //  a collection of type `std::vec::Vec<Shoe>` cannot be built from `std::iter::Iterator<Item=&Shoe>`
        .into_iter() //获取 vector 所有权的迭代器
        .filter(|s| s.size == shoe_size)
        .collect()
}

pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
