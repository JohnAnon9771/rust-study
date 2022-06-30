pub fn print(text: &str) {
    println!("{}", text);
}

pub fn shadowing() {
    let x = 5;
    let x = x + 1;
    let test: i32;

    // scopes
    {
        // intern shadowing
        let x = x * 2;
        test = x;
        println!("inner: {}", x);
    }

    println!("outer: {}", x);
    println!("test: {}", test);
}
