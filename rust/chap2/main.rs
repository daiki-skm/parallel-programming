fn let_example() -> u32 {
    let x = 100;
    let mut y = 20;
    let z: u32 = 5;
    let w;
    y *= x+z;
    w = 8;
    y + w
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn hello(v: u32) {
    println!("Hello, world!: v = {}", v);
}

fn is_even(n: u32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

fn pred(v: u32) -> Option<u32> {
    if v == 0 {
        None
    } else {
        Some(v - 1)
    }
}

fn print_prev(v: u32) {
    match pred(v) {
        Some(w) => println!("pred({}) = {}", v, w),
        None => println!("pred({}) is undefined", v),
    }
}

fn even_odd() {
    for n in 0..10 {
        println!("{} is {}", n, if is_even(n) { "even" } else { "odd" });
    }
}

fn even_odd_loop() {
    let mut n = 0;
    loop {
        println!("{} is {}", n, if is_even(n) { "even" } else { "odd" });
        n += 1;
        if n == 10 {
            break;
        }
    }
}

fn mul(x: &mut u64, y: &u64) {
    *x *= *x * *y;
}

fn my_func() {
    let mut x = 10;
    let y = 20;
    println!("x = {}, y = {}", x, y);
    mul(&mut x, &y);
    println!("x = {}, y = {}", x, y);
}

fn app_n(f: fn(u64) -> u64, mut n: u64, mut x: u64) -> u64 {
    loop {
        if n == 0 {
            return x;
        }
        x = f(x);
        n -= 1;
    }
}

fn mul2(x: u64) -> u64 {
    x * 2
}

fn my_func2() {
    println!("app_n(mul2, 4, 3) = {}", app_n(mul2, 4, 3));
}

fn mul_x(x: u64) -> Box::<dyn Fn(u64) -> u64> {
    Box::new(move |y| x * y)
}

fn my_func3() {
    let f = mul_x(3);
    println!("f(5) = {}", f(5));
}

struct Apple {}
struct Gold {}
struct FullStomach {}

fn get_gold(a: Apple) -> Gold {
    Gold {}
}

fn get_full_stomach(a: Apple) -> FullStomach {
    FullStomach {}
}

fn my_func4() {
    let a = Apple {};
    let g = get_gold(a);
    // let s = get_full_stomach(a);
}

struct Foo {
    val: u32
}

fn lifetime_add<'a>(x: &'a Foo, y: &'a Foo) -> u32 {
    x.val + y.val
}

fn my_func5() {
    let x = Foo { val: 10 };
    {
        let y = Foo { val: 20 };
        let z = lifetime_add(&x, &y);
        println!("z = {}", z);
    }
}

fn add_val(x: Foo, y: Foo) -> (u32, Foo, Foo) {
    (x.val + y.val, x, y)
}

fn mul_val(x: Foo, y: Foo) -> (u32, Foo, Foo) {
    (x.val * y.val, x, y)
}

fn my_func6() {
    let x = Foo { val: 10 };
    let y = Foo { val: 20 };
    let (a, xn, yn) = add_val(x, y);
    let (b, _, _) = mul_val(xn, yn);
    println!("a = {}, b = {}", a, b);
}

fn my_func7() {
    let mut x = Foo { val: 10 };
    {
        let a = &mut x;
        println!("a.val = {}", a.val);
        // println!("x.val = {}", x.val);

        let b: &Foo = a;
        // a.val = 20;
        println!("b.val = {}", b.val);

        a.val = 30;
    }

    {
        let c = &x;
        println!("c.val = {}", c.val);
        println!("x.val = {}", x.val);

        // let d = &mut x;
        // d.val = 40;

        println!("c.val = {}", c.val);
    }

    println!("x.val = {}", x.val);
}

#[derive(Copy, Clone)]
struct Vec2 {
    x: f64,
    y: f64
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

fn my_func8() {
    let mut v = Vec2::new(10.0, 5.0);
    println!("v.norm = {}", v.norm());
    v.set(3.8, 9.1);
    println!("v.norm = {}", v.norm());
}

use std::ops::Add;

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

fn my_func9() {
    let v1 = Vec2::new(10.0, 5.0);
    let v2 = Vec2::new(3.7, 8.7);
    let v3 = v1 + v2;
    println!("v3.x = {}, v3.y = {}", v3.x, v3.y);
    println!("v2.x = {}", v2.x);
}

fn add_3times<T>(a: T) -> T
    where T: Add<Output = T> + Copy
{
    a + a + a
}

use std::thread::spawn;

fn hello_world() {
    println!("Hello, world!");
}

fn my_func10() {
    spawn(hello_world).join();

    let h = || println!("Hello, world!");
    spawn(h).join();
}

fn my_func11() {
    let v = 10;
    let f = move || v*2;

    let result = spawn(f).join();
    println!("result = {:?}", result);

    match spawn(|| panic!("I'm panicked!")).join() {
        Ok(v) => println!("successed"),
        Err(e) => {
            let s = e.downcast_ref::<&str>();
            println!("e = {:?}", s)
        }
    }
}

fn main() {
    // println!("Hello, world!");
    // println!("{}", let_example());
    // let n = add(10, 5);
    // hello(n);
    // println!("{}", is_even(10));
    // print_prev(0);
    // even_odd();
    // even_odd_loop();
    // my_func();
    // my_func2();
    // my_func3();
    // my_func4();
    // my_func5();
    // my_func6();
    // my_func7();
    // my_func8();
    // my_func9();
    // println!("{}", add_3times(10));
    // my_func10();
    my_func11();
}
