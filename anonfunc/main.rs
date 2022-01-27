// Vec of functions in Rust

fn hello(s: &str) -> String {
    return format!("Hello, {}", s);
}

fn goodbye(s: &str) -> String {
    return format!("Goodbye, {}", s);
}

fn sample1() {
    // .push() into Vec
    // Needs boxing

    let mut v: Vec<Box<dyn Fn(&str) -> String>> = Vec::new();

    v.push(Box::new(|s| hello(s)));
    v.push(Box::new(goodbye));

    for f in v {
        let message = f("World");
        println!("{}", message);
    }
}

fn sample2a() {
    // .collect() on array iter
    // Needs Boxing and typed array (including array size)

    let fns: [Box<dyn Fn(&str) -> String>; 2] = [
        Box::new(|s| hello(s)),
        Box::new(goodbye),
    ];
    let v: Vec<&Box<dyn Fn(&str) -> String>> = fns.iter().collect();

    for f in v {
        let message = f("World");
        println!("{}", message);
    }
}

fn sample2b() {
    // Can avoid typing the array itself
    // But means even uglier casting on first array element

    let fns = [
        Box::new(|s: &str| hello(s)) as Box<dyn for<'r> Fn(&'r str) -> String>,
        Box::new(goodbye),
    ];
    let v: Vec<&Box<dyn Fn(&str) -> String>> = fns.iter().collect();

    for f in v {
        let message = f("World");
        println!("{}", message);
    }
}

fn sample2c() {
    // Just lambdas
    // Doesn't help with needing to cast...

    let fns = [
        Box::new(|s: &str| hello(s)) as Box<dyn for<'r> Fn(&'r str) -> String>,
        Box::new(|s| goodbye(s)),
    ];
    let v: Vec<&Box<dyn Fn(&str) -> String>> = fns.iter().collect();

    for f in v {
        let message = f("World");
        println!("{}", message);
    }
}

fn sample2d() {
    // Just function pointers
    // Functions must be defined separately, but cleaner than above

    let fns = [
        hello,
        goodbye,
    ];
    let v: Vec<&fn(&str) -> String> = fns.iter().collect();

    for f in v {
        let message = f("World");
        println!("{}", message);
    }
}

fn sample3() {
    // Using vec![] macro works fine

    let v: Vec<fn(&str) -> String> = vec![
        hello,
        goodbye,
    ];

    for f in v {
        let message = f("World");
        println!("{}", message);
    }
}

fn main() {
    // and maybe you don't need the Vec...

    let fns = [
        sample1,
        sample2a,
        sample2b,
        sample2c,
        sample2d,
        sample3,
    ];

    for f in fns {
        f();
    }
}

