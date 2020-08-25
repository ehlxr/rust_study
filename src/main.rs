use std::cell::RefCell;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{collections::HashMap, fmt::Display, fs, fs::File, io, io::ErrorKind, thread};

fn main() {
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    let c: char = 'd';
    println!("{}", c);

    let x = 5;
    // cannot assign twice to immutable variable
    // x = x + 1;

    // 使用 let 重新定义 x，隐藏了之前 x 的值
    let x = x + 1;
    println!("The value of x is {}", x);

    // let x = "test";
    let x: &'static str = "test";
    println!("The value of x is {}", x);

    // 可变变量
    let mut x = 2;
    x = x * 3;
    // expected integer, found reference
    // x = "test";
    println!("The value of x is {}", x);

    // : 指定变量类型
    let mut y: i32 = 22;
    println!("The value of y is {}", y);

    y = 12;
    println!("The value of y is {}", y);

    // 可变引用
    let mut s = String::from("hello");
    let s1 = &mut s;
    // 在特定作用域中的特定数据有且只有一个可变引用
    // let s2 = &mut s;

    change(s1);
    println!("{}", s1);
    {
        let s2 = &mut s;
        println!("{}", s2);
    }

    // 多个不可变引用是可以的
    let r1 = &s;
    let r2 = &s;
    // 不能在拥有不可变引用的同时拥有可变引用
    // let r3 = &mut s;

    println!("{}, {}", r1, r2);

    // ---------------------------
    println!("----------------------------------------");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    let m = Message::Quit;
    m.call();

    let m = &Message::Move { x: 1, y: 2 };
    match m {
        Message::Move { x, y } => println!("x: {}, y: {}", x, y),
        _ => {}
    }
    m.call();

    let m = Message::Write(String::from("test"));
    m.call();

    let m = Message::ChangeColor(1, 2, 3);
    m.call();

    // ---------------------------
    println!("----------------------------------------");
    let my_str = String::from("hello world!");
    // let my_str = "hello world!";
    println!("{}, {}", my_str, first_words(&my_str[..]));
    println!("get first word: {}", first_words("hello world"));

    let rect = Rectangle {
        width: 21,
        height: 23,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 is {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rece4 = Rectangle::square(8);
    println!(
        "The area of the rectangle is {} square pixels.",
        rece4.area()
    );

    // ---------------------------
    println!("-----------------vec-----------------------");
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    // let mut v=Vec::new();
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let f = v.get(100);
    println!("{:?}", f);
    let f = &v[0];
    println!("{}", f);
    // v.push(12);

    let d: Option<u32> = None;
    println!("{:?} {}", d, d.is_none());

    let s = Some("ddd");
    println!("{:#?}", s);

    let s: Option<u32> = Some(2);
    println!("{},{}", s.unwrap(), s.is_some());

    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();

    let s1 = String::from("Hello");
    let s2 = String::from("World");

    // + == fn add(self, s: &str) -> String
    // 这个语句会获取 s1 的所有权，附加上从 s2 中拷贝的内容，并返回结果的所有权
    let s3 = s1 + &s2;
    let s = format!("{}-{}", s2, s3);
    println!("{}", s);

    let hello = "你好";
    println!("{}", hello.len()); // output: 6

    let s = &hello[0..3]; // output: 你
    println!("{}", s);

    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }

    // ---------------------------
    println!("----------------------------------------");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{}", score.unwrap());

    for (k, v) in scores.iter() {
        println!("{}, {}", k, v)
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    let black = scores.entry(String::from("black")).or_insert(50);

    // cannot borrow `scores` as immutable because it is also borrowed as mutable
    // println!("{:?}, {}", scores, black);
    println!("{}", black);
    println!("{:#?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // ---------------------------
    println!("----------------------------------------");
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            ot_err => panic!("There was a problem opening the file: {:?}", ot_err),
        },
    };
    println!("{:?}", f.metadata());

    match read_username_from_file("hello2.txt") {
        Ok(st) => println!("{}", st),
        Err(err) => panic!("There was a problem read string: {:?}", err),
    };

    let h: std::net::IpAddr = "127.0.0.1".parse().unwrap();
    let s: String = "127.0.0.1".parse().unwrap();
    println!("{:?}, {}", h, s);

    let v = ["d", "fd"];
    println!("{:?}", v);
    let v = vec!["d", "fd"];
    println!("{:?}", v);

    // ---------------------------
    println!("----------------------------------------");
    let t = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("{}", t.to_string());

    let t1 = returns_summarizable();

    println!("{}", notify(&t, &t1));

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest2(&char_list);
    println!("The largest char is {}", result);

    // ---------------------------
    println!("----------------------------------------");
    let p = Pair::new(String::from("t"), String::from("s"));
    p.cmp_display();

    let p = Pair::new(
        Rectangle {
            width: 2,
            height: 3,
        },
        Rectangle {
            width: 4,
            height: 9,
        },
    );
    println!("{:?}", p);
    // no method named `cmp_display` found for type `Pair<Rectangle>` in the current scope
    // p.cmp_display();

    // ---------------------------
    println!("----------------------------------------");
    let s1 = String::from("abcd");
    let r;
    {
        let s2 = String::from("qw");
        r = longest(s1.as_str(), s2.as_ref());
        println!("The longest string is {}", r);
    }
    // `s2` does not live long enough
    // println!("The longest string is {}", r);

    let closure = |x: u32| -> u32 { x + 2 };
    // let closure = |x| x + 2;
    println!("closure: {}", closure(3));

    let mut s = String::from("hello");
    let d = change;
    d(&mut s);
    println!("{}", s);

    generate_workout(10, 3);

    // ---------------------------
    println!("----------------------------------------");
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("{}", equal_to_x(4));

    let x = vec![1, 2, 3];
    // move 强制闭包获取其使用的环境值的所有权
    let equal_to_x = move |z| z == x;
    //value used here after move
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    println!("{}", equal_to_x(y));

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("{:?}", in_my_size);

    let counter = Counter::new();
    let v: Vec<_> = counter.into_iter().map(|x| x + 1).collect();
    println!("{:?}", v);
    let v: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .collect();
    println!("{:?}", v);
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("{}", sum);

    // ---------------------------
    println!("----------------------------------------");
    use crate::List::*;

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 这会导致栈溢出
    // println!("a next item = {:?}", a.tail());
    // 并不是 a.tail(); 引起死循环，打印函数触发循环引用

    // ---------------------------
    println!("----------------------------------------");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!(
        "leaf strong = {}, weak = {}, parent = {:?}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        leaf.parent.borrow().upgrade(),
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        // let leaf = Rc::new(Node {
        //     value: 3,
        //     parent: RefCell::new(Rc::downgrade(&branch)),
        //     children: RefCell::new(vec![]),
        // });

        println!(
            "branch strong = {}, weak = {}, parent = {:?}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
            branch.parent.borrow().upgrade(),
        );

        println!(
            "leaf strong = {}, weak = {}, parent = {:?}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
            leaf.parent.borrow().upgrade(),
        );
    } // -> 离开作用域，此时，Rc<branch> 的强引用（strong_count）为 0，但是 Weak<branch> 弱引用数（weak_count）仍然为 1（引用 Weak<branch> 的 Rc<leaf> 仍然存在），
      // weak_count 无需计数为 0 就能使 Rc<branch> 实例被清理。
    println!(
        "leaf strong = {}, weak = {}, parent = {:?}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
        leaf.parent.borrow().upgrade(),
    );

    // ---------------------------
    println!("----------------------------------------");
    let x = Rc::new(5);
    let y = *x;
    assert_eq!(5, y);

    let r = Rc::new("Rust".to_string());
    // Rc 不支持解引用移动
    // let x = *r;
    // println!("{:?}", x);
    println!("{:?}", *r);

    let x = RefCell::new(5);
    println!("{}", *x.borrow());
    // 变量 x 非 mut，可通过以下代码修改 x 的值
    *x.borrow_mut() = 7;
    println!("{}", *x.borrow());

    let x = Box::new(5);
    let x1 = *x;
    println!("Box {}", x1);

    // ---------------------------
    println!("--------函数和方法的隐式解引用强制多态--------------");
    // let m = Box::new(String::from("Rust"));
    let m = MyBox::new(String::from("Rust"));
    //String 上的 Deref 实现: fn deref(&self) -> &str {...}

    // hello_str(&(*m)[..]);
    // 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型
    // &m -> &String -> &str
    hello_str(&m);

    // ---------------------------
    println!("---------------------------");
    let jh = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    jh.join().unwrap();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v);
    //      ^ value used here after move
    handle.join().unwrap();

    // ---------------------------
    println!("---------------------------");
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        // tx.send(12).unwrap();
        tx.send(String::from("12")).unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // ---------------------------
    println!("---------------------------");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // ---------------------------
    println!("---------------------------");
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // ---------------------------
    println!("---------------------------");

    let bt = Bt {
        height: 10,
        label: Some(String::from("test")),
    };

    let h = bt.label.unwrap();
    println!("{}", h);
    // borrow of moved value: `bt`
    // println!("{:?}", bt);

    // ---------------------------
    println!("---------------------------");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let x: u32 = 5;
    let point = (3, x as i32);
    print_coordinates(&point);

    // ---------------------------
    println!("---------------------------");
    // 匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 多个模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 通过 ... 匹配值的范围
    // 范围只允许用于数字或 char 值
    let x = 5;
    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // 解构枚举
    // let msg = Message::ChangeColor(0, 160, 255);
    let msg = &Message::Move { x: 1, y: 2 };
    // let msg = &Message::Quit;
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // 解构嵌套的结构体 & 枚举
    let msg = Msg::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Msg::ChangeColor(Color::_Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Msg::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // 解构引用
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
    println!("{}", sum_of_squares);

    // 解构结构体和元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{} {} {} {}", feet, inches, x, y);

    // 忽略模式中的值
    foo(3, 4);
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }
    match numbers {
        (first, .., last) => println!("Some numbers: {}, {}", first, last),
        // error: `..` can only be used once per tuple or tuple struct pattern
        // (.., second, ..) => println!("Some numbers: {}", second),
    }

    // 通过在名字前以一个下划线开头来忽略未使用的变量
    let _x = 5;
    // let y = 10;
    //     ^ help: consider prefixing with an underscore: `_y`

    struct Pt {
        x: i32,
        _y: i32,
        _z: i32,
    }
    let origin = Pt { x: 0, _y: 0, _z: 0 };
    match origin {
        Pt { x, .. } => println!("x is {}", x),
    }

    // 匹配守卫提供的额外条件
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = 4;
    let y = true;
    match x {
        // 匹配守卫中引用外部变量 y，而不是创建新变量 y
        // if 条件作用于整个 4 | 5 | 6 模式，类似：(4 | 5 | 6) if y => println!("yes"),
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // 使用 @ 可以在一个模式中同时测试和保存变量值。
    enum Ms {
        Hello { id: i32 },
    }
    let msg = Ms::Hello { id: 5 };
    match msg {
        Ms::Hello { id: id_var @ 3...7 } => println!("Found an id in range: {}", id_var),
        // 此匹配分支不能使用 id
        Ms::Hello { id: 10...12 } => println!("Found an id in another range"),
        Ms::Hello { id } => println!("Found some other id: {}", id),
    }

    // 遗留模式： ref 和 ref mut
    let robot_name = &Some(String::from("Bors"));
    match robot_name {
        Some(name) => println!("Found a name: {}", name),
        // 老版本中会如下使用
        // &Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);

    // ---------------------------
    println!("---------------------------");
    let mut num = 5;
    let r1 = &num as *const i32; // 不可变的裸指针
    let r2 = &mut num as *mut i32; // 可变的裸指针
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {
        println!("this dangerous function");
    }
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    //    let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);
    println!("{:?}", a);
    println!("{:?}", b);

    // 使用 extern 函数调用外部 C 语言代码
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 定义可供 C 语言调用的函数
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    // 静态变量
    // 访问不可变静态变量是安全的
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    // 访问和修改可变静态变量都是 不安全 的
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    unsafe trait Foo {
        // methods go here
    }
    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // ---------------------------
    println!("---------------------------");
    struct Context<'s>(&'s str);
    // 声明一个不短于 'c 的生命周期 's。
    // struct Parser<'c, 's: 'c> {
    struct Parser<'c, 's> {
        context: &'c Context<'s>,
    }
    impl<'c, 's> Parser<'c, 's> {
        fn parse(&self) -> Result<(), &'s str> {
            Err(&self.context.0[1..])
        }
    }
    fn parse_context(context: Context) -> Result<(), &str> {
        Parser { context: &context }.parse()
    }

    let centext = Context("test");
    match parse_context(centext) {
        Err(s) => println!("{}", s),
        _ => (),
    }

    // ---------------------------
    println!("---------------------------");
    trait Red {}
    struct Ball<'a> {
        diameter: &'a i32,
    }
    // impl<'a> Red for Ball<'a> {}
    impl Red for Ball<'_> {}

    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;

    // ---------------------------
    println!("---------------------------");
    struct StrWrap<'a>(&'a str);
    // '_ 表明在此处使用省略的生命周期,这意味着我们仍然知道 StrWrap 包含一个引用，不过无需所有的生命周期注解来知道。
    // fn foo<'a>(string: &'a str) -> StrWrap<'a> {
    fn fsw(string: &str) -> StrWrap<'_> {
        StrWrap(string)
    }

    // ---------------------------
    println!("---------------------------");
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // ---------------------------
    println!("---------------------------");
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    // let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // let list_of_statuses: Vec<Status> = (0u32..20).map(|i| Status::Value(i)).collect();
    println!("{:?}", list_of_statuses);

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    let rc = returns_closure();
    // println!("{}", rc(12));

    // ---------------------------
    println!("---------------------------");
    // use hello_macro::HelloMacro;
    // use hello_macro_derive::HelloMacro;

    // #[derive(HelloMacro)]
    // struct Pancakes;

    // Pancakes::hello_macro();

    let user1 = User {
        username: String::from("hllo"),
        email: String::from("hell@world.com"),
        sign_in_count: 1,
        active: false,
    };
    println!("{:?}", user1);

    let s: &str = "Hello";
    println!("{}", s);
    let mut s: String = "Hello".to_string();
    let w = "World";
    s.push_str(w); // 并不需要获取 w 的所有权
    s.push('!');
    println!("{} {}", s, w);

    let t = (1, "34");
    test_tuple(t);
    println!("{} {}", t.0, t.1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
                       // println!("{} {} {}", s1, s3, s2);
    println!("{} {}", s3, s2);

    let hello = "测试中文字符串";
    for c in hello.chars() {
        print!("{} ", c);
    }

    for b in hello.bytes() {
        print!("{} ", b);
    }

    // let s = hello[1]; // Rust 的字符串不支持索引
    // let s = &hello[0..4]; // thread 'main' panicked at 'byte index 4 is not a char boundary; it is inside '试' (bytes 3..6) of `测试中文字符串`', src/libcore/str/mod.rs:2219:5
    println!("{}", &hello[0..3]);

    println!("--------------HashMap-----------");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    for (k, v) in &scores {
        println!("k {}, v {}", k, v);
    }
    // 只在键没有对应值时插入
    // Entry 的 or_insert 方法在键对应的值存在时就返回这个值的 Entry ，
    // 如果不存在则将参数作为新值插入并返回修改过的 Entry
    let x3 = scores.entry("Blue".to_string()).or_insert(60);
    println!("{}", x3);
    let x2 = scores.entry("Red".to_string()).or_insert(55);
    println!("{}", x2);
    println!("{:?}", scores);

    let nms = vec![String::from("Blue"), String::from("Yellow")];
    let scs = vec![10, 50];
    let scores: HashMap<_, _> = nms.iter().zip(scs.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("hello");
    let field_value = 456;
    let mut map1 = HashMap::new();
    map1.insert(field_name, field_value);
    // borrow of moved value: `field_name`
    // println!("{} {} {:?}", field_name, field_value, map1);
    println!("{} {:?}", field_value, map1);

    let k = &String::from("hello");
    let v = String::from("world");
    let mut map1 = HashMap::new();
    map1.insert(k, v);
    // borrow of moved value: `v`
    // println!("{} {} {:?}", k, v, map1);
    println!("{} {:?}", *k, map1);

    let option = map1.get(k);
    println!("{}", option.unwrap());

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for tex in text.split_whitespace() {
        let counts = map.entry(tex).or_insert(0);
        *counts += 1;
    }
    println!("{:?}", map);

    println!("----------------错误处理-----------------");
    let fnames = "hello2.txt";
    let result = File::open(fnames);
    let file = match result {
        Ok(f) => f,
        // Err(e) => {
        //     println!("Problem opening the file: {:?}", e);
        //
        //     File::create(fnames).unwrap()
        // }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(fnames) {
                Ok(f) => f,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            oe => panic!("Problem opening the file: {:?}", oe),
        },
    };
    println!("{:?}", file);

    println!("----------------match 匹配 枚举------------------");
    let m = Message::Quit;
    match m {
        Message::Write(s) => println!("{}", s),
        // 此处可使用任意字符匹配其它情况
        abc => println!("{:?}", abc),
    };

    println!("----------------错误传递--------------");
    let r = read_username_from_file("hello2.txt");
    match r {
        Ok(s) => println!("read from file result: {}", s),
        Err(e) => println!("read from file error: {:?}", e),
    };

    println!("--------------泛型--------------");
    let p = Point1 { x: 5, y: 10 };

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let p1 = Point1 { x: "hello", y: 'd' };
    let p2 = p.mixup(p1);
    println!("{:?}", p2);
}

#[derive(Debug)]
struct Point1<T, U> {
    x: T,
    y: U,
}

impl<D, Z> Point1<D, Z> {
    fn x(&self) -> &D {
        &self.x
    }
}

impl Point1<i32, i32> {
    fn y(&self) -> i32 {
        self.y
    }
}

impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, p: Point1<V, W>) -> Point1<T, W> {
        Point1 { x: self.x, y: p.y }
    }
}

fn test_tuple(t: (i32, &str)) {
    println!("{} {}", t.0, t.1);
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

use core::fmt;
use std::fmt::Formatter;
use std::io::Read;
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // let len = slice.len();
    // assert!(mid <= len);
    // (&mut slice[..mid], &mut slice[mid..])

    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

#[derive(Debug)]
struct Bt {
    height: u32,
    label: Option<String>,
}

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
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

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello_str(name: &str) {
    println!("Hello, {}!", name);
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        use crate::List::*;
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//enum IpAddrKind {
//    V4,
//    V6,
//}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Color {
    _Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Msg {
    _Quit,
    ChangeColor(Color),
}

// Quit 没有关联任何数据。
// Move 包含一个匿名结构体。
// Write 包含单独一个 String。
// ChangeColor 包含三个 i32。
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("this message call.");
    }
}

//fn first_words<'a>(s: &'a str) -> &'a str {
fn first_words(s: &str) -> &str {
    let b = s.as_bytes();
    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
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

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}

///  传播错误
fn read_username_from_file(path: &str) -> Result<String, io::Error> {
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

trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
struct Tweet {
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

fn notify<T, U>(t: &T, u: &U) -> String
where
    T: Summary + ToString,
    U: Summary,
{
    format!("{}, {}", u.summarize(), t.to_string())
}

// 返回实现了指定 trait 的方法
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
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

fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只有那些为 T 类型实现了 PartialOrd trait（来允许比较）和 Display trait（来启用打印）的 Pair<T> 才会实现 cmp_display 方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
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

struct Cacher<T, K, V>
where
    T: Fn(&K) -> V,
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    V: Copy,
    K: Hash + Eq,
    T: Fn(&K) -> V,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Self {
            calculation,
            value: HashMap::<K, V>::new(),
        }
    }

    fn value(&mut self, args: K) -> V {
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

fn generate_workout(intensity: u32, random_number: u32) {
    // let calculation = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_millis(500));
        num + 1
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        // .iter() //  a collection of type `std::vec::Vec<Shoe>` cannot be built from `std::iter::Iterator<Item=&Shoe>`
        .into_iter() //获取 vector 所有权的迭代器
        .filter(|s| s.size == shoe_size)
        .collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| {
            println!("cal: {}", a);
            a + 1
        });

        let v1 = c.value(2);
        let v1 = c.value(2);
        assert_eq!(v1, 3);

        let v2 = c.value(-2);
        assert_eq!(v2, -1);
    }

    #[test]
    fn call_with_different_type() {
        let mut c = Cacher::new(|a| {
            println!("\ncal: {}", a);
            7
        });

        let v = c.value("tetsss");
        let v = c.value("s");
        assert_eq!(v, 7);
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}
