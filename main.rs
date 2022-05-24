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

fn main() {
    println!("Hello, world!");
    println!("{}", let_example());
    let n = add(10, 5);
    hello(n);
    println!("{}", is_even(10));
    print_prev(0);
    even_odd();
    even_odd_loop();
    my_func();
}
