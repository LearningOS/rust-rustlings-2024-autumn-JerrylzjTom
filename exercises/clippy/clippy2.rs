// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res: i32 = 42;
    let option = Some(12);
    while let Some(x) = option {
        res = res.checked_add(x).unwrap_or_else(|| {
            println!("Overflow occurred!");
            i32::MAX // or choose an appropriate fallback value
        });
    }
    println!("{}", res);
}
